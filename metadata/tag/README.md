# tagctl

アーティスト(ライバー)情報を管理するツール

## cmd

- 引数に基づいて, 以下の処理を行う
  - validate
  - `code snippet`ファイルの更新
  - フロント用のminifiedデータを生成しファイルに出力

## data format

コードブロック内で, 例示用にコメントアウトを用いたいので`jsonc`を指定しているが, 実際は全て`json`ファイルである.

### write manually

- [`tags.json`](./data/tags.json)

タグの情報を管理

```jsonc
{
  "karaoke": {
    /// 日本語
    "ja": "歌枠",
    /// 英語
    "en": "karaoke",
    /// 整数id. 永続的でなく, 各ビルド時に一意に識別出来たらok. 検索インデックスで用いる
    "intId": 0
    /// このタグを持つクリップの再生をブロックするかどうか. trueならブロック. falseがデフォルトで記述しなくていい
    /// "blocked": false
  },
  "3d-debut": {
    "ja": "3Dお披露目",
    "en": "3D Debut",
    "blocked": true,
    "intId": 4
  },
}
```

## auto generated

- `tags.min.json

上の`tags.json`をそのままminifyしたもの
