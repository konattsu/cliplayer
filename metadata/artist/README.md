# artistctl

アーティスト(ライバー)情報を管理するツール

## cmd

- `artistctl`: アーティスト情報を基に以下の処理を行う
  - validate
  - `code snippet`ファイルの更新
  - 検索インデックス, フロント用のminifiedデータを生成しファイルに出力

## data format

コードブロック内で, 例示用にコメントアウトを用いたいので`jsonc`を指定しているが, 実際は全て`json`ファイルである.

### write manually

自分で頑張って書くファイル.

- [`livers.json`](./data/livers.json)

ライバーの情報を管理

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
    // 卒業したかどうか. falseは記述しなくていい
    // "isGraduated": false,
    // 整数id. 永続的でなく, 各ビルド時に一意に識別出来たらok. 検索インデックスで用いる
    "intId": 0,
  },
}
```

- [`official_channels.json`](./data/official_channels.json)

公式チャンネルの情報を管理

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

### auto generated

自動で生成されるファイル. 主にfrontendで使いたいので必要に応じてminifiedなどの処理をして出力する.

- `livers_search_index.min.json`

ライバー名の検索インデックス

```jsonc
[
  { "key": "栞葉るり", "artistId": "ruri-shioriha" },
  { "key": "しおりはるり", "artistId": "ruri-shioriha" },
  { "key": "Shioriha Ruri", "artistId": "ruri-shioriha" },
  { "key": "倉持めると", "artistId": "meruto-kuramochi" },
  { "key": "くらもちめると", "artistId": "meruto-kuramochi" },
  { "key": "Kuramochi Meruto", "artistId": "meruto-kuramochi" },
  // aliasは表示の優先度を下げたいので, aliasかどうかを判断できるようにしておく
  { "key": "めるち", "artistId": "meruto-kuramochi", "isAlias": true },
]
```

- `livers.min.json`

ライバーの情報をminifiedしたもの

```jsonc
{
  "ruri-shioriha": {
    "ja": "栞葉るり",
    "jah": "しおりはるり",
    "en": "Shioriha Ruri",
    "color": "2887FF",
    // "isGraduated": false  // 卒業したかどうか. falseは記述されない
    "intId": 133,
  },
  "meruto-kuramochi": {
    "ja": "倉持めると",
    "jah": "くらもちめると",
    "en": "Kuramochi Meruto",
    "color": "EB4682",
    "intId": 123,
  },
}
```

- `channels.min.json`

YouTubeチャンネルIDをキーとし, 値がライバー, もしくは公式チャンネルIDのマップ

```jsonc
{
  // YouTubeチャンネルID
  "UC7_MFM9b8hp5kuTSpa8WyOQ": {
    "id": "ruri-shioriha", // ライバーID (liverId)
    "kind": "liver", // enum: "liver" | "official"
  },
  "UC-JSeFfovhNsEhftt1WHMvg": {
    "id": "nijisaji-en-official", // 公式チャンネルID (officialChannelId)
    "kind": "official",
  },
}
```

- `official_channels.min.json`

公式チャンネルの情報をminifiedしたもの

```jsonc
{
  "nijisanji-official": {
    "ja": "にじさんじ公式",
    "jah": "にじさんじこうしき",
    "en": "Nijisanji Official",
    "channelId": "UCX7YkU9nEeaoZbkVLVajcMg",
  },
}
```
