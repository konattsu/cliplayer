# cliplayer

YouTube 上の動画やアーカイブから歌唱 clip を検索し、連続再生するための静的サイト。
フロントエンドは Angular、検索やデータ生成は Rust ツール群で支える。

## Overview

- Frontend: Angular による静的サイト
- Rust tools: データ管理、変換、検索インデックス生成
- Search: 事前生成した index をブラウザ側で検索

## Read This First

- サービス全体の流れ: [`docs/data_flow.md`](docs/data_flow.md)
- フロントエンドの画面構成と責務: [`docs/site_design.md`](docs/site_design.md)
- 検索システムの全体像: [`search/overview.md`](search/overview.md)
- 検索エンジンの内部設計: [`search/engine/design.md`](search/engine/design.md)
- 検索インデックスのバイナリ形式: [`search/binary_schema.md`](search/binary_schema.md)
- WASM 境界: [`search/engine-wasm/usage.md`](search/engine-wasm/usage.md)

## Data Format Docs

frontend が読む生成済みデータの形式:

- artist データ: [`metadata/artist/format.md`](metadata/artist/format.md)
- tag データ: [`metadata/tag/format.md`](metadata/tag/format.md)
- music データ: [`music/data/format.md`](music/data/format.md)

## Rust Workspace

- `cmn_rs`: 共通ユーティリティ
- `metadata`
  - `artist`: アーティスト管理
  - `tag`: タグ管理
- `music/musictl`: 音楽データ管理
- `search`
  - `index-core`: 検索インデックス schema
  - `index-builder`: 検索インデックス生成
  - `engine`: 検索エンジン本体
  - `engine-wasm`: ブラウザ向け WASM 境界

### Dependency

- `musictl` -> `metadata`
- `search/engine` -> `search/index-core`
- `search/index-builder` -> `search/index-core`, `metadata`, `musictl`
- `search/engine-wasm` -> `search/engine`, `search/index-core`
