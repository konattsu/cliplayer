
## 修正・改善すべき点

### 1. `/play` の queue 復元仕様が弱い

「queue は localStorage」とあるが、共有・復元・直リンクの扱いが曖昧です。

問題：

* `/play` を開いたとき queue が無い場合の挙動が未定義
* `/clip/:clipUuid` から来たときの queue 初期化規則が曖昧
* localStorage の古い queue が新しい build のデータと不整合を起こす可能性がある

改善：

* queue に `dataBuildId` / `indexBuildId` を持たせる
* 復元不能 clip は除外または警告表示
* `/play?clip=...` は常に単一 clip queue を作る、など明文化する

---

### 2. cursor paging の無効化条件が不足

cursor は「同一 build・同一 query・同一 sort のみ」と書けていますが、frontend 側でどう検証するかが弱いです。

改善：

```ts
type SearchCursorEnvelope = {
  cursor: string;
  queryKey: string;
  sortKey: string;
  indexBuildId: string;
};
```

URL の `cursor` をそのまま engine に渡さず、現在の `queryKey` と一致しなければ破棄するべきです。

---

### 3. 検索 request の canonicalization が重要なのに詳細不足

「URL パラメータを SearchRequest に正規化」とあるが、ここが実装上のバグ源になります。

特に必要：

* tag/artist/channel の順序を正規化
* 空配列と未指定の扱いを統一
* date range の境界を明確化
* `embeddable=false` と未指定の差を明確化
* `unlisted` パラメータ名と内部 `is_unlisted` の対応を固定

これをしないと、同じ検索条件なのに URL・cache・cursor・queryKey がズレます。

---

### 4. `MusicRepository` が肥大化しやすい

min JSON 群の読み込み、lookup、join、view model 化を全部 `MusicRepository` に寄せると巨大化します。

分けた方がよいです。

```text
MusicDataLoader
  fetch + parse + schema validation

MusicCatalogRepository
  id lookup

ClipViewModelAssembler
  clip_uuid -> 表示用 model

VideoSiblingResolver
  videoId -> sibling clips
```

`Repository` は lookup までにして、表示用 join は別層にした方が保守しやすいです。

---

### 5. 静的 JSON の schema validation が書かれていない

Rust tools が生成するとはいえ、frontend 側でも最低限の検証が必要です。

問題：

* 壊れた JSON
* 古いフォーマット
* 欠損 ID
* `clip.videoId` が `videos.min.json` に存在しない
* tag/liver/channel の参照切れ

改善：

* zod 等で parse
* `schemaVersion` / `dataBuildId` を全ファイルに入れる
* 起動時に互換性チェック
* fatal error と部分劣化表示を分ける

---

### 6. YouTube IFrame Player の状態遷移が未設計

player 制御は一番バグりやすいです。

不足：

* API script load 前
* player ready 前
* video load 中
* seek 中
* playing
* paused
* ended
* error
* embeddable error
* clip end timer active

`YoutubePlayerService` は薄い wrapper だけだと不十分です。最低限、状態機械として扱うべきです。

---

### 7. clip 終了検知の方式が未定義

「clip 終了秒で次へ」とあるが、YouTube IFrame API は clip 単位の終了イベントを出しません。

必要：

* `setInterval` / `requestAnimationFrame` / player state polling の方針
* タブ非アクティブ時のズレ対策
* seek 後の誤判定防止
* `endSec` 未満で止まった場合の扱い
* 連続再生時の race condition 対策

ここは明確に設計した方がよいです。

---

### 8. `blocked tag` の意味が曖昧

`blocked` が「再生禁止」なのか「検索非表示」なのか「注意表示」なのかが曖昧です。

改善：

```ts
type PlaybackAvailability =
  | { kind: "playable" }
  | { kind: "blocked-by-tag"; tagIds: number[] }
  | { kind: "not-embeddable" }
  | { kind: "deleted-or-unavailable" };
```

「なぜ再生できないか」を UI に出せる形にした方がよいです。

---

### 9. search と play の接続がやや粗い

「全件を再生」がありますが、検索結果が cursor paging の場合、全件 queue をどう作るかが未定義です。

問題：

* 現ページだけなのか全検索結果なのか
* 全件が数千件ある場合どうするか
* cursor を順に辿って queue 化するのか
* engine 側に `limit` を大きく渡すのか

改善：

* 初版は「現在ページを再生」だけにする
* 「検索条件から再生」は後回し
* 全件再生するなら最大件数を決める

---

### 10. localStorage の versioning が必要

queue を保存するなら、最低限これが必要です。

```ts
type PersistedPlayerState = {
  version: 1;
  dataBuildId: string;
  queue: PlayQueue;
  savedAt: string;
};
```

古い形式を読んで壊れるのを防ぐべきです。

---

### 11. error / loading / empty state が未定義

静的サイトでも失敗は起きます。

必要な画面状態：

* 初回ロード中
* 検索 index 読み込み中
* WASM 初期化失敗
* JSON fetch 失敗
* 検索結果 0 件
* clip 参照切れ
* YouTube 埋め込み不可
* queue 空
* localStorage 復元失敗

ここを初版から決めておくべきです。

---

### 12. Angular の状態管理方針が少し曖昧

`SearchPageState`, `PlayerService` などは良いですが、Angular なら Signals / RxJS のどちらを主にするか決めた方がよいです。

改善案：

* UI 状態: Angular Signals
* 非同期 stream / Worker / Player event: RxJS
* service 外部公開は readonly signal または observable
* component 側で直接 mutable state を持ちすぎない

---

## 優先度高い修正

まず直すべき順はこれです。

1. `dataBuildId` / `schemaVersion` を全データ・queue・cursor に入れる
2. `SearchRequest` 正規化と `queryKey` 生成を設計する
3. `PlayerService` / `YoutubePlayerService` を状態機械として設計する
4. queue 復元・直リンク・localStorage の仕様を明文化する
5. `MusicRepository` を loader / repository / view model assembler に分割する

結論として、ページ構成や責務分離は良いです。ただし現状は「理想の画面設計」に寄っていて、**壊れたデータ・古い queue・cursor 不整合・YouTube player の非同期状態**への耐性がまだ弱いです。
