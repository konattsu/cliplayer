# artistctl

アーティスト(ライバー)情報を管理するツール

## cmd

- `artistctl`: アーティスト情報を基に以下の処理を行う
  - validate
  - `code snippet`ファイルの更新
  - 検索インデックス, フロント用のminifiedデータを生成しファイルに出力

## data format

表示は`jsonc`形式だが, 実際は全て`json`を使う.

- [`livers.json`](./data/livers.json)

```jsonc
{
  // liverId. 公式サイトのタレント一覧のパスパラメータ(talents/l/%s), `%s`の部分
  "mito-tsukino": {
    // 日本語. 公式サイト参照
    "ja": "月ノ美兎",
    // 平仮名. 公式サイト参照
    "jah": "つきのみと",
    // 英語. 公式サイト参照
    "en": "Tsukino Mito",
    // エイリアス. 現在は平仮名のみ用いる
    "aliases": ["いいんちょう"],
    // YouTubeのチャンネルID
    "channelId": "UCD-miitqNY3nyukJ4Fnf4_A",
    // モチーフカラー. 公式サイト, wiki参照
    "color": "E43F3B",
    // 整数id. 永続的でなく, 各ビルド時に一意に識別出来たらok. 検索インデックスで用いる
    "intId": 0,
  },
}
```

- [`official_channels.json`](./data/official_channels.json)

```jsonc
{
  // officialChannelId. 適切な識別子を考えて付与
  "nijisanji-official": {
    // 日本語
    "ja": "にじさんじ公式",
    // 平仮名
    "jah": "にじさんじこうしき",
    // 英語
    "en": "Nijisanji Official",
    // エイリアス. 現在は平仮名のみ用いる
    "aliases": [],
    // YouTubeのチャンネルID
    "channelId": "UCX7YkU9nEeaoZbkVLVajcMg",
    // 整数id. 上と同様に永続的でなく, 各ビルド時に一意に識別出来たらok. 検索インデックスで用いる
    "intId": 950,
  },
}
```
