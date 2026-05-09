# cliplayer

## 現状

A の `s2` までは実装済み。
`s2` では frontend 用 min JSON 群と検索インデックスを同時に生成する。

## A. データと生成物

このリポジトリでは、以下の 3 系統のデータを扱う。

- **artist**: ライバー / 公式チャンネル等のメタデータ
- **tag**: 動画・クリップに付与するタグ定義
- **clips**: 動画と、その中の歌唱クリップ定義

さらに、これらから用途別に派生生成物を作る。

- **フロント用 min データ**: 軽量・参照しやすい形式。主に `*.min.json`
- **検索用インデックス**: 検索エンジン向けの `search_index.bin`
- **定義支援(snippet)ファイル**: 手入力を支援する VS Code snippet

## Build Metadata

frontend が生成物の整合性を検証できるように、`s2` の生成物には
build metadata を持たせる。

最低限必要な項目:

- `schemaVersion`
  - min JSON の frontend-facing schema version
- `datasetBuildId`
  - min JSON 群と検索インデックスがどの入力集合から生成されたかを識別する ID
- binary `formatVersion`
  - frontend が検索インデックスの互換性確認に使える binary format version

min JSON は次の envelope を持つ。

```ts
type MinEnvelope<T> = {
  schemaVersion: number;
  datasetBuildId: string;
  data: T;
};
```

`datasetBuildId` は opaque string として扱う。

- frontend は等値比較だけを行い、内部構造に依存しない
- 文字種と長さは `^[a-z0-9][a-z0-9._-]{7,127}$`
- `tools/build.sh` が `s2` build ごとに 1 回だけ生成し、すべての `*.min.json` と `search_index.bin` に同じ値を注入する

原則:

- 同じ `s2` build に属する min JSON 群と検索インデックスは同じ `datasetBuildId` を共有する
- `schemaVersion` は min JSON の後方互換性が壊れるときだけ上げる
- `formatVersion` は検索バイナリ format の後方互換性が壊れるときだけ上げる
- frontend は `datasetBuildId` を queue 復元や cursor 再利用判定に使う
- metadata は frontend が追加 fetch なしで読める位置に置く
  - JSON は同一ファイルのトップレベル
  - binary index は metadata section

### 重要な原則

- **source-of-truth(正)は常に「人が編集する定義ファイル」**に置く
- **派生物は正から生成**する。派生物から別の派生物を作る派生-from-派生は避ける
- **同じ入力から同じ出力が得られる**ようにする
- **同じ入力集合を表す build identity は 1 つに統一**する

## 処理ステップ

重要: `s1`, `s2` はすべて **s0 のみ** を入力とする。
数字は処理の複雑さや頻度を示すもので、処理の順序を示すものではない。

### s0. 定義ファイルを編集する(手動)

最初に編集するのは、以下の source-of-truth。

- artist
  - `metadata/artist/data/livers.json`
  - `metadata/artist/data/official_channels.json`
- tag
  - `metadata/tag/data/tags.json`
- clips
  - `music/data/input/*.json`
  - `music/data/music/YYYY/MM.json`

### s1. 定義支援(snippet)を生成・更新する(頻繁)

目的: 手入力のコストを下げ、記述揺れを減らす。

- artist → `.vscode/music.code-snippets` を更新
- tag → `.vscode/tags.code-snippets` を更新

実行例:

- `metadata artist snippet`
- `metadata tag snippet`

原則: snippet は **正データの補助**であり、正データの代替ではない。

### s2. frontend 用 min データと検索インデックスを生成する(頻繁)

目的:

- Angular frontend が軽量に読み込める min JSON を提供する
- 同じ source-of-truth から検索インデックスも同時に生成する

要件:

- すべての min JSON は `schemaVersion`, `datasetBuildId`, `data` を持つ
- 検索インデックスは `datasetBuildId` と `formatVersion` を持つ
- 同一 build に属する min JSON 群と検索インデックスは同じ `datasetBuildId` を共有する
- frontend が参照整合性と build 一致を確認できる shape にする

artist:

- 出力先: `public/music/`
- 出力例: `livers.min.json`, `channels.min.json`, `official_channels.min.json`, `livers_search_index.min.json`
- 生成: `metadata artist minify --dataset-build-id <id>`

tag:

- 出力先: `public/music/`
- 出力例: `tags.min.json`
- 生成: `metadata tag minify --dataset-build-id <id>`

clips:

- 出力先: `public/music/`
- 出力例: `clips.min.json`, `videos.min.json`
- 生成: `musictl build minify --dataset-build-id <id>`

search:

- 出力先: `public/search/`
- 出力例: `search_index.bin`
- 生成: `index-builder --dataset-build-id <id>`

`datasetBuildId` は生成系コマンドで必須とし、snippet 系コマンドでは受け取らない。

`datasetBuildId` の生成は `tools/build.sh` が担当する。
`tools/build.sh` は次の 3 コマンドで入力集合ハッシュを求め、それらをさらに SHA-256 でハッシュして最終 `datasetBuildId` を作る。

- `metadata artist hash-inputs`
- `metadata tag hash-inputs`
- `musictl build hash-inputs`

frontend は `s2` 生成物を読み込む際、まず metadata を確認し、
次に参照整合性を検証する。
たとえば `clip.videoId -> videos.min.json`, `video.videoTags -> tags.min.json`,
`clip.liverIds -> livers.min.json` の参照切れを検出できるようにする。

検索 cursor は `datasetBuildId`, query, sort を使って再利用可否を判定する。

詳細は [`search/overview.md`](../search/overview.md) を参照。

## 頻度の考え方(まとめ)

- **高頻度**: `s0`(定義), `s1`(snippet), `s2`(min + search)
  - clips の追加 / 更新 / 同期が走ったタイミングに追従
- 入力集合ハッシュが変わらない限り、同じ `datasetBuildId` を再利用できる
