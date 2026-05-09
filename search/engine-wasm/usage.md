# search/engine wasm design

この文書は `search/engine` を WASM から利用するための境界設計を扱う。
engine 本体の検索設計は [`design.md`](../engine/design.md) を参照。

## 1. 目的

WASM 対応で追加したい責務は次の 1 本である。

- Rust の `SearchEngine` を frontend から安全かつ素直に呼べる公開境界を作る

ここでいう公開境界は、単に `wasm-bindgen` を付けるだけではなく、
frontend から扱いやすい request / response 形状と error 形状を含む。

非目的:

- engine 本体の検索アルゴリズムを WASM 専用に書き換えること
- binary index format を WASM の都合で変更すること
- 初版で multi-thread, streaming decode, incremental load を入れること

## 2. 基本方針

### 2.1 core engine と wasm facade を分離する

既存の `SearchEngine` は Rust ネイティブ API として残す。
WASM のために engine 本体の API を JS 都合へ寄せすぎない。

採用方針:

- `src/engine.rs`
  - Rust ネイティブの主実装を保持する
- `src/wasm.rs`
  - `wasm-bindgen` で公開する薄い facade を置く
- `src/lib.rs`
  - 通常の Rust 公開 API と wasm 公開 API の両方を束ねる

この分離を採る理由:

- 検索ロジックと FFI 境界の責務が混ざらない
- Rust 向けテストをそのまま維持できる
- 将来、WASM 以外の FFI 境界を増やしても core を使い回せる

### 2.2 wasm facade は薄く保つ

WASM 層で行うのは主に次だけに絞る。

- `Uint8Array` 相当の bytes を `Arc<[u8]>` へ変換する
- `JsValue` を `SearchRequest` へ変換する
- `SearchResponse` を `JsValue` へ変換する
- Rust の `EngineError` を JS で扱える error へ変換する

query の正規化、辞書解決、paging、cursor 検証は
既存の `SearchEngine::search` に寄せる。

## 3. 公開 API の目標形

初版の目標形は次を想定する。

```rust
#[wasm_bindgen]
pub struct WasmSearchEngine {
    inner: crate::SearchEngine,
}

#[wasm_bindgen]
impl WasmSearchEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(index_bytes: Vec<u8>) -> Result<WasmSearchEngine, JsValue>;

    pub fn search(&self, request: JsValue) -> Result<JsValue, JsValue>;
}
```

意味:

- constructor
  - build 済み binary index を受けて engine を load する
- `search`
  - 構造化 request を受け、構造化 response を返す

公開関数は同期のままにする。
ただし静的サイトでの実運用は main thread 直呼びではなく、
Web Worker 内での利用を前提にする。

この設計を採る理由:

- 検索処理と index load は CPU バウンドであり、main thread では UI を止めやすい
- 静的サイトでは server 側へ逃がせないため、browser 内の実行場所の設計が重要になる
- API を async にしなくても、Worker 経由で UI freeze は避けられる

## 4. JS 境界のデータ方針

### 4.1 request / response は構造化データで受け渡す

frontend と engine の境界では URL パラメータ列ではなく
構造化 request をやり取りする、という `search/overview.md` の前提を維持する。

そのため初版では:

- `search(request: JsValue)` で構造化 request を受ける
- response も構造化 object として返す

opaque な JSON string を受け取って中で parse する方式は採らない。
理由:

- 型不一致の検知が遅くなる
- frontend 側のデバッグ性が落ちる
- 将来 TypeScript 型定義を出したい時に不利である

### 4.2 `serde` を正として `JsValue` と相互変換する

AST を含む request / response を手書きで JS 変換すると壊れやすい。
そのため API 型は `serde` 可能にする。

採用方針:

- `SearchRequest`
- `SearchResponse`
- `Cursor`
- `QueryNode`
- `TermNode`
- `SortSpec`
- `PageSpec`
- `SortField`
- `SortOrder`
- `TotalMode`
- `DateRange`
- `RangeBound`
- `QueryWarning`

に `serde::{Serialize, Deserialize}` を実装する。

WASM 境界では `serde-wasm-bindgen` を使い、
`JsValue <-> Rust struct` 変換を行う。

request を受ける型には原則 `#[serde(deny_unknown_fields)]` を付ける。
静的サイトでは frontend と wasm のバージョンずれが実運用で起こりうるため、
未知 field の黙殺でバグを隠さないことを優先する。

## 5. index bytes の受け取り方

### 5.1 constructor は owned bytes を受け取る

既存 API は `SearchEngine::load(Arc<[u8]>)` を要求する。
WASM 側では JS から受けた bytes を Rust 所有へ変換したうえで
`Arc<[u8]>` に包んで load する。

```rust
let bytes: std::sync::Arc<[u8]> = index_bytes.into();
let inner = crate::SearchEngine::load(bytes)?;
```

この設計を採る理由:

- JS 側の buffer lifetime に依存しない
- `LoadedIndex` の所有形と整合する
- 後続 request で bytes を再受領する必要がない

JS / TypeScript から見た API 仕様としては、
constructor 引数は `Uint8Array` とみなす。

明記しておく前提:

- constructor 呼び出し後、呼び出し側は元の `Uint8Array` を再利用してよい
- Rust 側には owned copy が渡る
- index が大きい場合、load 時に相応の初期化コストが発生する

### 5.2 初版では zero-copy 最適化を狙いすぎない

`Uint8Array` から Rust 所有メモリへのコピーは発生しうる。
ただし初版では correctness と API 安定性を優先する。

copy 削減は将来の最適化候補とし、初版の必須要件にはしない。

## 6. cursor の扱い

### 6.1 初版から opaque token を返す

`design.md` の cursor は内部的には次を持つ。

```text
dataset_build_id
query_fingerprint
sort_field
sort_order
last_published_at
last_doc_id
```

ただし wasm facade ではこの内部構造をそのまま公開しない。
公開 API では opaque token を返し、次 request でも opaque token を受ける。

理由:

- frontend 側が cursor の内部 schema に依存しなくて済む
- `dataset_build_id` や `query_fingerprint` の表現を JS 都合で漏らさずに済む
- 将来 cursor payload を変更しても frontend API を壊しにくい

### 6.2 token 化の責務は wasm facade 側に寄せる

opaque token は公開 API の都合であり、検索 core 自体の責務ではない。
そのため token の encode / decode は wasm facade 側の責務として扱う。

採用方針:

- core の `SearchResponse` は引き続き `Option<Cursor>` を持つ
- wasm facade は `Option<Cursor>` を `Option<String>` に変換して返す
- 次 request では `Option<String>` を decode して `Option<Cursor>` に戻す

この分離を採ると、Rust ネイティブ API は構造化 cursor のまま維持できる。

### 6.3 token 形式

初版では token は URL-safe な文字列にする。
候補は base64url である。

token payload は次のどちらかで表現する。

1. cursor 構造体をそのまま `serde` で直列化する
2. wasm 専用の token payload struct を定義して直列化する

推奨は 2 である。
公開 token に乗せる shape を Rust 内部 struct から切り離せるため、
将来 core の `Cursor` を変更しても token schema を安定させやすい。

token payload には必ず version を持たせる。

例:

```json
{
  "v": 1,
  "dataset_build_id": "dataset-build-20260509abcdef0123456789abcdef0123456789abcdef01234567",
  "query_fingerprint": "456",
  "sort_field": "published_at",
  "sort_order": "desc",
  "last_published_at": 1710000000,
  "last_doc_id": 42
}
```

ここでの `dataset_build_id` と `query_fingerprint` は token 内部では文字列として保持する。
公開 API では opaque token のため frontend はこれを意識しないが、
token schema としては build identity の内部表現を frontend に漏らさない。

### 6.4 response invariant

`has_more` と `next_cursor` の関係は次で固定する。

- `has_more = false` のとき `next_cursor = null`
- `has_more = true` のとき `next_cursor` は non-null

したがって、次の組み合わせは不正である。

- `has_more = true` かつ `next_cursor = null`
- `has_more = false` かつ `next_cursor != null`

この invariant は API 契約として文書化し、テストでも固定する。

## 7. error の扱い

### 7.1 Rust の `EngineError` は structured error object に変換する

WASM 公開関数は `Result<JsValue, JsValue>` とし、
error 側には structured object を返す。

最低限の shape は次を想定する。

```ts
type SearchError = {
  code: "INVALID_REQUEST" | "INVALID_CURSOR" | "QUERY_TOO_COMPLEX" | "CORRUPT_INDEX" | "VERSION_MISMATCH" | "UNSUPPORTED_FEATURE" | "INTERNAL_INDEX" | "BINARY" | "INTERNAL";
  message: string;
};
```

message は人間向け、`code` は frontend 分岐向けである。
静的サイトでは backend の再解釈層が存在しないため、
message 文字列に依存しない分岐を最初から持つ価値が高い。

### 7.2 cursor 起因の失敗も code で区別する

`INVALID_CURSOR` のような大分類だけではなく、
必要なら将来 `details` を追加できる余地を持つ。

ただし初版では details を必須にせず、
まずは安定した `code` を固定する。

## 8. crate 構成と依存

### 8.1 追加する依存

初版候補:

- `wasm-bindgen`
- `serde`
- `serde-wasm-bindgen`

必要に応じて:

- `js-sys`

`serde_json` は JS 境界の一次手段にはしない。
`JsValue` と直接相互変換できる `serde-wasm-bindgen` を優先する。

### 8.2 crate type

WASM 出力のため、`Cargo.toml` では `cdylib` を有効にする。
通常の Rust ライブラリ利用も継続するため、`rlib` も維持する。

想定:

```toml
[lib]
crate-type = ["rlib", "cdylib"]
```

### 8.3 feature 分離

初版では feature 分離は必須ではないが、
将来のビルド時間や依存抑制を考えると次は有力である。

- `default = []`
- `feature = "wasm"`

ただし依存追加量が小さいうちは、まず単純構成で進めてもよい。

## 9. 予想される API shape

TypeScript から見た概形は次を想定する。

```ts
type SearchRequest = {
  api_version?: 1;
  query: QueryNode | null;
  sort: {
    field: "published_at";
    order: "asc" | "desc";
  };
  page: {
    limit: number;
    cursor: string | null;
  };
  total_mode: "exact" | "none";
};

type SearchResponse = {
  clip_uuids: string[];
  next_cursor: string | null;
  total_mode: "exact" | "none";
  total: number | null;
  has_more: boolean;
  warnings: QueryWarning[];
};
```

`QueryNode` と `TermNode` は判別子付き object にする。
`type` を discriminator とする internally tagged な形を採る。

例:

```ts
type QueryNode = { type: "and"; children: QueryNode[] } | { type: "or"; children: QueryNode[] } | { type: "not"; child: QueryNode } | { type: "term"; term: TermNode };

type TermNode = { type: "artist_any_in"; values: string[] } | { type: "tag_any_in"; values: string[] } | { type: "channel_any_in"; values: string[] } | { type: "is_unlisted_eq"; value: boolean } | { type: "embeddable_eq"; value: boolean } | { type: "published_at_range"; range: DateRange };
```

この形を選ぶ理由:

- TypeScript の discriminated union と相性がよい
- frontend 側の分岐が素直になる
- variant ごとに payload 名を安定して明示できる

`DateRange` は engine 内部と同様に unix timestamp seconds を使う。
静的サイトでは browser locale や timezone に引きずられたくないため、
WASM API でも日時文字列は使わない。

```ts
type RangeBound = {
  value: number;
  inclusive: boolean;
};

type DateRange = {
  lower: RangeBound | null;
  upper: RangeBound | null;
};
```

### 9.1 request validation 契約

静的サイト上での暴走を防ぐため、WASM 境界の契約として次を明示する。

- `page.limit` は `1 <= limit <= 1000`
- `sort` は単一指定のみで、初版は `published_at` 固定
- `query = null` は match-all
- `And([])` と `Or([])` は不正 request

内部の `design.md` でも複雑性制限はあるが、
WASM API 文書でもユーザー入力境界として明示しておく。

### 9.2 query complexity 契約

初版では次の上限を設ける前提で設計する。

- `max_query_depth = 16`
- `max_boolean_nodes = 128`
- `max_terms = 128`
- `max_values_per_any_in = 256`

値そのものは実装時に最終調整してよいが、
「ブラウザ内で無制限の query を受けない」ことは設計として固定する。

## 10. 実装順

安全な実装順は次のとおり。

1. API 型へ `serde` を追加する
2. `src/wasm.rs` に `WasmSearchEngine` を追加する
3. `Cargo.toml` に wasm 用依存と `crate-type` を追加する
4. Rust 単体テストで request / response の serialize 形を固定する
5. `wasm32-unknown-unknown` で build できることを確認する

この順を採る理由:

- 型 shape を先に固定しないと frontend 連携がぶれる
- facade は薄いため、最後に組んでも core への影響が小さい
- build 設定だけ先に変えると、未使用コードや未整理依存が増えやすい

## 11. テスト方針

最低限ほしいテストは次のとおり。

- `SearchRequest` の `JsValue -> Rust` 変換が通る
- `SearchResponse` の `Rust -> JsValue` 変換が通る
- 未知 ID warning が JS 側へ落ちる
- `next_cursor` を次 request に戻してページ継続できる
- 不正 cursor が JS 側で error になる
- 壊れた index bytes を constructor が reject する

WASM 専用テスト基盤をすぐ入れない場合でも、
少なくとも `serde` 変換の Rust テストで API shape は固定する。

## 12. 設計上の決定事項

実装前に決めるべき論点は次で確定する。

- cursor は初版から opaque token にする
- enum の JSON shape は `type` を持つ判別子付き object にする
- 公開 API で `u64` を直接露出しない
- `query_fingerprint` は frontend 公開 API には出さない
- error は structured object を返す
- 実運用は Web Worker 前提にする

### 12.1 enum の JSON shape

`QueryNode` と `TermNode` は判別子付き object にする。
Rust 実装では `serde` の tagging 指定でこれを実現する。

external tag を避ける理由:

- frontend で payload の取り回しがやや不自然になる
- nested enum のとき TypeScript 型が読みにくくなりやすい

internal tag だけで押し切らない理由:

- tuple/newtype variant を混ぜたとき制約が増えやすい
- payload key を variant ごとに明示しづらい

そのため、初版は判別子フィールドを `type` に寄せた object 形を採る。

### 12.2 `u64` を公開 API で直接扱わない前提

当初案では `u64` を JS `number` で扱う案もありえたが、
長期運用では `query_fingerprint` などの安全性が弱い。
静的サイトでは一度配布した token が長く残りやすいため、
ここを運用制約に頼る設計は避ける。

この前提を採る理由:

- frontend 公開 API が単純なまま保てる
- token 内部だけで `u64` の表現戦略を完結できる
- 将来 `u64` の実値域を広げても public API を壊さない

したがって:

- request / response の公開 shape では `u64` フィールドを持たない
- token payload 内では文字列化を許容する
- Rust ネイティブ API の `Cursor` は現状の `u64` のまま維持する

### 12.3 `query_fingerprint` の公開範囲

`query_fingerprint` は cursor 妥当性検証の内部データであり、
frontend の意味論として直接扱わせる価値が低い。

そのため:

- request object に `query_fingerprint` フィールドは置かない
- response object にも `query_fingerprint` フィールドは置かない
- token payload の内部値としてのみ持つ

これにより、frontend 側は「同じ query で cursor を再利用する」という
挙動だけ意識すればよく、検証用フィールドの存在を知らなくてよい。

### 12.4 静的サイト前提の運用

静的サイトでは:

- index bytes は事前生成して配信する
- browser 側で fetch して `Uint8Array` 化する
- WASM engine は Web Worker 内で load / search する

この運用を前提にすると、API 設計で重視すべきは
「CPU バウンド処理を UI thread に置かないこと」と
「frontend が string token と構造化 request だけで扱えること」である。

## 13. 初版の推奨結論

現時点では次で進めるのが妥当である。

- core engine は現状の Rust API を維持する
- WASM 専用の薄い facade を別 module で追加する
- request / response は `serde-wasm-bindgen` で構造化データとして渡す
- cursor は初版から opaque token にする
- `QueryNode` / `TermNode` は `type` を持つ判別子付き object にする
- `query_fingerprint` は token 内部に閉じ込める
- token payload には version を持たせ、内部の `u64` は文字列化を許容する
- public error は message 文字列ではなく `code` 付き object にする
- 実運用は static site + Web Worker 前提で main thread から切り離す

これにより、検索コアを崩さずに frontend から実際に呼べる最短経路を作れる。
