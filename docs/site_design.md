# cliplayer site design

この文書は、`cliplayer` の静的 Angular サイト初版の設計を定義する。
対象はページ構成だけではなく、frontend が扱うデータ境界、検索状態、
再生 queue、永続化、YouTube player 制御、異常系の扱いまで含む。

関連:

- 全体の生成フロー: [`./data_flow.md`](./data_flow.md)
- 検索基盤の責務: [`../search/overview.md`](../search/overview.md)
- WASM 境界: [`../search/engine-wasm/usage.md`](../search/engine-wasm/usage.md)
- frontend が読む min データ:
  - [`../metadata/artist/format.md`](../metadata/artist/format.md)
  - [`../metadata/tag/format.md`](../metadata/tag/format.md)
  - [`../music/data/format.md`](../music/data/format.md)

## 1. 目的

サイトの主目的は次の 2 つである。

- 歌唱 clip を見つけやすくする
- 見つけた clip を連続再生しやすくする

このサイトでは、**検索単位も再生単位も 1 clip** とする。
ただし clip は動画の断片であり、同一動画に複数 clip が属することが多い。
そのため UI は clip を単独で見せるだけでなく、
「この clip がどの動画のどの並びにいるか」を常に分かるようにする。

## 2. 初版の前提

- サイトは静的配信する
- frontend は Angular で実装する
- backend API は持たない
- 検索は `public/search/search_index.bin` と `search/engine-wasm` を使い、ブラウザ内で実行する
- 表示データは `public/music/*.min.json` を読む

責務分離は次のとおり。

- Rust tools:
  - 正データから min JSON と検索 index を生成する
- search engine:
  - query 評価
  - sort
  - cursor paging
  - `clip_uuid` の返却
- frontend:
  - URL と検索状態の相互変換
  - `clip_uuid` から表示データを引く join
  - queue 管理
  - YouTube player 制御
  - 永続化と復元

## 3. 設計原則

初版では次を強く守る。

- 検索条件の正規化を 1 箇所に寄せる
- queue と cursor は build/version 不整合を検出できるようにする
- 壊れたデータ、古い永続状態、埋め込み不可動画を正常系の一部として設計する
- 同じ lookup や join を各 component で手書きしない
- page 設計よりも、状態遷移と異常系を先に固定する

## 4. 画面構成

初版の必須ページは 3 つとする。

- `/`
  - home
- `/search`
  - 検索条件の編集と結果一覧
- `/play`
  - 現在の queue を再生する画面

補助 route:

- `/clip/:clipUuid`
  - clip 直リンク
  - 実装上は `/play?clip=...` に寄せてもよいが、外部共有用の安定 URL として route は持つ

## 5. ページ要件

### 5.1 home

役割:

- サイトの説明
- `/search` への主導線
- データが揃っているなら、おすすめ導線を数本だけ出す

置く内容:

- ヒーロー
- 検索導線
- 代表タグ導線
- 最近追加または最近公開の clip 導線
- 注意事項
  - 埋め込み不可動画はサイト内再生できない
  - 動画削除や非公開化により再生不能になりうる

home は情報量を増やしすぎない。
主目的は `/search` に送ること。

### 5.2 search

責務:

- 検索条件の編集
- URL への検索状態保存
- 検索結果の表示
- 現在ページの結果から queue を作る

主要 UI:

- artist 入力補助
- tag filter
- channel filter
- `embeddable` filter
- `is_unlisted` filter
- `published_at` range
- sort
  - `published_at desc`
  - `published_at asc`
- result list
- current-page queue 作成アクション
- 個別再生 / 個別 queue 追加

初版では「全検索結果を再生」は入れない。
cursor paging のある検索結果全体を queue 化する仕様は複雑で、
初版で曖昧に入れるべきではない。

検索結果 1 件は clip を表す。最低限出す情報:

- 楽曲名
- 歌唱者
- 元動画タイトル
- 投稿日時
- タグ
- clip 再生時間
- 再生可否

同一動画に複数 clip があることを見せるため、結果カードでは次を出す。

- sibling clip 数
- 展開時の同一動画 clip 一覧
- その場で sibling clip を再生開始する導線

### 5.3 play

責務:

- queue の再生
- 現在 clip の表示
- 同一動画内 clip 遷移
- 再生不能 item の扱い

主要 UI:

- 現在再生中 clip 情報
- YouTube player またはフォールバック
- queue 一覧
- sibling clip 一覧
- 再生モード
  - 順番通り
  - 1 clip repeat
  - queue loop

現在再生中の表示項目:

- 楽曲名
- 歌唱者
- 元動画タイトル
- チャンネル表示
- clip の開始秒 / 終了秒
- 元動画を YouTube で開く導線
- 公式切り抜きがある場合の導線
- 再生不可理由

queue が空の場合は空状態を明示し、`/search` へ戻す導線を出す。

## 6. ルーティングと URL

想定 route:

| route             | 役割          |
| ----------------- | ------------- |
| `/`               | home          |
| `/search`         | 検索          |
| `/play`           | queue 再生    |
| `/clip/:clipUuid` | clip 直リンク |

URL で持つべき状態は検索中心に絞る。

- 検索状態
  - query string に保持
- queue 本体
  - URL には載せない
- 単一 clip 直リンク
  - `/clip/:clipUuid` または `/play?clip=...` で表現

`/search` の query string 候補:

- `artist`
- `tag`
- `channel`
- `embeddable`
- `unlisted`
- `from`
- `to`
- `sort`
- `cursor`

`/play` の初期化規則:

- `/play?clip=<uuid>`
  - 常に単一 clip queue を新規作成する
- `/clip/:clipUuid`
  - `/play?clip=<uuid>` と同義で扱う
- `/play` を query なしで開いた場合
  - 永続化された queue を復元できれば使う
  - 復元不能なら空 queue として扱う

## 7. データアセットと build metadata

### 7.1 読み込む静的アセット

初版 frontend は次を読む。

- `public/music/livers.min.json`
- `public/music/channels.min.json`
- `public/music/official_channels.min.json`
- `public/music/livers_search_index.min.json`
- `public/music/tags.min.json`
- `public/music/clips.min.json`
- `public/music/videos.min.json`
- `public/search/search_index.bin`

### 7.2 build metadata の要件

frontend 側で整合性確認を行うため、min JSON 群と検索 index には
少なくとも次の metadata を持たせたい。

- `schemaVersion`
- `datasetBuildId`

min JSON は次の共通 shape を持つ。

```ts
type MinEnvelope<T> = {
  schemaVersion: number;
  datasetBuildId: string;
  data: T;
};
```

`datasetBuildId` は opaque string とし、frontend は等値比較だけを行う。
形式は `^[a-z0-9][a-z0-9._-]{7,127}$` とする。

検索 index 側は JSON と同じ shape に揃える必要はない。
最低限、frontend が比較可能な `datasetBuildId` と、
互換性確認に使える binary format version を読める必要がある。

注意:

- min JSON と検索 index は同じ `datasetBuildId` を共有する
- 検索 index 側では加えて binary format version を必須にする

### 7.3 frontend が必要とする役割分担

| データ                         | 用途                     |
| ------------------------------ | ------------------------ |
| `clips.min.json`               | clip の主データ          |
| `videos.min.json`              | 動画情報と sibling 解決  |
| `livers.min.json`              | artist 表示              |
| `official_channels.min.json`   | 公式チャンネル表示       |
| `channels.min.json`            | `channelId` の所属解決   |
| `tags.min.json`                | tag 表示と playback 制約 |
| `livers_search_index.min.json` | artist 入力補助          |
| `search_index.bin`             | clip 検索                |

## 8. データロードと validation

Rust tools が生成したデータであっても、frontend で最低限の validation は行う。

検出したい異常:

- JSON parse failure
- 想定外 schema
- 必須 field 欠損
- `clip.videoId` の参照切れ
- `liverIds`, `videoTags`, `channelId` の参照切れ
- build metadata 不一致

方針:

- fetch + parse + validation をロード層で実施する
- fatal error と partial degradation を分ける
- 検索 index が読めない場合でも、最低限の静的閲覧が可能かは後で判断する
- 参照切れ clip は再生不可扱いに落とせるようにする

validation ライブラリは `zod` 等を想定してよい。

## 9. frontend 内の責務分割

前回案の `MusicRepository` は責務が広すぎるので分割する。

想定する主 service / module:

- `MusicDataLoader`
  - fetch
  - parse
  - validation
  - build metadata の収集
- `MusicCatalogRepository`
  - clip / video / artist / tag / channel の lookup
- `ClipViewModelAssembler`
  - `clip_uuid` から表示用 model を構築
- `VideoSiblingResolver`
  - `videoId` から sibling clip 群を解決
- `SearchWorkerClient`
  - Worker 初期化
  - index load
  - search request 実行
- `SearchQueryCanonicalizer`
  - URL / form state / search request の正規化
- `PlayerService`
  - queue 制御
  - 再生状態機械
- `PlayerPersistence`
  - IndexedDB 永続化
- `YoutubePlayerAdapter`
  - YouTube IFrame API 境界

join は `ClipViewModelAssembler` に寄せ、component ごとに手書きしない。

## 10. 検索設計

### 10.1 基本方針

- search engine は Web Worker 内で動かす
- main thread は UI と YouTube player 制御に専念する
- frontend は構造化 request を組み立てるだけに留める

### 10.2 UI から作る query

初版で UI から生成する検索条件は、検索基盤の構造化 filter に合わせる。

- `artist_id any_in`
- `tag_id any_in`
- `channel_id any_in`
- `is_unlisted eq`
- `embeddable eq`
- `published_at range`

初版では全文検索を前提にしない。
自由入力欄を置く場合も、artist 補助検索など用途を限定して見せる。

### 10.3 canonicalization

検索条件の canonicalization は必須である。
URL、form state、cache key、cursor 再利用判定がここに依存する。

正規化規則:

- `artist`, `tag`, `channel` の複数値は sort + dedup する
- 空配列は未指定として扱う
- `embeddable` と `unlisted` は三値にする
  - 未指定
  - `true`
  - `false`
- `from`, `to` の境界の意味を固定する
- URL 上の `unlisted` は内部 `is_unlisted` に必ず写像する
- sort 未指定時の default を固定する

生成物:

- `CanonicalSearchState`
- `SearchRequest`
- `queryKey`
- `sortKey`

`queryKey` は同一意味の検索条件なら必ず同じ値になる必要がある。

### 10.4 cursor の扱い

cursor は opaque token として扱うが、URL の `cursor` をそのまま engine に流してはいけない。

frontend 側で持つ envelope の例:

```ts
type SearchCursorEnvelope = {
  cursor: string;
  queryKey: string;
  sortKey: string;
  datasetBuildId: string;
};
```

再利用条件:

- 現在の `queryKey` が一致する
- 現在の `sortKey` が一致する
- 現在の `datasetBuildId` が一致する

1 つでもずれたら cursor は破棄し、先頭ページから再検索する。

## 11. 検索結果と queue の接続

初版でサポートする queue 生成は次に限定する。

- 単一 clip を再生
- 現在ページの検索結果だけで queue を作る
- 同一動画内 clip 群だけで queue を作る

やらないこと:

- cursor を辿って全検索結果を queue 化する
- 数千件規模の queue 自動生成

この制限により、検索設計と再生設計の境界を単純に保つ。

## 12. Playback availability

再生可否は `embeddable` だけでは決まらない。
UI が理由を表示できる形で持つ。

例:

```ts
type PlaybackAvailability = { kind: "playable" } | { kind: "blocked-by-tag"; tagIds: string[] } | { kind: "not-embeddable" } | { kind: "missing-video" } | { kind: "missing-clip" } | { kind: "unavailable-on-youtube" };
```

`tags.min.json` の `blocked` は、初版では **検索非表示には使わない**。
意味は「queue 追加不可 / 再生不可」とする。
見えていてもよいが、理由付きで無効化する。

## 13. Queue と永続化

### 13.1 queue の内部表現

queue は clip UUID 列を中心に持つ。
加えて build/version を持たせる。

```ts
type PlayQueue = {
  version: 1;
  datasetBuildId: string;
  clipUuids: string[];
  currentIndex: number;
  source: { kind: "single-clip"; clipUuid: string } | { kind: "search-page"; queryKey: string } | { kind: "video"; videoId: string };
};
```

### 13.2 永続化先

初版から `localStorage` ではなく `IndexedDB` を使う。

理由:

- queue は将来的に長くなりうる
- build metadata や保存時刻、再生設定など周辺情報も保存したい
- schema version を持つレコード管理に向いている
- 復元失敗時の削除や migration を扱いやすい

`localStorage` は使わないか、使っても軽い UI 設定程度に留める。

### 13.3 永続化レコード

```ts
type PersistedPlayerState = {
  version: 1;
  savedAt: string;
  queue: PlayQueue;
  playbackMode: "normal" | "repeat-one" | "loop-queue";
};
```

復元時の規則:

- `version` 不一致なら破棄
- `datasetBuildId` 不一致なら queue を再検証する
- clip が存在しない item は除外する
- 全件消えたら空 queue に落とす
- 復元に失敗したら warning を表示し、壊れた保存データは消す

## 14. YouTube player 設計

### 14.1 薄い wrapper ではなく状態機械として扱う

YouTube IFrame API は非同期状態が多く、単純 wrapper では壊れやすい。
`YoutubePlayerAdapter` と `PlayerService` の間で状態機械を持つ。

最低限必要な状態:

- `idle`
- `script-loading`
- `player-creating`
- `ready`
- `video-loading`
- `seeking`
- `playing`
- `paused`
- `clip-ending-watch-active`
- `ended`
- `embeddable-error`
- `api-error`

### 14.2 clip 終了検知

YouTube API は clip 単位終了イベントを持たないため、frontend 側で検知する。

初版方針:

- 再生中だけ定期 polling する
- polling は `setInterval` ベースでよい
- interval は荒すぎず細かすぎない値に固定する
- `currentTime >= endSec - epsilon` で clip 終了と判定する
- seek 直後の誤判定を避けるため、短い grace period を入れる

補足:

- タブ非アクティブ時の timer 遅延は起こりうる
- そのため `endSec` を少し超えても next へ進めばよい設計にする
- `endSec` 到達前に pause された場合は自動遷移しない

### 14.3 埋め込み不可と再生エラー

埋め込み不可や YouTube 側 error は通常系として扱う。

- queue 上では再生不可 item として可視化
- 現在 item が再生不能なら理由を表示
- 連続再生中なら次 item へスキップ可能にする
- 「YouTube で開く」導線は常に残す

## 15. 状態管理方針

Angular 側の状態管理は次で統一する。

- UI 状態
  - Signals を主に使う
- Worker 通信、YouTube event、永続化 I/O
  - RxJS を使う
- service が公開する state
  - readonly signal または observable に限定する

component 側で mutable な複製状態を増やしすぎない。

## 16. Angular ディレクトリ方針

`components/`, `hooks/` のような曖昧な置き場は作らない。
feature と責務で切る。

```text
src/app/
  app.routes.ts
  core/
    data/
    search/
    player/
    youtube/
    persistence/
  features/
    home/
    search/
    play/
    clip/
  shared/
    ui/
    format/
```

役割:

- `core/`
  - アプリ共通 service と repository
- `features/`
  - route 単位の画面機能
- `shared/ui/`
  - 再利用 UI
- `shared/format/`
  - 表示整形

## 17. エラー・ローディング・空状態

初版から次の状態を明示的に設計する。

- 初回データロード中
- min JSON fetch 失敗
- min JSON validation 失敗
- search index 読み込み中
- WASM 初期化失敗
- 検索結果 0 件
- queue 空
- queue 復元失敗
- clip 参照切れ
- 埋め込み不可
- YouTube API error

fatal と non-fatal を分ける。

- fatal:
  - 必須 min データが壊れていて基本表示が成立しない
- non-fatal:
  - 一部 clip だけ参照切れ
  - queue 復元失敗
  - 埋め込み不可

## 18. 実装順

実装は次の順がよい。

1. Rust tools 側で `schemaVersion`, `datasetBuildId`, `formatVersion` の方針を固める
2. `MusicDataLoader` と validation を作る
3. `MusicCatalogRepository`, `ClipViewModelAssembler`, `VideoSiblingResolver` を作る
4. `SearchQueryCanonicalizer` と `/search` の URL 同期を作る
5. `SearchWorkerClient` で index load と search を通す
6. `PlayerService` と `YoutubePlayerAdapter` の状態機械を作る
7. `IndexedDB` 永続化を入れる
8. `/play` の復元・直リンク・skip を詰める
9. `home` を整える

## 19. 初版でやらないこと

- backend API
- ログイン
- プレイリスト共有
- 全文検索
- facet 集計
- 検索結果全件 queue 化
- offline 対応
- 複数 player 同時管理

初版は、clip を安全に検索して安全に連続再生できることに責務を絞る。
