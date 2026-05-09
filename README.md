# cliplayer

YouTube上の動画/アーカイブから歌唱部分のみを連続的に再生する静的サイト。事前に動画の歌唱部分を定義しておき、ユーザーはそれらを連続的に再生することができる。高度な検索エンジンも提供。

## Frontend

- angularで実装
- 静的サイト

## Rust Tools

データ管理, 加工と検索エンジンの実装に使用。

### Overview

- cmn_rs: 共通のユーティリティ. どのクレートからも依存されてよい
- metadata:
  - artist: アーティスト管理
  - tag: タグ管理
- music/musictl: 音楽管理
- search:
  - index-core: 検索インデックスのスキーマ定義
  - index-builder: 検索インデックスの構築
  - engine: 検索エンジン
  - engine-wasm: wasmで動く検索エンジン

### Dependency

- musictl -> metadata
- search/engine -> search/index-core
- search/index-builder -> search/index-core, metadata, musictl
- search/engine-wasm -> search/engine, search/index-core

## Important Project Docs

This repository also includes several companion documents for design and data flow:

- `docs/data_flow.md` — overall data pipeline and generation stages (s0/s1/s2/s3)
- `docs/site_design.md` — frontend site design assumptions and page flow
- `search/binary_schema.md` — search index binary format and section layout
- `search/engine/design.md` — search engine implementation design
- `search/overview.md` — search system overview and responsibilities

## Frontend min file schemas

The frontend consumes generated minified JSON under `public/music/`. The structure and format of those frontend-facing min files are documented in:

- `metadata/artist/format.md`
- `metadata/tag/format.md`
- `music/data/format.md`
