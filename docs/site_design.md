# cliplayer site design

`cliplayer` の静的 Angular サイトにおける、初版 frontend の概要設計。
詳細なデータ生成や検索内部実装ではなく、画面構成と frontend の責務に絞ってまとめる。

関連:

- 全体の生成フロー: [`./data_flow.md`](./data_flow.md)
- 検索システム概要: [`../search/overview.md`](../search/overview.md)
- WASM 境界: [`../search/engine-wasm/usage.md`](../search/engine-wasm/usage.md)
- frontend が読む生成データ:
  - [`../metadata/artist/format.md`](../metadata/artist/format.md)
  - [`../metadata/tag/format.md`](../metadata/tag/format.md)
  - [`../music/data/format.md`](../music/data/format.md)

## 目的

- 歌唱 clip を探しやすくする
- 見つけた clip を連続再生しやすくする

このサイトでは、検索単位も再生単位も `1 clip` とする。
同一動画に複数 clip が属することがあるため、clip 単体だけでなく動画内での並びも辿れる UI にする。

## 前提

- サイトは静的配信する
- frontend は Angular で実装する
- backend API は持たない
- 検索は `public/search/search_index.bin` と `search/engine-wasm` を使ってブラウザ内で実行する
- 表示データは `public/music/*.min.json` を読む

責務分離:

- Rust tools: 正データから min JSON と検索 index を生成する
- search engine: query 評価、sort、cursor paging、`clip_uuid` の返却を担当する
- frontend: URL と検索状態の同期、表示用 join、queue 管理、YouTube player 制御、永続化を担当する

## 設計原則

- 検索条件の正規化は 1 箇所に集約する
- lookup や join は component に散らさない
- queue と cursor は build/version の不整合を検出できるようにする
- 壊れたデータ、古い保存状態、埋め込み不可動画を通常系として扱う

## 画面構成

初版の route は次に絞る。

| route | 役割 |
| --- | --- |
| `/` | home |
| `/search` | 検索条件の編集と結果一覧 |
| `/play` | queue 再生 |
| `/clip/:clipUuid` | clip 直リンク |

### home

- サイト概要を短く伝える
- 主導線は `/search` に寄せる
- 必要なら代表タグやおすすめ clip への導線を少数だけ置く
- 埋め込み不可や動画削除で再生不能になる可能性は明示する

### search

- 検索条件を編集し、URL と同期する
- 検索結果を clip 単位で表示する
- 現在ページの結果から queue を作れるようにする

初版で扱う主な条件:

- artist
- tag
- channel
- `embeddable`
- `is_unlisted`
- `published_at` range
- sort

結果一覧では最低限、楽曲名、歌唱者、元動画タイトル、公開日時、タグ、clip 長、再生可否を表示する。
同一動画に複数 clip がある場合は sibling clip を辿れるようにする。

### play

- queue を再生する
- 現在 clip の情報を表示する
- 同一動画内 clip への移動を提供する
- 再生不可 item は理由付きで扱う

queue が空の場合は空状態を出し、`/search` へ戻す導線を置く。

## URL と状態

- 検索状態は `/search` の query string に保持する
- queue 本体は URL に載せない
- 単一 clip の直リンクは `/clip/:clipUuid` または `/play?clip=...` で表現する

`/play` の初期化規則:

- `/play?clip=<uuid>` は単一 clip queue を新規作成する
- `/clip/:clipUuid` は `/play?clip=<uuid>` と同義で扱う
- `/play` を query なしで開いた場合は保存済み queue を復元し、無理なら空 queue にする

## データと検索

frontend は次の生成済みアセットを利用する。

- `public/music/*.min.json`
- `public/search/search_index.bin`

前提:

- min JSON と検索 index は同じ `datasetBuildId` を共有する
- frontend は build/version の整合性を確認する
- `clip_uuid` から表示用情報を引く join は frontend 側で行う
- 検索 engine は Web Worker 内で動かし、main thread は UI と player 制御に専念する

検索条件の canonicalization は必須とする。
URL、form state、cache key、cursor 再利用判定は同じ正規化結果に依存する。

## 再生と永続化

- queue は clip UUID の列として管理する
- queue の保存には `IndexedDB` を使う
- 保存データには version と `datasetBuildId` を持たせる
- 復元時は version 不一致、build 不一致、参照切れ clip を検出して補正または破棄する

再生可否は `embeddable` だけでは決めない。
埋め込み不可、参照切れ、YouTube 側の利用不可など、UI に理由を出せる状態で扱う。

## エラー方針

初版から次を明示的に扱う。

- 初回データロード失敗
- min JSON の parse / validation 失敗
- search index / WASM 初期化失敗
- 検索結果 0 件
- queue 空
- queue 復元失敗
- clip 参照切れ
- 埋め込み不可
- YouTube API error

fatal error と partial degradation は分ける。
基本表示が成立しない場合だけ fatal とし、一部 clip の問題や保存データ破損は継続可能な異常として扱う。

## 初版でやらないこと

- backend API
- ログイン
- プレイリスト共有
- 全文検索
- facet 集計
- 検索結果全件 queue 化
- offline 対応
- 複数 player 同時管理

初版は、clip を安全に検索し、安全に連続再生できることに責務を絞る。
