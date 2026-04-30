# cliplayer

YouTube上の動画/アーカイブから歌唱部分のみを連続的に再生するサイト。事前に動画の歌唱部分を定義しておき、ユーザーはそれらを連続的に再生することができる。高度な検索エンジンも提供。

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
