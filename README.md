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

## TODO

`/music`のrustツールである`musictl`に対して、テストを充実させたいです. 構想を練っています. 単体テストは少し書いていますが, 結合テスト, e2eテストも書きたいです. e2eは以下の構想ですが, 変えてもらって全く問題ないです. 依存の`metadata`にもテストが必要かもしれません. 構想が出来たら, ある程度詳細にmarkdownに出力してください.

- e2e
  - testcontainers-rs + mockserver(wiremock)
  - endpointのconstを #[\cfg(test)]とか#[\cfg(features ...)]で切り替え
  - constでなくlazy_static使う <- portが多分動的なのでenvぐらいから引っ張ってくる
  - tempfileはもちもちもちもちもちろん使う
