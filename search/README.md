# 検索エンジン / インデックス

このディレクトリでは、検索用インデックスの論理 schema と、そのインデックスを前提にした検索 API 型を定義する。
入力データ全体の位置づけは [`SPECIFICATION.md`](../SPECIFICATION.md) を参照。

## 1. 目的

この検索基盤の初版は、clip を単位に次のことを安定して行うことを目的とする。

- ライバーで絞り込む
- タグで絞り込む
- 投稿チャンネルで絞り込む
- `is_unlisted`, `embeddable` で絞り込む
- `published_at` の範囲で絞り込む
- `published_at` の `asc` / `desc` ソートを行う
- cursor ベースでページングする
- URL に載せるユーザー指定クエリと、UI 側の必須フィルタを frontend 側で合成できる

初版では全文検索、スコアリング、facet 集計、複合ソートは扱わない。

## 2. 責務の分離

### `search/index-core`

- 検索インデックスの schema を定義する
- engine が読む共通型を提供する
- 現在の Rust 実装では主に `index-core/src/schema/**` に置く

### `search/index-builder`

- source-of-truth から検索用インデックスを構築する
- 文字列 ID を内部整数 ID に正規化する
- postings / columns / sort index を構築する
- 現在の Rust 実装では主に `index-builder/src/build/**` に置く

### 検索エンジン側

- 構築済みインデックスを読み込む
- クエリ AST を評価する
- sort / paging / total を適用する
- `clip_uuids` を返す
- 現在の Rust 実装では API 型を主に `engine/src/api/**` に置く

### frontend 側

- URL と検索フォームの相互変換を行う
- `required_filter` と `user_query` を `and(...)` で合成して request を組み立てる
- 検索結果の `clip_uuid` から表示用データを引く

## 3. データモデル

検索単位は **clip** で、1 record = 1 `clip_uuid` とする。

### ID の役割

- `doc_id`
  - 検索エンジン内部でだけ使う dense な連番 `u32`
  - postings, columns, bitset, sort index の共通キー
- entity id
  - `clip_uuid`, `video_id`, `channel_id`, `artist_id`, `tag_id` などの業務 ID
  - source-of-truth では文字列、index 内では整数 ID に正規化する

`doc_id` は build ごとに振り直してよい。外部 API や URL には露出しない。

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

補足:

- `clip_id` と `doc_id` は別物
- `artist_ids`, `tag_ids` は集合として扱う
- `channel_id`, `is_unlisted`, `embeddable`, `published_at` は単一値として扱う

### 正規化ルール

- 文字列 ID は `clip`, `video`, `channel`, `artist`, `tag` ごとに独立した辞書へ入れる
- `published_at` は `TimestampSecs` として UTC unix time seconds に正規化する
- `is_unlisted`, `embeddable` は `true` / `false` のみを取る
- `artist_ids`, `tag_ids` は build 時に sort + dedup して保持する

## 4. インデックスの論理構造

`SearchIndex` は次の主要要素で構成される。

実装上は `index-core/src/schema/search_index.rs` を起点に `schema/**` へ分割している。

### `Dictionaries`

文字列 ID と内部整数 ID の相互変換表。

- `clip_uuid <-> clip_id`
- `video_id <-> video_id`
- `channel_id <-> channel_id`
- `artist_id <-> artist_id`
- `tag_id <-> tag_id`

役割:

- クエリ入力の文字列 ID を内部 ID に解決する
- 検索結果の `clip_id` を `clip_uuid` に戻す

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

役割:

- ソートや cursor の評価に使う
- range 条件の境界値評価に使う
- 最終的な `doc_id -> clip_id -> clip_uuid` 解決に使う

### `ExactIndexes`

完全一致フィルタ用の inverted index。

- `artist_id -> sorted doc_id[]`
- `tag_id -> sorted doc_id[]`
- `channel_id -> sorted doc_id[]`
- `is_unlisted(false/true) -> sorted doc_id[]`
- `embeddable(false/true) -> sorted doc_id[]`

要件:

- posting list は `doc_id` 昇順
- posting list は重複なし
- `doc_id` が dense なので、必要に応じて bitset に変換できる

### `SortIndexes`

sort 用の順序 index 群。
現状は `published_at` 用の 1 本だけを持つ。

- `published_at`
  - `(published_at asc, doc_id asc)` の順で並んだ `doc_id[]`

要件:

- 同一 `published_at` の順序は `doc_id` を tie-breaker にして安定化する
- `desc` は専用列を持たず、`asc` 配列を逆順に走査して実現する

## 5. build 時の検証

index builder は最低限次を検証してから build を進める。

- clip が未知の `channel_id` を参照していないこと
- clip が未知の `artist_id` を参照していないこと
- clip が未知の `tag_id` を参照していないこと
- 辞書に入れた文字列 ID が正しく内部 ID に解決できること

現状の build 処理では、各 clip について次を行う。

1. `clip_uuid` 順に clip を安定化する
2. 各 ID 用辞書を構築する
3. clip を `NormalizedClipRecord` に正規化する
4. `ColumnStore` を構築する
5. `ExactIndexes` を構築する
6. `SortIndexes` を構築する

## 6. クエリモデル

検索クエリは UI 依存の断片的なパラメータではなく、AST として表現する。

### 全体構造

```json
{
  "query": {
    "type": "and",
    "children": [
      { "type": "term", "field": "is_unlisted", "op": "eq", "value": false },
      { "type": "term", "field": "artist_id", "op": "any_in", "values": ["suisei"] },
      { "type": "term", "field": "tag_id", "op": "any_in", "values": ["original-song"] }
    ]
  },
  "sort": [
    { "field": "published_at", "order": "desc" }
  ],
  "page": {
    "limit": 50,
    "cursor": null
  },
  "total_mode": "none"
}
```

意味:

- `query`
  - 検索エンジンがそのまま評価する AST
  - `required_filter` と `user_query` は frontend 側で `and(...)` に合成してからここへ入れる
- `sort`
  - 現状は `published_at` のみ
- `page`
  - `limit` と構造化 cursor
- `total_mode`
  - `exact` か `none`

### query node

```ts
type QueryNode =
  | { type: "and"; children: QueryNode[] }
  | { type: "or"; children: QueryNode[] }
  | { type: "not"; child: QueryNode }
  | TermNode;

type TermNode =
  | { type: "term"; field: "artist_id"; op: "any_in"; values: string[] }
  | { type: "term"; field: "tag_id"; op: "any_in"; values: string[] }
  | { type: "term"; field: "channel_id"; op: "any_in"; values: string[] }
  | { type: "term"; field: "is_unlisted"; op: "eq"; value: boolean }
  | { type: "term"; field: "embeddable"; op: "eq"; value: boolean }
  | { type: "term"; field: "published_at"; op: "range"; gte?: number; gt?: number; lte?: number; lt?: number };
```

制約:

- `and.children`, `or.children` は 1 要素以上
- `not` は単項のみ
- `any_in` は「指定 values のいずれかに一致」を意味する
- `any_in.values` は empty を禁止する
- `any_in.values` は query 正規化時に sort + dedup する
- `published_at` の境界値は UTC unix time seconds
- `query = null` は match-all 相当として扱ってよい

### query 型の意図

- `artist_id`, `tag_id`
  - record 側が複数値なので `any_in` を使う
- `channel_id`
  - record 側は単一値だが、API 上は複数候補指定を許すため `any_in` を使う
- `is_unlisted`, `embeddable`
  - bool のため `eq`
- `published_at`
  - lower / upper bound を持つ `range`

`all_in` は現状の必須機能ではない。
同一 field に対する複数条件を `and` で束ねれば表現できるため、初版では operator を増やさない。

### 正規化 ルール

query parse 後、評価前に次の正規化を行ってよい。

- `And(And(...))`, `Or(Or(...))` を flatten する
- `Not(Not(x)) -> x` を適用する
- `Not(And(xs)) -> Or(Not(x)...)` を適用する
- `Not(Or(xs)) -> And(Not(x)...)` を適用する
- つまり `Not` は最終的に term 直上にだけ現れる NNF まで落としてよい
- `any_in.values` は empty を reject する
- `any_in.values` は sort + dedup する
- 正規化後の `And` / `Or` の子順は、evaluator が扱いやすい順に並べ替えてよい

## 7. 検索結果モデル

公開 API の返り値は最低限次を持つ。

```ts
type SearchResult = {
  clip_uuids: string[];
  next_cursor: { doc_id: number } | null;
  total_mode: "exact" | "none";
  total?: number;
  has_more: boolean;
};
```

要件:

- `clip_uuids` は sort と paging 適用後の順序を保つ
- `next_cursor` は次ページがなければ `null`
- `has_more` は次ページ有無を表す
- `total` は `total_mode = "exact"` のときだけ返す
- cursor は sort 順序に依存する構造化データとして持つ
- 現状の cursor は `{ doc_id }`

内部では `doc_id` ベースで処理し、最後に `clip_uuid` へ戻す。

## 8. 評価方針

### 完全一致フィルタ

次の条件は postings を使って評価する。

- `artist_id`
- `tag_id`
- `channel_id`
- `is_unlisted`
- `embeddable`

term 評価結果はまず `SortedDocIds` で持ち、必要に応じて bitset に変換する。

### `published_at` range

`published_at` の範囲条件は、`sort_indexes.published_at` を使って評価する。

基本方針:

- sort index 上で開始・終了位置を決める
- 必要なら `published_at` column を参照して境界条件を確認する
- `NOT` を扱えるように、最終的には bitset に落とせる形を前提にする

### sort / paging

- filter で候補集合を作る
- `sort_indexes.published_at` を順方向または逆方向に走査する
- 候補集合に含まれる `doc_id` だけを拾う
- cursor は `{ doc_id }` を元に次の走査開始位置を決める

### total

- `total_mode = "none"`
  - `limit + 1` 件見つかった時点で早期停止してよい
- `total_mode = "exact"`
  - ページ返却に十分な件数が見つかっても、総件数のため候補全体を最後まで評価する

## 9. URL パラメータ方針

- URL には `user_query` を載せる
- `required_filter` は URL に含めない
- request 直前に frontend 側で `and(user_query, required_filter)` を作り、engine には合成後の `query` を渡す
- URL スキーマの詳細は frontend 側の責務とする
- engine の request / response では構造化 `Cursor` を使う
- URL に載せる必要がある場合だけ、frontend 側で `Cursor` を opaque token へ encode / decode する

## 10. エラー方針

### query / runtime

- 未知の field / op / enum 値は query parse error
- 未知の辞書 ID 参照は empty result として扱ってよい
- index の load 失敗や version 不一致は load error

### build

- 入力データの参照整合性が壊れていれば build error にする
- postings や sort index が内部整合性を満たさなければ build error にする

## 11. 初版の非目標

- 自由文全文検索
- スコアリング
- facet 集計
- ハイライト
- あいまい検索
- 複数キー複合ソート
- offset ベースの深いページング

初版は **構造化フィルタ + `published_at` ソート + cursor paging** に責務を絞る。

## 12. 今後の拡張余地

- sort key の追加
- 文字列検索用 index の追加
- facet 集計用 section の追加
- query schema version の導入
- バイナリフォーマットの物理表現詳細の明文化

## 13. 現在の Rust モジュール対応

実装では `model` という総称は使わず、より具体的な名前に分けている。

- `search/index/src/schema/**`
  - `SearchIndex`, `Dictionaries`, `ColumnStore`, `ExactIndexes`, `SortIndexes`
  - `DocId`, `ClipId` などの ID 型
  - `TimestampSecs`
- `search/engine/src/api/query/input.rs`
  - `SearchRequest`, `QueryNode`, `TermNode`, `SortSpec`, `PageSpec`
- `search/engine/src/api/query/resolved.rs`
  - `ResolvedQueryNode`, `ResolvedTermNode`
- `search/engine/src/api/query/types.rs`
  - `SortField`, `SortOrder`, `TotalMode`, `RangeBound`, `DateRange`
- `search/engine/src/api/query/doc_set.rs`
  - `DocSet`
- `search/engine/src/api/pagination.rs`
  - `Cursor`
- `search/engine/src/api/response.rs`
  - `SearchResponse`, `InternalSearchResponse`
