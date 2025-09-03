# データの扱われ方

preview用に`jsonc`使ってるが実際は全て`json`

## 動画のデータ

public/に配置. フロントのビルドとは関係ない

- 手動で記述
  - `input/foo.json`: 動画の情報を一時的に記述
- 一次生成
  - `(年)/(月).json`: 年月ごとに動画の情報をまとめたもの
- 二次生成, 頻繁に変更されるのでpublic/に配置
  - `clips.min.json`: クリップの情報をまとめたもの
  - `videos.min.json`: 動画の情報をまとめたもの

### `input/foo.json`

新規動画(楽曲)データを追加する

[スキーマ](/tools/music_data.schema.json)を[設定](/.vscode/settings.json)済み

```jsonc
[
  {
    "videoId": "ZeFvqdvutb4",
    "uploaderName": "(例示用)",
    "videoTags": ["karaoke"],
    "clips": [
      {
        "songTitle": "おねがいダーリン",
        "artists": ["ruri-shioriha"],
        "externalArtists": ["(例示用)"],
        "clippedVideoId": "(例示用)",
        "startTime": "PT1M10S",
        "endTime": "PT4M21S",
        "clipTags": ["(例示用)"]
      },
      {
        "songTitle": "命に嫌われている。",
        "artists": ["ruri-shioriha"],
        "startTime": "PT7M12S",
        "endTime": "PT11M34S"
      }
    ]
  }
]
```

フィールドについて

- `videoId`: YouTubeの動画ID
- `uploaderName`: 動画のアップロード者名. 箱外のチャンネルのときのみ任意の適切な名称を付与する. 空文字列は絶対にno
- `videoTags`: 動画のタグ

  - e.g. "karaoke", "3d", "sitr-nagoya"
  - 命名規則
    - lower kebab-case
    - 子要素が親要素のタグを含む場合は親要素のタグも付与
      - e.g. 3dお披露目であれば `3d`, `3d-debut`の二つを付与

- `clips`: 動画のクリップ情報の配列
  - `songTitle`: 楽曲のタイトル
    - e.g. "おねがいダーリン", "命に嫌われている。"
  - `artists`: 楽曲の歌唱者のアーティストID(箱内)の配列
  - `externalArtists`: 楽曲の外部アーティスト名(箱外)の配列
  - `clippedVideoId`: このクリップが公式で切り抜かれているときはその動画id, 切り抜かれてないとnull
    - e.g. [この配信](https://youtu.be/6gKIA3_ihCY?t=1h4m20s)で最後から2番目に歌われている曲は, ご本人さんのチャンネルに[動画](https://youtu.be/NNVQm3qtkOY)として投稿されているので `NNVQm3qtkOY`を指定
  - `startTime`: クリップの開始時間. 音(イントロ||声)が流れた時間を指定
    - e.g. 1分30.4秒など中途半端であれば0.4秒早めて`PT1M30S`を指定.
  - `endTime`: クリップの終了時間. 同上
    - e.g. 3分24.1秒であれば0.9秒遅くして`PT3M25S`を指定
  - `clipTags`: クリップのタグ. 現在のところ未使用

### `(年)/(月).json`

実際に動画の情報を格納するファイル

`publishedAt`の日付を基にフォルダを決定. 先ほどの日付(`publishedAt`)の古いほうが先頭になるように記述. また, 同じ動画(videoIdが同一)は同時に存在しないことを保証

```jsonc
[
  {
    "videoId": "ZeFvqdvutb4",
    "title": "【収益化記念】イ　ン　タ　ー　ネ　ッ　ト　カ　ラ　オ　ケ　T　I　M　E【栞葉るり/にじさんじ】",
    "channelId": "UC7_MFM9b8hp5kuTSpa8WyOQ",
    "uploaderName": "(例示用)",
    "publishedAt": "2023-12-10Z21:00:00Z",
    "syncedAt": "2025-05-10T12:00:00Z",
    "duration": "PT59M22S",
    "privacyStatus": "public",
    "embeddable": true,
    "videoTags": ["karaoke", "2d"],
    "clips": [
      {
        "songTitle": "おねがいダーリン",
        "artists": ["ruri-shioriha"],
        "externalArtists": ["(例示用)"],
        "startTime": "PT1M10S",
        "endTime": "PT4M21S",
        // uuid version 4
        "uuid": "d5cb8a6b-fb40-424d-9079-c62bd76b92a5",
        "clipTags": ["(例示用)"],
        "clippedVideoId": "(例示用)",
        // 今は未実装. 実装しても, actionsで直接付与できるものではなくローカルからのPRになりそう
        "volumePercent": 50
      },
      {
        "songTitle": "命に嫌われている。",
        "artists": ["ruri-shioriha"],
        "startTime": "PT7M12S",
        "endTime": "PT11M34S",
        "uuid": "6af3a9fb-05ab-4e53-8cdf-9e63869c4246"
      }
    ]
  }
]
```

### `clips.min.json`

主にクリップの情報

```jsonc
{
  "d5cb8a6b-fb40-424d-9079-c62bd76b92a5": {
    // このクリップが含まれる動画id
    "videoId": "ZeFvqdvutb4",
    // secsに変換
    "startTimeSecs": 70,
    "endTimeSecs": 261,

    // 後は一緒
    "songTitle": "おねがいダーリン",
    "artists": ["ruri-shioriha"],
    "externalArtists": ["(例示用)"],
    "clippedVideoId": "(例示用)",
    "clipTags": ["(例示用)"],
    "volumePercent": 50
  },
  "6af3a9fb-05ab-4e53-8cdf-9e63869c4246": {
    "videoId": "ZeFvqdvutb4",
    "startTimeSecs": 432,
    "endTimeSecs": 694,

    "songTitle": "命に嫌われている。",
    "artists": ["ruri-shioriha"],
  }
}
```

### `videos.min.json`

主に動画情報

```jsonc
{
  "ZeFvqdvutb4": {
    // 順番は保証しない
    "clipUuids": [
      "d5cb8a6b-fb40-424d-9079-c62bd76b92a5",
      "6af3a9fb-05ab-4e53-8cdf-9e63869c4246"
    ],
    // secsに変換
    "durationSecs": "3562",

    // 後は一緒
    "title": "【収益化記念】イ　ン　タ　ー　ネ　ッ　ト　カ　ラ　オ　ケ　T　I　M　E【栞葉るり/にじさんじ】",
    "channelId": "UC7_MFM9b8hp5kuTSpa8WyOQ",
    "uploaderName": "(例示用)",
    "publishedAt": "2023-12-10Z21:00:00Z",
    "syncedAt": "2025-05-10T12:00:00Z",
    "privacyStatus": "public",
    "embeddable": true,
    "videoTags": ["karaoke", "2d"]
  }
}
```

### `clip_docs.min.json`

```jsonc
[
  {
    "docId": 0,  // auto-inc
    "videoId": 0, // 対応表は別で保持
    "artistIntIds": [6, 42],  // artists_data.jsonを参照して整数化
    "tagIds": [0, 1],  // 対応表は別で保持
    "channelIntId": 0, // Option<int>, artists_data.jsonを参照して整数化
    "publishedAtSec":  0  // unix timestamp
  }
]
```

### `video_id_record.min.json`

```jsonc
{
  "ZeFvqdvutb4": 0,  // auto-inc
  "6gKIA3_ihCY": 1
}
```

### `tag_id_record.min.json`

```jsonc
{
  "karaoke": 0,  // auto-inc
  "3d": 1,
  "sitr-nagoya": 2
}
```

## 動画以外のデータ

フロントのビルド時に埋め込み

- 手動で記述
  - `artists_data.json`: アーティストの全ての情報
- 自動で生成, 変更頻度が低いので`src/`に配置
  - `artist_search_index.min.json`: アーティスト名の検索インデックス
  - `artists.min.json`: アーティストの一部の情報をflattenしてまとめたもの
  - `channels.min.json`: チャンネルidからアーティストidの対応

### `artists_data.json`

アーティスト情報を一元管理

非公式wikiの左側の順番で記述, セグメント分離されている場合は空行を挟む.

[スキーマ](/tools/artists_data.schema.json)を[設定](/.vscode/settings.json)済み

一旦jpのみ

```jsonc
{
  "ruri-shioriha": {
    // 日本語での名前
    "ja": "栞葉るり",
    // 日本語の平仮名表記
    "jah": "しおりはるり",
    // 英語(ローマ字)での表記
    "en": "Shioriha Ruri",
    // 表記ゆれ, 呼称を許容するため. 平仮名のみ
    "aliases": [],
    // YouTubeチャンネルID. ハンドルネームではない
    "channelId": "UC7_MFM9b8hp5kuTSpa8WyOQ",
    // モチーフカラー. 公式ページか非公式wiki参照
    "color": "2887FF",
    // "isGraduated": false  // 卒業済みかどうか, falseは記述しなくてよい
    "intId": 133  // 整数idも欲しいので手動で付与. 重複しないように
  },
  "meruto-kuramochi": {
    "ja": "倉持めると",
    "jah": "くらもちめると",
    "aliases": ["めるち"],
    "en": "Kuramochi Meruto",
    "channelId": "UCiA-trSZfB0i92V_-dyDqBw",
    "color": "EB4682",
    "intId": 123
  }
}
```

### `artist_search_index.min.json`

アーティスト名の検索インデックス

`artist_data.json`にある, `ja`, `jah`, `en`, `aliases`全ての値をキーにし, アーティストidを値にした配列. アーティスト名からアーティストidをO(n)で抽出できる

```jsonc
[
  { "key": "栞葉るり", "artistId": "ruri-shioriha" },
  { "key": "しおりはるり", "artistId": "ruri-shioriha" },
  { "key": "Shioriha Ruri", "artistId": "ruri-shioriha" },
  { "key": "倉持めると", "artistId": "meruto-kuramochi" },
  { "key": "くらもちめると", "artistId": "meruto-kuramochi" },
  { "key": "Kuramochi Meruto", "artistId": "meruto-kuramochi" },
  // aliasは表示の優先度を下げたいので, aliasかどうかを判断できるようにしておく
  { "key": "めるち", "artistId": "meruto-kuramochi", "isAlias": true }
]
```

### `artists.min.json`

アーティストの一部の情報をまとめたもの

`artists_data.json`の一部を抜粋して, フロントエンドでの表示に必要な情報のみを含む.

```jsonc
{
  "ruri-shioriha": {
    "ja": "栞葉るり",
    "jah": "しおりはるり",
    "en": "Shioriha Ruri",
    "color": "2887FF",
    // "isGraduated": false  // 卒業済みかどうか, falseは記述されない
    "intId": 133
  },
  "meruto-kuramochi": {
    "ja": "倉持めると",
    "jah": "くらもちめると",
    "en": "Kuramochi Meruto",
    "color": "EB4682",
    "intId": 123
  }
}
```

### `channels.min.json`

YouTubeチャンネルIDとアーティストidの対応hashmap

O(1)で引くため, 辞書形式を使用.

```jsonc
{
  "UC7_MFM9b8hp5kuTSpa8WyOQ": {
    "artistId": "ruri-shioriha",
    "intId": 133
  },
  "UCiA-trSZfB0i92V_-dyDqBw": {
    "artistId": "meruto-kuramochi",
    "intId": 123
  }
}
```
