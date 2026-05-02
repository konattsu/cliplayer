# cliplayer

YouTube上の動画/アーカイブから歌唱部分のみを連続的に再生するサイト。事前に動画の歌唱部分を定義しておき、ユーザーはそれらを連続的に再生することができる。高度な検索エンジンも提供。

## Rust Tools

### Overview

- cmn_rs: 共通のユーティリティ. どのクレートから依存されてよい
- metadata:
  - artist: アーティスト管理
  - tag: タグ管理
- music/musictl: 音楽管理
- search: (開発中)
  - index-core: 検索インデックスのスキーマ定義
  - index-builder: 検索インデックスの構築
  - engine: 検索エンジン

### Dependency

musictl -> metadata
search/engine -> search/index-core
search/index-builder -> metadata, musictl
