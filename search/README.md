# 検索基盤の概要

このディレクトリは、clip を検索単位とする検索基盤を扱う。
ここで定義するのは次の 3 層である。

- 検索 index の論理 schema
- 構築済み index の物理 binary format
- その index を前提にした検索 API

検索エンジンの実装設計は [`engine/README.md`](./engine/README.md) を参照。
物理 format の詳細は [`binary_schema.md`](./binary_schema.md) を参照。
入力データ全体の位置づけは [`SPECIFICATION.md`](../SPECIFICATION.md) を参照。

## 1. 初版の目的

初版は clip 単位で次を安定して行うことを目的とする。

- ライバーで絞り込む
- タグで絞り込む
- 投稿チャンネルで絞り込む
- `is_unlisted`, `embeddable` で絞り込む
- `published_at` の範囲で絞り込む
- `published_at` の `asc` / `desc` ソートを行う
- cursor ベースでページングする
- frontend 側で `required_filter` と `user_query` を合成できる

初版では全文検索、スコアリング、facet 集計、複合ソートは扱わない。

## 2. 全体の流れ

検索の全体像は次のとおり。

1. source-of-truth から clip 群を読む
2. `index-builder` が文字列 ID を内部整数 ID に正規化する
3. `index-builder` が columns / exact indexes / sort index を構築する
4. `index-core::binary` が build 済み index を binary に保存する
5. `engine` が binary を読み込み、query AST を評価する
6. `engine` が sort / paging / total を適用して `clip_uuid` を返す
7. frontend が `clip_uuid` を使って表示用データを引く

frontend と engine の境界では、断片的な URL パラメータではなく
構造化 query request をやり取りする。

## 3. 責務分離

### `search/index-core`

- 検索 index の論理 schema を定義する
- builder / engine が共有する型を提供する
- 構築済み index の binary reader / writer を提供する
- 物理 format は [`binary_schema.md`](./binary_schema.md) で定義する

### `search/index-builder`

- source-of-truth から検索 index を構築する
- 文字列 ID を内部整数 ID に正規化する
- columns / postings / sort index を構築する
- build 時の整合性検証を行う

### `search/engine`

- 構築済み binary index を読む
- query AST を正規化し、辞書解決し、評価する
- sort / paging / total を適用する
- `clip_uuid` を返す

### frontend

- URL と検索フォームの相互変換を行う
- `required_filter` と `user_query` を `and(...)` で合成する
- 検索結果の `clip_uuid` から表示用データを引く

## 4. データモデル

検索単位は **clip** で、1 record = 1 `clip_uuid` とする。

### ID の役割

- `doc_id`
  - engine 内部だけで使う dense な `u32`
  - postings, columns, bitset, sort index の共通キー
- entity id
  - `clip_uuid`, `video_id`, `channel_id`, `artist_id`, `tag_id` などの業務 ID
  - source-of-truth では文字列、index 内では整数 ID に正規化する

`doc_id` は build ごとに振り直してよい。
外部 API や URL に安定 ID として露出しない。

### 正規化済み record

各 clip は少なくとも次のフィールドを持つ。

| field | type | 説明 |
| --- | --- | --- |
| `doc_id` | `u32` | 検索内部の文書番号 |
| `clip_id` | `u32` | `clip_uuid` を整数化した ID |
| `video_id` | `u32` | `video_id` を整数化した ID |
| `published_at` | `TimestampSecs` | UTC unix time seconds |
| `channel_id` | `u32` | 投稿チャンネル ID |
| `is_unlisted` | `bool` | URL 限定公開か |
| `embeddable` | `bool` | 埋め込み可否 |
| `artist_ids` | `u32[]` | clip に紐づくライバー ID 群 |
| `tag_ids` | `u32[]` | clip に紐づくタグ ID 群 |

正規化ルール:

- 文字列 ID は entity ごとに独立した辞書へ入れる
- `published_at` は UTC unix time seconds に正規化する
- `artist_ids`, `tag_ids` は build 時に sort + dedup して保持する

## 5. 論理 index 構造

`SearchIndex` は次の要素で構成される。
この論理構造を binary に落とす方法は [`binary_schema.md`](./binary_schema.md) を参照。

### `Dictionaries`

文字列 ID と内部整数 ID の対応表。

- `clip_uuid <-> clip_id`
- `video_id <-> video_id`
- `channel_id <-> channel_id`
- `artist_id <-> artist_id`
- `tag_id <-> tag_id`

### `ColumnStore`

`doc_id` から各フィールド値を引くための列ストア。

- `clip_ids`
- `video_ids`
- `published_ats`
- `channel_ids`
- `is_unlisteds`
- `embeddables`
- `artist_id_lists`
- `tag_id_lists`

### `ExactIndexes`

完全一致フィルタ用の inverted index。

- `artist_id -> sorted doc_id[]`
- `tag_id -> sorted doc_id[]`
- `channel_id -> sorted doc_id[]`
- `is_unlisted(false/true) -> sorted doc_id[]`
- `embeddable(false/true) -> sorted doc_id[]`

### `SortIndexes`

sort 用の順序 index 群。
初版は `published_at` だけを持つ。

- `published_at`
  - `(published_at asc, doc_id asc)` の順で並んだ `doc_id[]`

`desc` は専用配列を持たず、`asc` 配列を逆順に走査して
`(published_at desc, doc_id desc)` を実現する。

## 6. build 時の責務

`index-builder` は最低限次を保証してから binary を出力する。

- clip が未知の `channel_id` を参照していないこと
- clip が未知の `artist_id` を参照していないこと
- clip が未知の `tag_id` を参照していないこと
- 辞書と columns / postings / sort index の整合性が取れていること

初版の build 手順は次の流れを前提にする。

1. clip を安定順に並べる
2. 各 entity 用辞書を構築する
3. clip を正規化済み record に変換する
4. `ColumnStore` を構築する
5. `ExactIndexes` を構築する
6. `SortIndexes` を構築する
7. binary へ変換する

## 7. Query / Response 概要

engine に渡す request は、UI 固有のパラメータ列ではなく AST を持つ構造体とする。
Rust 型の詳細は `engine/src/api/**` に置く。

### query

query は `and`, `or`, `not`, term からなる木構造を持つ。
term は次をサポートする。

- `artist_id any_in`
- `tag_id any_in`
- `channel_id any_in`
- `is_unlisted eq`
- `embeddable eq`
- `published_at range`

`query = null` は match-all として扱ってよい。

### sort

初版の sort は `published_at` 1 本のみをサポートする。

### page

- `limit`
- `cursor`

cursor は構造化データとして扱う。
frontend からは opaque token として扱い、engine 側では少なくとも
`index_build_id`, `query_fingerprint`, sort 情報, seek key を持つ。
seek key は `published_at` 単体ではなく `(published_at, doc_id)` を使う。

### response

最低限次を返す。

- `clip_uuids`
- `next_cursor`
- `total_mode`
- `total`
- `has_more`
- `warnings` を追加する余地がある

engine は内部では `doc_id` ベースで評価し、返却直前に `clip_uuid` へ戻す。

## 8. Cursor と URL の方針

cursor についての前提は次のとおり。

- cursor は同一 index build に対してのみ有効
- cursor は同じ query / sort 条件に対してのみ再利用する
- cursor を URL に載せる必要がある場合だけ frontend 側で encode / decode する

build をまたぐ stable cursor は初版の対象外とする。
ただし誤用検出のため、cursor payload には build 情報と query / sort の検証情報を持たせる。

## 9. 現在の Rust 実装の置き場所

- `index-core/src/schema/**`
  - 論理 schema
- `index-core/src/binary/**`
  - binary reader / writer / validation
- `index-builder/src/build/**`
  - build pipeline
- `engine/src/api/**`
  - search request / response / query 型

engine 実装本体の設計は [`engine/README.md`](./engine/README.md) を参照。

## 10. 初版の非目標

- 自由文全文検索
- スコアリング
- facet 集計
- ハイライト
- あいまい検索
- 複数キー複合ソート
- offset ベースの深いページング

初版は **構造化フィルタ + `published_at` ソート + cursor paging** に責務を絞る。

## 11. 今後の拡張余地

- sort key の追加
- 文字列検索用 index の追加
- facet 集計用 section の追加
- query schema version の導入
- cursor payload の拡張
- binary format の physical encoding の追加
