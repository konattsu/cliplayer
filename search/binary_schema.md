# Search Binary Format Memo

`index-core` の論理スキーマは保持し、保存形式は `index-core::binary` に分離する。
物理フォーマットは `magic + fixed header + section table + section payloads` の sectioned binary format を採用する。

この形式を選ぶ理由は次の通り。

- `serde + 汎用バイナリ codec` より部分読み込みしやすい
- `HashMap` などの in-memory 向け表現を永続化しなくてよい
- `engine` が WASM でも、必要な section だけ読める
- 将来の section 追加や encoding 変更に耐えやすい

## Scope

このメモは `format_version = 1` の保存形式の方針をまとめる。
`v1` は read-optimized storage を目的にし、汎用 serialization にはしない。

`v1` の前提制約:

- little-endian 固定
- `record_count`, 各種 `id`, 各種 `offset` は `u32`
- `u32` の上限を超える build は builder error
- reader は `format_version = 1` 以外を拒否してよい

## File Layout

ファイル全体は次の順序を基本とする。

1. file header
2. section table
3. metadata section
4. dictionary sections
5. column sections
6. exact index sections
7. sort index sections
8. 将来拡張用の optional sections

reader はまず header と section table だけを読み、`format_version`、`record_count`、各 section の `offset` と `byte_len` を検証する。
その後は必要な section だけを参照する。

## Header / Section Table

header には最低限次を持たせる。

- `magic`
- `format_version`
- `section_count`
- `record_count`
- `section_table_offset`
- `required_features`
- `optional_features`

`required_features` は reader が理解必須の feature 集合、`optional_features` は未知でも無視できる feature 集合とする。
reader は未知の `required_features` を検出したら reject する。

section table には section ごとに次を持たせる。

- `section_id`
- `offset`
- `byte_len`
- `item_count`
- `physical_encoding`

`physical_encoding` は section の論理 schema version とは分離し、payload の物理表現だけを表す。
`v1` で許可する値は原則 `raw_le` のみとし、将来 `bitpacked`, `delta_u32_varint`, `zstd_raw` などを追加できる設計にする。

## Section Id Policy

`section_id` は名前空間を分けて管理する。

- `0x0001`: metadata
- `0x1000` 台: dictionaries
- `0x2000` 台: scalar / list columns
- `0x3000` 台: exact indexes
- `0x4000` 台: sort indexes
- `0x8000` 以上: experimental / private

reader 側の規則:

- 同一 `section_id` の重複は禁止
- 未知 section は `required_features` に関係しない限り無視してよい
- 必須 section 欠落は load error

## Required Sections for v1

`format_version = 1` で必須とする section は次の通り。

- metadata
- dictionary: clips
- dictionary: videos
- dictionary: channels
- dictionary: artists
- dictionary: tags
- column: clip_ids
- column: video_ids
- column: published_ats
- column: channel_ids
- column: is_unlisteds
- column: embeddables
- column: artist_id_lists
- column: tag_id_lists
- exact index: artist_docs
- exact index: tag_docs
- exact index: channel_docs
- exact index: is_unlisted_docs
- exact index: embeddable_docs
- sort index: published_at

`v1` では optional section がなくても検索可能な最小集合を必須扱いにする。
将来 section が増えても、`v1` reader は未知 optional section を無視できるようにする。

## Validation Rules

reader は少なくとも次を検証する。

- `magic` が正しい
- `format_version` が対応範囲内
- `section_count` が異常に大きすぎない
- `offset + byte_len` が overflow しない
- 各 section が file length 内に収まる
- 各 section の `offset` が 8 byte aligned
- section 同士が overlap しない
- `section_table` 自体が file 内に収まる
- 必須 section がすべて存在する
- `section_id` の重複がない
- 未知 `required_features` がない

reader は壊れた binary による巨大 allocation を避ける。
`item_count` や `byte_len` から allocation サイズを決める場合は、上限検証を先に行う。

## Section Design

### Metadata

metadata section には次を入れる。

- `dataset_build_id`
- `builder_version`
- 必要なら追加 build metadata

`dataset_build_id` は min JSON 側の `datasetBuildId` と同じ build identity を表す。
frontend はこの値を cursor や永続化済み queue の再利用判定に使う。

`record_count` は header を canonical とする。
metadata に重複保持する場合は reader が一致を必須検証する。
初版は不整合余地を減らすため、`record_count` は header のみを正とする方がよい。

### Dictionaries

辞書は `HashMap` を保存せず、辞書ごとに `string pool + offsets` で持つ。

- `count: u32`
- `offsets: [u32; count + 1]`
- `utf8_bytes: [u8; total_len]`

`id -> string` は `offsets[id]..offsets[id + 1]` で O(1) 参照できる。
`string -> id` は初版では file に持たず、reader が必要に応じて補助 index を構築する。

辞書 section の不変条件:

- `offsets.len() == count + 1`
- `offsets[0] == 0`
- `offsets` は単調増加
- `offsets[count] == utf8_bytes.len()`
- すべての文字列は valid UTF-8
- 空文字列は禁止
- 同一辞書内の duplicate string は禁止

文字列比較規則:

- 辞書文字列は UTF-8 byte-wise identity で比較する
- Unicode 正規化や大文字小文字の吸収は file format の責務にしない
- build 側が必要なら正規化後の文字列を辞書に入れる

### Scalar Columns

`clip_ids`, `video_ids`, `published_ats`, `channel_ids`, `is_unlisteds`, `embeddables` は column として保持する。

- `u32` / `i64` は固定長配列
- `bool` は初版では `u8` 配列

不変条件:

- `clip_ids.len() == record_count`
- `video_ids.len() == record_count`
- `published_ats.len() == record_count`
- `channel_ids.len() == record_count`
- `is_unlisteds.len() == record_count`
- `embeddables.len() == record_count`
- bool column の各値は `0` または `1`

`bool` を bit-packed にするとサイズは減るが、初版は実装単純性と検証容易性を優先する。

### Variable-Length Columns

`artist_id_lists`, `tag_id_lists` は `offsets + values` で保持する。

- `offsets: [u32; record_count + 1]`
- `values: [u32; total_value_count]`

不変条件:

- `offsets[0] == 0`
- `offsets` は単調増加
- `offsets[record_count] == values.len()`
- 各 list 内の id は昇順
- 各 list 内の id 重複は禁止

これは現在の `U32ListColumn` と同じ方向で、`doc_id -> &[u32]` を効率よく引ける。

### Exact Indexes

`artist_docs`, `tag_docs`, `channel_docs` は dense postings table に落とす。

- `offsets: [u32; term_count + 1]`
- `doc_ids: [u32; total_postings]`

`is_unlisted_docs`, `embeddable_docs` は初版では false/true の 2 本を保持する。
NOT や完全一致 filter の実装を単純にすることを優先する。

exact index の不変条件:

- `offsets[0] == 0`
- `offsets` は単調増加
- `offsets[term_count] == doc_ids.len()`
- 各 posting list は `doc_id` 昇順
- 各 posting list の重複は禁止
- すべての `doc_id < record_count`

bool exact index の追加条件:

- false_docs と true_docs はどちらも昇順
- false_docs と true_docs の積集合は空
- false_docs と true_docs の和集合は `0..record_count` 全体

### Sort Indexes

初版の sort index は `published_at` のみとし、`doc_ids_asc: [u32; record_count]` を持つ。

不変条件:

- `doc_ids_asc.len() == record_count`
- `doc_ids_asc` は permutation of `0..record_count`
- 並び順は `published_at asc, doc_id asc`

この tie-break を仕様に含める。
これにより build ごとの順序を安定化できる。

注意点:

- 降順を単純 reverse で実現すると順序は `published_at desc, doc_id desc` になる
- もし将来 `published_at desc, doc_id asc` を要求するなら、別の sort 表現が必要

`v1` では降順は reverse でよい前提にする。

## Alignment / Padding

物理フォーマットは Rust の struct layout に依存させず、各 field を明示的に little-endian で書く。
compiler 依存の alignment や padding は file 仕様に持ち込まない。

section 境界の方針:

- section payload の開始 offset は 8 byte 境界に揃える
- section 内の配列は packed に連続配置する
- section 間の空きは zero padding とする

8 byte 境界に揃える理由:

- `u64` / `i64` を含む section の reader 実装が単純になる
- 将来の拡張でも十分扱いやすい
- padding 量が小さい

padding は section 境界だけで管理し、section 内の意味論には含めない。
reader は `offset` と `byte_len` を信用し、padding byte の値には依存しない。

## Reader-Side HashMap Memory Policy

reader 側で `string -> id` 解決が必要な場合、辞書 section から補助 `HashMap` を構築できる。
ただしこれは file format の必須要素ではなく、engine の運用方針として扱う。

初版の方針:

- file には `id -> string` だけを保存する
- `string -> id` は reader が lazy に構築する
- 構築対象は必要な辞書だけに限定する

これにより、未使用の辞書については `HashMap` 分のメモリを消費しない。
一方で query 頻度が高い辞書は、一度 map 化すれば以後の解決コストを下げられる。

メモリ面の注意点:

- `HashMap<String, u32>` は string pool と文字列を二重保持しやすい
- borrowed key を使えるならそちらの方が有利
- ただし初版は実装単純性を優先し、通常の `HashMap` を許容してよい

将来メモリ削減が必要なら次を検討する。

- `HashMap<&str, u32>` 相当の borrowed index
- `sorted (str_offset, str_len, id)` を file に持ち、binary search で解決する方式
- hot な辞書だけ map 化し、他は binary search にとどめる方式

## Summary

初版の binary format は、汎用 serialization ではなく read-optimized storage として設計する。
中核の判断は次の通り。

- sectioned binary format を採用する
- `v1` の必須 section 集合と reject 条件を固定する
- dense id を配列に落とし、`HashMap` を保存しない
- alignment/padding は section 境界だけで明示管理する
- reader 側の `HashMap` 構築は optional かつ lazy にする
