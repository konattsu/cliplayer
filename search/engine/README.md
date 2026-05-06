# search/engine design

この README は `search/engine` の実装設計だけを扱う。
全体像は [`../README.md`](../README.md)、binary schema は
[`../binary_schema.md`](../binary_schema.md) を参照。

## 1. 役割

`engine` の責務は次の 1 本である。

- build 済み binary index を読み、query を `doc_id` 集合として評価し、
  sort / paging / total を適用して `clip_uuid` を返す

初版の対象:

- `artist`, `tag`, `channel` の exact filter
- `is_unlisted`, `embeddable` の bool filter
- `published_at` range
- `published_at` の `asc` / `desc`
- cursor paging
- `total_mode = none | exact`

初版の非対象:

- 全文検索
- スコアリング
- facet 集計
- 複数 sort key

## 2. 初版の重要な設計判断

### 2.1 binary reader 主役

engine は `schema::SearchIndex` 全体を materialize しない。
主役は [`index_core::binary::SearchIndexReader`](../index-core/src/binary/reader.rs)
と各 view であり、必要な section だけ読む。

### 2.2 `doc_id` 中心

評価、paging、total は最後まで `doc_id` ベースで扱う。
返却直前にだけ `clip_uuid` へ戻す。

### 2.3 cursor は `doc_id` 単体ではなく seek cursor にする

`{ doc_id }` だけでは query / sort / index build の取り違えを検証できない。
そのため cursor は少なくとも次を持つ構造体にする。

```text
index_build_id
query_fingerprint
sort_field
sort_order
last_published_at
last_doc_id
```

意味:

- `index_build_id`
  - 同じ build に対する cursor かを検証する
- `query_fingerprint`
  - 同じ query に対する cursor かを検証する
- `sort_field`, `sort_order`
  - 同じ sort に対する cursor かを検証する
- `last_published_at`, `last_doc_id`
  - 再開位置を表す seek key

cursor の build をまたいだ安定性は要求しない。
`index_build_id` は安定化のためではなく、誤用検出のために入れる。

### 2.4 tie-break は明示仕様にする

`published_at` が同値になることは稀だが、paging の安定性は tie-break に依存する。
したがって順序は明示的に次で固定する。

- `Asc`: `(published_at asc, doc_id asc)`
- `Desc`: `(published_at desc, doc_id desc)`

ただし物理 index は `published_at asc, doc_id asc` の 1 本だけを持つ。
`Desc` はその配列を逆走査して実現する。

この設計を採る理由:

- tie 頻度が低いため、desc 専用配列を持つ利益が小さい
- asc 1 本で十分に安定順序を定義できる
- 追加メモリなしで `Desc` を実現できる

### 2.5 初版は常に `DocSet` を構築する

`total_mode = none` でも、初版では
`ResolvedQueryNode -> DocSet -> sort scan`
の順を崩さない。

これは最速ではないが、責務境界が明確で実装が壊れにくい。
将来、iterator / predicate evaluator による early-stop 最適化を検討する。

## 3. 公開 API の目標形

実装の主入口は次を想定する。

```rust
pub struct SearchEngine {
    index: LoadedIndex,
}

impl SearchEngine {
    pub fn load(bytes: std::sync::Arc<[u8]>) -> Result<Self, EngineError>;
    pub fn search(
        &self,
        request: &crate::api::query::input::SearchRequest,
    ) -> Result<crate::api::response::SearchResponse, EngineError>;
}
```

`search` は immutable に動く。

### response の追加候補

未知 ID を黙殺すると診断性が悪いので、response には次を追加する余地を持つ。

```rust
warnings: Vec<QueryWarning>
```

初版候補:

- `UnknownArtistId`
- `UnknownTagId`
- `UnknownChannelId`

## 4. `LoadedIndex` 設計

`SearchIndexReader<'a>` は bytes を borrow するため、
owned bytes と borrowed reader を同じ struct に恒久保持しない。

```rust
pub struct LoadedIndex {
    bytes: std::sync::Arc<[u8]>,
    section_directory: SectionDirectory,
    record_count: u32,
    index_build_id: IndexBuildId,
    dictionaries: DictionaryCaches,
}
```

### 4.1 request 時に欲しい性質

request 実行時の reader / view 取得は cheap であるべきで、
header / section table の完全検証を毎回繰り返さない。

採用方針:

- load 時に magic / version / section table / required sections を検証する
- load 時に owned な `SectionDirectory` を構築する
- request 時はその directory から view を引く

`SearchIndexReader::new(&bytes)` を毎回呼ぶだけでは
section table の parse / validation を毎回繰り返す可能性があるため、
初版実装前に `index-core` 側の補助 API を足す前提で進める。

### 4.2 `DictionaryCaches`

逆引き map は load 時に構築する。

- `HashMap<Arc<str>, ChannelId>`
- `HashMap<Arc<str>, ArtistId>`
- `HashMap<Arc<str>, TagId>`

`String` より `Arc<str>` を優先する。
文字列複製コストを抑え、WASM でも load-time memory を節約しやすい。

`clip` / `video` の reverse map は初版必須ではない。

## 5. Request Validation と Complexity Limit

### 5.1 request validation

- `sort.len() == 1`
- `sort[0].field == PublishedAt`
- `limit > 0`
- `limit <= MAX_LIMIT`
- cursor の `index_build_id`, `query_fingerprint`, `sort_field`, `sort_order` が一致
- cursor の seek key が sort index 上に存在する

`sort = []` や複数 sort は validation error にする。

### 5.2 complexity limit

frontend バグや悪意ある入力で CPU / memory が跳ねないよう、少なくとも次を持つ。

- `MAX_QUERY_DEPTH`
- `MAX_BOOLEAN_NODES`
- `MAX_TERMS`
- `MAX_ANY_IN_VALUES`

### 5.3 normalization 規則

- `And(And(...))`, `Or(Or(...))` を flatten
- `Not(Not(x)) -> x`
- `Not(And(xs)) -> Or(Not(x)...)`
- `Not(Or(xs)) -> And(Not(x)...)`
- `Not` は term 直上にだけ残す
- `any_in.values` は sort + dedup
- `any_in.values` の空は reject

`And([])` と `Or([])` は受理しない。
これは `All` / `Empty` への暗黙変換でバグを隠さないためである。

lower / upper が矛盾する `published_at` range は
query error ではなく `Empty` へ正規化してよい。

## 6. 辞書解決

正規化済み query を `ResolvedQueryNode` に変換してから評価する。

### 解決対象

- `artist/tag/channel any_in`
  - string id を内部 ID へ変換
- `is_unlisted`, `embeddable`, `published_at`
  - そのまま使う

### 未知 ID の扱い

未知 string id は error にしない。

- 解決できた値だけ残す
- 1 件も解決できなければ、その term は `Empty`
- 同時に `QueryWarning` を積む

例:

- `artist any_in ["unknown"]` -> `Empty` + warning
- `artist any_in ["known", "unknown"]` -> `artist any_in ["known"]` + warning
- `not(artist any_in ["unknown"])` -> `All` + warning

論理的には `Not(Empty) = All` だが、
warning を返さない silent success にはしない。

## 7. `DocSet` 設計

`DocSet` は外部 API ではなく engine 内部表現である。
したがって `src/api/query/doc_set.rs` ではなく、
`src/doc_set.rs` または `src/eval/doc_set.rs` に置く。

```rust
pub enum DocSet {
    All,
    Empty,
    SortedDocIds(SortedDocIds),
    BitSet(Vec<u64>),
}
```

### 7.1 `SortedDocIds` の不変条件

`SortedDocIds` は次を必須とする。

- `doc_id asc`
- 重複なし
- すべての `doc_id < record_count`
- 空集合は `DocSet::Empty` で表し、空 `SortedDocIds` は作らない

`Vec<DocId>` を生で持たず、専用型か constructor で不変条件を集中管理する。

### 7.2 `BitSet` の不変条件

`BitSet` の長さは常に次で固定する。

```text
word_len = (record_count + 63) / 64
```

追加規則:

- `record_count` 外の末尾 bit は常に 0
- `Not`, `difference`, `count` は末尾 mask を考慮する

### 7.3 helper

少なくとも次を持たせる。

- `is_empty()`
- `contains(doc_id)`
- `count(record_count)`
- `to_bitset(record_count)`
- `intersect(lhs, rhs, record_count)`
- `union(lhs, rhs, record_count)`
- `difference(lhs, rhs, record_count)`

## 8. Term 評価

### 8.1 exact term

対象:

- `ArtistAnyIn`
- `TagAnyIn`
- `ChannelAnyIn`
- `IsUnlistedEq`
- `EmbeddableEq`

評価:

1. 対応 postings view を読む
2. `any_in` は posting list の union を取る
3. bool term は `true_docs` / `false_docs` を取る

返り値は原則 `SortedDocIds`。

### 8.2 `published_at` range

range 評価は `published_at_sort()` と `published_ats()` を使う。

1. sort index 上で lower / upper の位置を二分探索する
2. ヒット件数を求める
3. 表現を選ぶ

表現選択:

- `hit_count * size_of::<u32>() <= bitset_word_len * size_of::<u64>()`
  - `SortedDocIds`
- それ以外
  - `BitSet`

`SortedDocIds` を選ぶ場合は、ヒットした `doc_id` を collect して
`doc_id asc` に sort してから保持する。

この閾値により、小さい range で毎回 bitset 全体を確保するのを避ける。

## 9. 論理式評価

query は NNF 後の `ResolvedQueryNode` を再帰的に評価する。

### `And`

- `Empty` が出たら即終了
- `All` は単位元として無視
- 小さい posting 同士は merge intersection
- 必要になった時点で `BitSet` へ昇格

### `Or`

- `All` が出たら `All`
- `Empty` は単位元として無視
- union が膨らむなら `BitSet`

### `Not`

NNF 後は term にしかかからない前提で、
`difference(All, term)` を取る。

追加規則:

- `Not(All) = Empty`
- `Not(Empty) = All`

## 10. Sort / Paging / Total

### 10.1 sort

- `Asc`
  - `(published_at asc, doc_id asc)`
- `Desc`
  - `(published_at desc, doc_id desc)`

実装上は asc sort index 1 本だけを保持し、
`Desc` は逆走査する。

### 10.2 cursor seek

cursor の seek key は `(last_published_at, last_doc_id)` とする。

再開方法:

- `Asc`
  - asc sort index 上で seek key より大きい最初の位置から開始
- `Desc`
  - asc sort index 上で seek key より小さい直前の位置から開始

`published_at` tie が稀でも、seek key を `published_at` 単体にしない。
`doc_id` を含めて完全順序を作ることで paging の重複・欠落を防ぐ。

### 10.3 page 生成

sort scan では `limit + 1` 件まで拾う。

- `<= limit`
  - `has_more = false`
- `limit + 1`
  - 末尾 1 件を落とす
  - `has_more = true`
  - 返却末尾の `(published_at, doc_id)` を `next_cursor` に入れる

### 10.4 total

- `None`
  - page 生成後は早期停止してよい
  - ただし初版では DocSet 自体は先に構築済み
- `Exact`
  - `DocSet` 自体から件数を数える

件数計算:

- `All` -> `record_count`
- `Empty` -> `0`
- `SortedDocIds` -> `len()`
- `BitSet` -> `count_ones` 合計

## 11. エラー分類

最低限次へ分ける。

- `CorruptIndex`
- `VersionMismatch`
- `UnsupportedFeature`
- `InvalidRequest`
- `InvalidCursor`
- `QueryTooComplex`
- `InternalIndex`

未知 string id は error にせず warning に流す。

## 12. 推奨 module 分割

```text
src/
  api/
  error.rs
  index.rs
  normalize.rs
  resolve.rs
  eval.rs
  doc_set.rs
  paging.rs
  engine.rs
  lib.rs
```

## 13. テスト観点

unit test:

- query normalization
- complexity limit
- unknown id warning
- `SortedDocIds` の不変条件
- `BitSet` の末尾 mask
- range 二分探索
- small range / large range の表現切替
- asc / desc paging
- `next_cursor`, `has_more`
- cursor mismatch 検出
- `total_mode = none | exact`

integration test:

- 小さな `schema::SearchIndex` を組んで binary 化し、engine に読ませる
- `artist`, `tag`, `channel`, bool filter
- `and`, `or`, `not`
- `published_at` range
- `published_at desc` tie
- cursor paging をまたげること
- 古い build の cursor を reject すること

## 14. 実装順序

1. `index-core` 側の load-time validation / section directory 補助
2. `index-builder` 側の `index_build_id` 出力
3. `engine` の cursor / warning API 更新
4. `LoadedIndex`, reverse map, section directory
5. `DocSet` と helper
6. validation / normalization / complexity limit
7. 辞書解決
8. term 評価
9. boolean query 評価
10. paging / total
11. integration test
