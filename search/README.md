# 検索エンジン/インデックス

目的: 高速に検索できるインデックスと、そのインデックスだけを知っていれば動作する検索エンジンを生成する。

このディレクトリでは、s3 の生成物である検索インデックスと、その利用方法を定義する。
詳細入力元や s3 の位置づけは [`SPECIFICATION.md`](../SPECIFICATION.md) を参照。

## 1. 何をしたいか

- ライバー名で絞り込み
- 動画タグで絞り込み
- 投稿チャンネルで絞り込み
- 投稿日で範囲指定
- `privacyStatus`, `embeddable` で絞り込み
- 投稿日で `asc` / `desc` ソート
- ページング (`offset`, `limit`)
- URL query parameter に載せられる検索クエリ表現
- UI 側の必須フィルタと、ユーザー指定クエリを分離できること

## 2. 設計方針

- 検索インデックスはバイナリで提供する
- 検索エンジンは Rust で実装し、WASM としてフロントから利用できる形を優先する
- インデックスの内部表現は検索エンジン内に閉じ込める
- インデックス内部では、文字列はすべて整数 ID に正規化して扱う
- フロントが直接インデックス構造を読むことは前提にしない
- 検索エンジンの公開 API は、クエリ AST とページング情報を受け取り、順序保証された `clipUuid[]` を返すことを最小要件とする
- 動画 ID は検索結果に含めなくてよい
  - フロントは `clipUuid -> videoId` を O(1) で引ける前提のため
- WAND, MaxScore などのスコアリング系高速化は現時点では非対象
  - 今回は構造化フィルタとソートの基盤を固めることを優先する

## 3. 責務境界

### 3.1 index builder の責務

- s0 の正データを読む
- 検索対象レコードを正規化して確定する
- 転置インデックス、ソート用列、辞書などを構築する
- バージョン付きバイナリを出力する
- 入力ファイル由来の manifest を同梱する

### 3.2 search engine の責務

- バイナリインデックスをロードする
- クエリ AST を評価する
- 必須フィルタとユーザー指定クエリを合成する
- ソート、ページングを適用する
- `clipUuid[]` と総件数を返す

### 3.3 frontend の責務

- URL と検索フォームの相互変換
- `required_filter` の自動付与
- `clipUuid` から詳細表示用データを引く

## 4. 検索対象レコード

検索単位は **clip** とする。
1 レコード = 1 `clipUuid`。

実装上は以下の 3 種類の ID を明確に分離する。

- `doc_id`
  - 検索エンジン内部でのみ使う dense な文書番号
  - posting list, sort column, bitset の基準になる
  - 0..N-1 の連番 `u32` を基本とする
- `entity_id`
  - `clip_uuid`, `video_id`, `channel_id`, `artist_id`, `tag_id` など、source-of-truth に存在する業務上の識別子
  - 入力時は文字列だが、index build 時に整数 ID に変換する
- `string table offset`
  - 必要なら整数 ID から元文字列へ戻すための辞書参照

各レコードは最低限、以下の正規化済みフィールドを持つ。
ここで保持するのは基本的に **文字列そのものではなく整数 ID** とする。

| field | type | 説明 |
| --- | --- | --- |
| `doc_id` | u32 | 内部文書番号 |
| `clip_id` | u32 | `clip_uuid` を整数化した ID |
| `video_id` | u32 | 元動画 ID を整数化した ID |
| `published_at` | i64 | 投稿日時の unix time milliseconds |
| `channel_id` | u32 | 投稿チャンネル ID を整数化した ID |
| `privacy_status` | enum | `public \| unlisted \| private` |
| `embeddable` | bool | 埋め込み可否 |
| `artist_ids` | u32[] | 関連ライバー ID 群を整数化したもの |
| `tag_ids` | u32[] | タグ ID 群を整数化したもの |

補足:

- `clip_id` は `doc_id` と別物である
- `doc_id` は検索実行都合の内部 ID、`clip_id` は clip という業務エンティティの ID
- `artist_ids` は clip 単位で関連付くライバー集合を表す
- `tag_ids` は clip 単位で付与されたタグ集合を表す
- `channel_id` は動画投稿元チャンネルを表す
- 完全一致検索は、原則すべて整数 ID に正規化して評価する

## 5. 正規化ルール

### 5.1 ID 系

- `clip_uuid`, `video_id`, `channel_id`, `artist_id`, `tag_id` は source-of-truth では文字列のまま保持する
- builder はそれぞれに整数 ID を振る
- 検索時の条件指定は外部 API では文字列 ID を基本とし、engine 内で整数 ID に解決する
- `clip_uuid -> clip_id`
- `video_id(string) -> video_id(u32)`
- `channel_id(string) -> channel_id(u32)`
- `artist_id(string) -> artist_id(u32)`
- `tag_id(string) -> tag_id(u32)`
- すべてを 1 つの巨大辞書に混在させる必要はない
  - `clip`, `video`, `channel`, `artist`, `tag` ごとに独立辞書を持つ方が実装と検証が単純

### 5.1.1 `doc_id` の扱い

- `doc_id` は business identifier ではなく、検索 index 内の row 番号である
- `doc_id` は build ごとに振り直されてよい
- `doc_id` を URL や外部 API に露出しない
- posting list と sort column は `doc_id` を共通キーにして参照局所性を優先する
- 将来 segment 化するなら、外部表現としては `DocAddress { segment_ord, local_doc_id }` 相当を意識して設計する
  - ただし初版が単一セグメントなら、物理上は `u32 doc_id` のみで十分

### 5.2 日付

- `published_at` は UTC 基準の unix time milliseconds に正規化する
- ソートと範囲検索はすべてこの値に対して行う
- date-only 指定を UI が許す場合、境界の解釈は UI 側で UTC millisecond に変換してから engine に渡す

### 5.3 enum/bool

- `privacy_status` の取り得る値は `public`, `unlisted`, `private` に固定する
- 未知の値が入力に来た場合、builder は失敗すべき
- `embeddable` は `true` / `false` のみ

## 6. インデックスの論理構造

インデックスは、少なくとも以下の論理セクションを持つ。

### 6.1 header

- magic bytes
- format version
- 生成日時
- セクション数
- 各セクションの offset / length
- manifest への参照

### 6.2 manifest

- 入力ファイル一覧
- 各入力のハッシュ
- builder version
- format version
- record count
- 辞書サイズ
- `max_doc_id + 1`

### 6.3 record table

レコード ID (`doc_id: u32`) から固定長・可変長列へアクセスするための基底テーブル。

最低限保持するもの:

- `doc_id -> clip_id`
- `doc_id -> video_id`
- `doc_id -> published_at`
- `doc_id -> channel_id`
- `doc_id -> privacy_status`
- `doc_id -> embeddable`
- `doc_id -> artist_id[]`
- `doc_id -> tag_id[]`

補足:

- `doc_id -> clip_uuid(string)` を直接持たない
- 外部応答で `clipUuid` が必要なら `doc_id -> clip_id -> clip_uuid` の 2 段解決にする
- 本格的な検索実装に寄せるなら、record table は stored fields / column store 的な責務として扱うのが自然

### 6.4 dictionary tables

- `clip_uuid(string) <-> clip_id(u32)`
- `video_id(string) <-> video_id(u32)`
- `channel_id(string) <-> channel_id(u32)`
- `artist_id(string) <-> artist_id(u32)`
- `tag_id(string) <-> tag_id(u32)`

要件:

- ユーザークエリの文字列 ID を engine 内部の整数 ID に変換できること
- 検索結果として必要なら `clip_id` から `clip_uuid` に逆変換できること
- 変換テーブルは index に含める
- 文字列辞書は UTF-8 の一括 blob + offset table のような形で持つと実装しやすい

### 6.5 postings

転置インデックス。少なくとも以下を持つ。

- `artist_id -> sorted doc_id[]`
- `tag_id -> sorted doc_id[]`
- `channel_id -> sorted doc_id[]`
- `privacy_status -> sorted doc_id[]`
- `embeddable(true/false) -> sorted doc_id[]`

要件:

- posting list は `doc_id` 昇順
- AND / OR / NOT 評価時に線形マージ可能であること
- 圧縮方式は実装時に選んでよいが、format version ごとに固定すること

### 6.6 sort columns

- `published_at asc` 用の `doc_id[]`
- `published_at desc` 用の `doc_id[]`

補足:

- `desc` は `asc` の逆順参照でもよい
- 将来ソートキーが増えるなら、列単位で拡張できる構造にする
- `doc_id` が dense である前提を維持すると、sort column も単純配列でよい

## 7. 推奨バイナリフォーマット

物理フォーマットは以下の性質を満たすこと。

- ランダムアクセス可能
- セクションスキップ可能
- version mismatch を早期検出できる
- WASM 上で余計な JSON parse を避けられる

初版では以下のような単一ファイル構成を推奨する。

1. `Header`
2. `Section Directory`
3. `Manifest`
4. `Dictionaries`
5. `Record Table`
6. `Postings`
7. `Sort Columns`

ファイル名例:

- `public/search/clips-search-index.bin`
- `public/search/clips-search-index.manifest.json`

備考:

- manifest はバイナリ同梱に加えて、人間確認用の JSON を別出力してもよい
- ただし検索エンジンの動作に必要なメタ情報は `.bin` 単体で完結しているべき

## 8. クエリモデル

検索クエリは UI 用の簡易パラメータではなく、**AST を正とする**。

### 8.1 全体構造

```json
{
  "user_query": {
    "type": "and",
    "children": [
      { "type": "term", "field": "artist_id", "op": "in", "values": ["suisei"] },
      { "type": "term", "field": "tag_id", "op": "in", "values": ["original-song"] }
    ]
  },
  "required_filter": {
    "type": "term",
    "field": "privacy_status",
    "op": "eq",
    "value": "public"
  },
  "sort": [
    { "field": "published_at", "order": "desc" }
  ],
  "page": {
    "offset": 0,
    "limit": 50
  }
}
```

意味:

- `user_query`: URL 共有対象になる、ユーザーが明示的に操作した条件
- `required_filter`: UI やアプリ方針で強制される条件
- 実行時には `effective_query = and(user_query, required_filter)` として扱う
- query parse の最初の段階で、文字列 ID は対応する整数 ID に解決する

### 8.2 node 種別

```ts
type QueryNode =
  | { type: "and"; children: QueryNode[] }
  | { type: "or"; children: QueryNode[] }
  | { type: "not"; child: QueryNode }
  | TermNode;

type TermNode =
  | { type: "term"; field: "artist_id"; op: "in"; values: string[] }
  | { type: "term"; field: "tag_id"; op: "in"; values: string[] }
  | { type: "term"; field: "channel_id"; op: "in"; values: string[] }
  | { type: "term"; field: "privacy_status"; op: "eq"; value: "public" | "unlisted" | "private" }
  | { type: "term"; field: "embeddable"; op: "eq"; value: boolean }
  | { type: "term"; field: "published_at"; op: "range"; gte?: number; gt?: number; lte?: number; lt?: number };
```

制約:

- `and.children`, `or.children` は 1 要素以上
- `not` は単項のみ
- `in` の意味は「いずれか 1 つ以上に一致」
- 同一 field に対する「すべて含む」を将来入れる場合は `op: "all_in"` を追加する

### 8.3 空クエリ

- `user_query` が空の場合は `match_all` 相当として扱う
- 物理表現として `null` を許してよい
- `required_filter` のみが存在する状態を正当なクエリとする

## 9. URL パラメータ方針

- URL には `user_query` のみを載せる
- `required_filter` は URL に含めない
- URL 上は AST 全体を JSON 文字列化して圧縮・エンコードしてよい
- ただし URL スキーマは engine ではなく frontend 側の責務とする

例:

- `?q=...` は `user_query` を表す
- `sort`, `offset`, `limit` は分離してもよいし、まとめて 1 オブジェクト化してもよい

## 10. 実行結果

検索エンジンの返り値は最低限以下を持つ。

```ts
type SearchResult = {
  total: number;
  clip_uuids: string[];
  next_offset: number | null;
};
```

要件:

- `clip_uuids` の順序は `sort` と `page` 適用後の順序を保証する
- `total` はページング前の総件数
- `next_offset` は次ページが存在しない場合 `null`

将来拡張候補:

- `debug_info`
- `matched_doc_ids`
- `facet_counts`

ただし初版では返しすぎない。まずは `clipUuid` 列挙に責務を絞る。

内部実装としては以下の段階を分ける。

1. evaluator は `doc_id[]` または bitset を返す
2. top-level search は sort / paging 後に `doc_id[]` を `clip_id[]` に変換する
3. 公開 API が `clipUuid[]` を返すなら最後に辞書逆引きを行う

つまり、外部 API は `clipUuid[]` を返してよいが、内部処理は最後まで整数 ID ベースで進める。

## 11. フィルタ評価の方針

### 11.1 完全一致フィルタ

以下は posting list の積/和で処理する。

- `artist_id`
- `tag_id`
- `channel_id`
- `privacy_status`
- `embeddable`

WAND, MaxScore などは不要:

- 今回はスコア付き上位 K 検索ではない
- 主処理は boolean filter + sort 済み列の走査で足りる
- 先に `doc_id`, postings, dictionary の責務分離を固める方が重要

### 11.2 日付範囲フィルタ

`published_at` は以下のいずれかで実装する。

- sort column に対する二分探索
- 専用の range index

初版は `published_at asc` 列に対する二分探索で十分。

### 11.3 ソート

- まず filter で候補集合を作る
- その後 `published_at` のソート列順にスキャンし、候補集合に含まれる `doc_id` だけを拾う
- ページサイズに達したら停止してよい

## 12. バージョニング方針

- バイナリ format version を必須とする
- query schema version も別で持てるようにする
- builder と engine の互換性判定は `format_version` を基準に行う
- 後方互換を壊す変更は major version を上げる

最低限ほしい識別子:

- `index_format_version`
- `query_schema_version`
- `builder_version`

## 13. エラー方針

- 未知の field / op / enum 値は query parse error
- 未知の辞書 ID 参照は empty result として扱ってよい
- 壊れたバイナリ、version 不一致、section 欠損は load error
- builder 入力不整合は build error として失敗すべき

## 14. 初版の非目標

初版では以下をやらない。

- 自由文全文検索
- スコアリング
- 複数キー複合ソート
- facet 集計
- ハイライト
- あいまい検索

まずは **構造化フィルタ + 投稿日ソート** に絞る。

## 15. 実装メモ

- `search/index_fmt`:
  - バイナリフォーマット定義
  - header / section / dictionary / postings の serialize / deserialize
- `search/engine`:
  - query AST 定義
  - filter evaluator
  - sort / paging
  - WASM 公開 API

Rust 内部での代表型イメージ:

```rust
pub struct SearchRequest {
    pub user_query: Option<QueryNode>,
    pub required_filter: Option<QueryNode>,
    pub sort: Vec<SortSpec>,
    pub page: PageSpec,
}

pub struct SearchResponse {
    pub total: u32,
    pub clip_uuids: Vec<String>,
    pub next_offset: Option<u32>,
}
```

より内部寄りの型イメージ:

```rust
pub type DocId = u32;
pub type ClipId = u32;
pub type VideoId = u32;
pub type ChannelId = u32;
pub type ArtistId = u32;
pub type TagId = u32;
```

## 16. 今後詰めるべき点

- `artist_id` が clip に複数付くときの意味論を source データ側と完全に揃える
- `privacy_status = private` を本当に index 化するか
- graduated artist の扱いを query 条件として持つか、前段の正規化で吸収するか
- manifest JSON の公開粒度
- URL エンコード形式(JSON 直列化 / compact binary / base64url など)
