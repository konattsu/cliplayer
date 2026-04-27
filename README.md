# cliplayer

## Rust Tools

### Overview

- cmn_rs: 共通のユーティリティ
- metadata:
  - artist: アーティスト管理
  - tag: タグ管理
- music/musictl: 音楽管理
- search: (開発中)
  - engine: 検索エンジン
  - index_fmt: 検索インデックスのフォーマット

### Dependency

musictl -> metadata
search/engine -> metadata, index_fmt
search/index_fmt -> metadata, musictl
