# cliplayer

## データと生成物

このリポジトリでは、以下の3系統のデータを扱う。

- **artist**: ライバー/公式チャンネル等のメタデータ
- **tag**: 動画・クリップに付与するタグ定義
- **clips**: 動画と、その中の歌唱クリップ定義

さらに、これらから用途別に派生生成物を作る。

- **フロント用 min データ**: 軽量・参照しやすい形式(主に `*.min.json`)
- **検索用インデックス**: 検索エンジン向け(生成コストが高い想定)
- **定義支援(snippet)ファイル**: 手入力を支援する VS Code snippet

### 重要な原則

- **source-of-truth(正)は常に「人が編集する定義ファイル」**に置く。
- **派生物は正から生成**する。派生物から別の派生物を作る(派生-from-派生)は避ける。
- **同じ入力から同じ出力が得られる**(再現可能)ようにする。
- **頻度の異なる派生物(例: min と検索インデックス)はステップを分離**し、生成タイミングも分ける。

## 処理ステップ

重要: s1, s2, s3はすべて**s0のみ**を入力とする。数字は処理の複雑さや頻度を示すもので、処理の順序を示すものではない。

### s0. 定義ファイルを編集する(手動)

最初に編集するのは、以下の「非 min」(正)データ。

- artist
  - `metadata/artist/data/livers.json`
  - `metadata/artist/data/official_channels.json`
- tag
  - `metadata/tag/data/tags.json`
- clips
  - `music/data/input/*.json` (追加・更新の作業用入力)
  - `music/data/music/YYYY/MM.json` (永続化される正データ)

### s1. 定義支援(snippet)を生成・更新する(頻繁)

目的: 手入力のコストを下げ、記述揺れを減らす。

- artist → `.vscode/music.code-snippets` を更新 (ライバーID候補など)
- tag → `.vscode/tags.code-snippets` を更新 (タグ候補など)

実行例:

- `metadata artist --operation generate_snippet`
- `metadata tag --operation generate_snippet`

原則: snippet は **編集対象(正データ)の補助**であり、正データの代替ではない。

### s2. フロント用 min データを生成する(頻繁)

目的: フロント(Angular)が軽量に読み込めるデータを提供する。

- artist
  - 出力先: `src/music/`
  - 出力例: `livers.min.json`, `channels.min.json`, `official_channels.min.json`, `livers_search_index.min.json`
  - 生成: `metadata artist ...` 相当
- tag
  - 出力先: `src/music/`
  - 出力例: `tags.min.json`
  - 生成: `metadata tag --step s2`
- clips
  - 出力先: `public/music/`
  - 出力例: `clips.min.json`, `videos.min.json`
  - 生成: `musictl add apply` / `musictl update apply` / `musictl sync` が min 生成まで行う

運用上の意図:

- `src/music/*` はフロントのビルドに含める(バンドルされる)用途
- `public/music/*` はフロントのビルドとは独立に配信できる(静的配信)用途

### s3. 検索インデックスを生成する(低頻度)

目的: 検索エンジン向けに、artist/tag/clips を統合した検索用インデックスを作る。

要件:

- **min ではない正データ(s0)を読み取って生成**する
  - artist: `metadata/artist/data/*.json`
  - tag: `metadata/tag/data/tags.json`
  - clips: `music/data/music/**`
- 生成物は、フロント用 min (s2) とは **別アーティファクト**として扱う
- **生成頻度は s2 より遅くてよい**(生成コストが高い想定)

設計:

- s3 は独立ジョブ/ワークフローに分ける
  - 例: 手動トリガー / スケジュール(週1など)
- s3 の実行可否は **入力のハッシュ**で判定してスキップできるようにする
  - `livers.json`, `official_channels.json`, `tags.json`, `music/data/music/**` をまとめた manifest を作り、前回と同一なら rebuild しない
- s3 の出力には、どの入力から生成したかを追跡できるように **manifest(メタ情報)** を同梱する
- 生成したインデックスは`public/`へ出力

## 頻度の考え方(まとめ)

- **高頻度**: 0(定義), s1(snippet), s2(min)
  - clips の追加/更新/同期が走ったタイミングに追従
- **低頻度**: s3(検索インデックス)
  - 週次/リリース/手動など
  - 入力ハッシュ一致ならスキップ可能にして「毎回組む」コストを避ける
