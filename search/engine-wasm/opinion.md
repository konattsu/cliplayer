# 意見

## 重大な改善点

### 1. `u64` を JS `number` 扱いにするのは危険

一番危ないです。

JS の `number` は整数を安全に扱える範囲が `2^53 - 1` までです。
`index_build_id` や `query_fingerprint` を `u64` のまま設計すると、将来ハッシュ値・ID・fingerprint が安全整数を超えた瞬間に cursor 検証が壊れます。

特に `query_fingerprint` はハッシュ値っぽいので、むしろ `u64` 全域を使いたくなる可能性が高いです。

改善案:

```ts
type IndexBuildId = string;
type QueryFingerprint = string;
```

または cursor token 内部だけでも文字列化するべきです。

```json
{
  "v": 1,
  "index_build_id": "12345678901234567890",
  "query_fingerprint": "9876543210987654321",
  "sort": {...},
  "last": {...}
}
```

「safe integer に収める」という運用制約は、長期的には弱いです。初版から string にしておく方が API 破壊を避けられます。

---

### 3. `JsError` message だけでは後で絶対に困る

文書では structured error object を初版必須にしていませんが、これは弱いです。

frontend 側では、少なくとも次を分岐したくなります。

* request バグ
* cursor 期限切れ
* index 不整合
* query too complex
* unsupported feature
* internal error

message 文字列だけだと UI 側で安全に分岐できません。

改善案:

`JsError` ではなく、少なくとも JS 側で読める code を持つ error にする。

```ts
type SearchErrorCode =
  | "INVALID_REQUEST"
  | "INVALID_CURSOR"
  | "CURSOR_INDEX_MISMATCH"
  | "QUERY_TOO_COMPLEX"
  | "CORRUPT_INDEX"
  | "UNSUPPORTED_FEATURE"
  | "INTERNAL";
```

WASM 側では `Result<JsValue, JsValue>` にして error object を返す方が扱いやすいです。

```ts
type SearchError = {
  code: SearchErrorCode;
  message: string;
  details?: unknown;
};
```

---

### 4. constructor が同期 load なのは UI freeze の危険がある

初版で async にしない方針は理解できます。
ただし index bytes が大きい場合、`new(index_bytes)` の中で decode/validate/load すると main thread が固まります。

特に frontend から使うなら、検索 index は数 MB〜数十 MB になりがちです。

改善案:

初版の同期 API は残してよいですが、設計上は明記すべきです。

```ts
// main thread で使う場合は小規模 index 前提
new WasmSearchEngine(indexBytes)
```

本格運用では Web Worker 前提にするのが安全です。

結論: Web Worker 前提で進める. 必要ならasyncも初版で導入する.

---

### 5. `Vec<u8>` constructor は JS 側 API として少し曖昧

Rust 側では `Vec<u8>` でよいですが、JS/TS から見ると実際には `Uint8Array` を渡す形になります。

設計文書では `Uint8Array 相当` と書いていますが、API 仕様として明記した方がいいです。

```ts
constructor(indexBytes: Uint8Array)
```

また、`wasm-bindgen` が `Vec<u8>` を受ける場合はコピーが発生します。
それ自体は初版で許容できますが、以下を明記すべきです。

* JS buffer は constructor 後に破棄してよい
* Rust 側に owned copy される
* index が大きい場合は初期化コストがある

---

## API 設計上の不足

### 6. request/response の versioning がない

SearchRequest の shape は将来ほぼ確実に変わります。

* sort field 追加
* term 種類追加
* total mode 追加
* scoring/sort mode 追加
* warning 追加
* cursor token version 更新

そのため request に API version を入れるか、WASM package version と token version の関係を明記するべきです。

例:

```ts
type SearchRequest = {
  api_version?: 1;
  query: QueryNode | null;
  sort: SortSpec[];
  page: PageSpec;
  total_mode: TotalMode;
};
```

必須にしなくても、cursor token には version 必須です。

---

### 7. `sort: Array<...>` なのに実質 `published_at` だけ

TypeScript 例では `sort` が配列ですが、field は `"published_at"` だけです。

これは仕様が曖昧です。

確認すべき点:

* 複数 sort を許可するのか
* 初版は sort 1 個限定なのか
* tie-breaker は常に `doc_id` なのか
* sort 未指定時の default は何か

改善案:

初版は単純にした方がよいです。

```ts
type SearchRequest = {
  sort: {
    field: "published_at";
    order: "asc" | "desc";
  };
};
```

内部では必ず、

```text
published_at + doc_id
```

で安定順序にする、と明記すべきです。

---

### 8. `has_more` と `next_cursor` の整合ルールが未定義

response に、

```ts
next_cursor: string | null;
has_more: boolean;
```

がありますが、組み合わせの意味を固定すべきです。

例えば:

| has_more | next_cursor | 意味     |
| -------- | ----------- | ------ |
| false    | null        | 次ページなし |
| true     | string      | 次ページあり |
| true     | null        | 不正     |
| false    | string      | 原則不正   |

この invariant を文書化し、テストすべきです。

---

### 9. `limit` の上限がない

`page.limit` は frontend から渡されるため、上限が必要です。

改善案:

```text
1 <= limit <= 100
```

または用途に応じて、

```text
default = 50
max = 200
```

を定義するべきです。

これがないと、巨大 limit によるメモリ使用量・応答時間悪化が起きます。

---

### 10. query complexity の定義がない

error に `query too complex` を想定していますが、何をもって complex とするかが未定義です。

最低限必要:

* AST node 数上限
* AND/OR children 数上限
* NOT の深さ制限
* values 配列の上限
* range 条件の数
* unknown ID の扱い

例:

```text
max_query_nodes = 128
max_values_per_term = 256
max_depth = 16
max_limit = 100
```

これは WASM では特に重要です。ブラウザ上で重い query を走らせると UI/Worker を占有します。

---

## serde / TypeScript 境界の問題

### 11. serde の enum tagging 方針がやや曖昧

文書では「adjacently tagged を優先」と書きつつ、TypeScript 例は実質 internally tagged に近いです。

例:

```ts
{ type: "and"; children: QueryNode[] }
```

これは `#[serde(tag = "type")]` で表現できます。

一方、adjacently tagged は普通こうです。

```json
{ "type": "term", "content": { ... } }
```

改善案:

どちらにするか明確化すべきです。

個人的には、今の TypeScript 例の形でよいです。

```rust
#[serde(tag = "type", rename_all = "snake_case")]
enum QueryNode {
    And { children: Vec<QueryNode> },
    Or { children: Vec<QueryNode> },
    Not { child: Box<QueryNode> },
    Term { term: TermNode },
}
```

`TermNode` も同様。

---

### 12. `DateRange` / `RangeBound` の JS 表現が未定義

検索条件に `published_at_range` があるなら、日付の表現を固定しないと壊れます。

結論: 内部のエンジンと同様に64bit timestamp seconds

---

### 13. unknown field の扱いが未定義

`serde` はデフォルトでは未知 field を無視することがあります。
API 境界ではこれがバグを隠します。

改善案:

request 系の型には原則:

```rust
#[serde(deny_unknown_fields)]
```

を付けるべきです。

特に query AST は typo を検出した方がいいです。

---

### 14. `null` と `undefined` の扱いが未定義

TypeScript 側では、

```ts
query: null
cursor: null
total: null
```

が出ています。

しかし JS では `undefined` も入り得ます。
`serde-wasm-bindgen` 変換で `undefined` をどう扱うかを確認し、仕様化した方がいいです。

おすすめ:

* public API では `undefined` 非推奨
* optional field は最小限
* absent と null の意味を分けない
* request は `null` を明示

---

## WASM / crate 構成の不足

### 15. wasm module を feature gate しないと native 側が汚れる

文書では feature 分離は必須でないとしていますが、本格実装なら最初から分けた方がいいです。

```toml
[features]
default = []
wasm = ["dep:wasm-bindgen", "dep:serde-wasm-bindgen", "dep:js-sys"]
```

```rust
#[cfg(feature = "wasm")]
pub mod wasm;
```

理由:

* native build に wasm-bindgen 依存を混ぜない
* CI が分けやすい
* 将来 CLI/server 側で不要な依存を避けられる

初版からやっても複雑さは小さいです。

---

### 16. `wasm32-unknown-unknown` で使えない依存の確認が不足

core engine 側に以下が混ざっていると WASM build で詰まります。

* `std::fs`
* `std::path` 依存の runtime 処理
* thread
* rayon
* tokio の一部機能
* OS 依存 API
* mmap
* time/local timezone 系

設計に「core engine は wasm32 で build 可能な依存に閉じる」を追加すべきです。

---

### 17. panic strategy が未定義

WASM で panic した場合の見え方は悪いです。

最低限:

```rust
console_error_panic_hook
```

を dev/test 用に入れるか検討すべきです。

ただし production 依存にするかは別です。

---

## テスト不足

### 18. `wasm-bindgen-test` がない

Rust の serde テストだけでは不十分です。
`serde-wasm-bindgen` は `serde_json` と挙動が違うことがあります。

追加すべき:

* `wasm-bindgen-test`
* Node.js または browser test
* JS/TS から実際に `new WasmSearchEngine(Uint8Array)` するテスト
* JS object を渡して search できるテスト
* error object/code のテスト

---

### 19. TypeScript 型生成の方針がない

文書では「将来 TypeScript 型定義を出したい」とありますが、本格実装なら初版から型定義の生成または手書き固定を決めるべきです。

選択肢:

* `tsify`
* `typescript-type-def`
* 手書き `.d.ts`
* frontend 側で zod schema を正とする

このプロジェクトでは zod を使っている文脈があるので、個人的にはこれが合います。

```ts
const SearchRequestSchema = z.object({...});
type SearchRequest = z.infer<typeof SearchRequestSchema>;
```

WASM に渡す前に frontend で validate できます。

結論: typescriptかつzodの使用を必須とする. しかし、wasm.mdはあくまで wasm 側の設計に集中させるため、ドキュメントにfrontendの内容を含める必要はないかも.

---

## 結論

この設計の方向性は正しいです。
ただし、今のまま実装すると後で危ないのは次です。

1. **`u64` を JS number にする前提**
2. **cursor token の versioning / validation / 改ざん耐性不足**
3. **`JsError` message 依存**
4. **query/page/sort の制約不足**
5. **serde enum shape の曖昧さ**
6. **Web Worker 前提の記述不足**
7. **wasm 実機テスト不足**

優先して直すなら、この順です。

```text
1. u64 は公開境界・token 内で string 化
2. cursor token v1 struct を定義
3. structured error code を初版から入れる
4. limit / query complexity / sort invariant を明記
5. serde tagging と DateRange 表現を固定
6. wasm feature gate と wasm-bindgen-test を追加
```

特に **cursor と error と number 表現** は、後から変えると frontend API 破壊になりやすいので、初版で固めるべきです。
