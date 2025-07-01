# foo

タグ関連の設計考える
動画に対して付与したものとクリップごとのものをどうするか...
2次生成で分離, clipsに動画に対してのタグも持たせる <-ありでは?

ユニット名などは名前から算出できるけど処理速度が低下しそう

...

...

手動で書いてプログラムに食わせる

初回追加時は入力ファイル(input.json)を作成し, プルリク出してマージされると一次生成に追加され, 自動的に(input.json)は消える

```json
[
  {
    "video_id": "ZeFvqdvutb4",
    "tags": ["karaoke", "2d"],
    "clips": [
      {
        "songTitle": "おねがいダーリン",
        "artists": ["ruri-shioriha"],
        "externalArtists": ["しぐれうい"],
        "isClipped": false,
        "startTime": "PT1M10S",
        "endTime": "PT4M21S",
        "tags": ["がぶっく"]
      },
      {
        "songTitle": "命に嫌われている。",
        "artists": ["ruri-shioriha"],
        "isClipped": false,
        "startTime": "PT7M12S",
        "endTime": "PT11M34S"
      }
    ]
  }
]
```

一次生成, apiを使って生成

```json
[
  {
    "videoId": "ZeFvqdvutb4",
    "title": "【収益化記念】イ　ン　タ　ー　ネ　ッ　ト　カ　ラ　オ　ケ　T　I　M　E【栞葉るり/にじさんじ】",
    "channelId": "UC7_MFM9b8hp5kuTSpa8WyOQ",
    "publishedAt": "2023-12-10Z21:00:00Z",
    "modifiedAt": "2025-05-10T12:00:00Z",
    "privacyStatus": "public",
    "embeddable": true,
    "tags": ["karaoke", "2d"],
    "clips": [
      {
        "songTitle": "おねがいダーリン",
        "artists": ["ruri-shioriha"],
        "externalArtists": ["しぐれうい"],
        "isClipped": false,
        "startTime": "PT1M10S",
        "endTime": "PT4M21S",
        "uuid": "0197bbd1-039e-77f6-9112-519561b61f9e",
        "tags": ["がぶっく"]
      },
      {
        "songTitle": "命に嫌われている。",
        "artists": ["ruri-shioriha"],
        "isClipped": false,
        "startTime": "PT7M12S",
        "endTime": "PT11M34S",
        "uuid": "0197bbd1-039e-7b67-8c92-6f557c60f187"
      }
    ]
  }
]
```

二次生成, public以下に配置

```json
// 実際はmin化
// clips.json
[
  {
    "songTitle": "おねがいダーリン",
    "artists": ["ruri-shioriha"],
    "externalArtists": ["しぐれうい"],
    "isClipped": false,
    "startTime": "PT1M10S",
    "endTime": "PT4M21S",
    "uuid": "0197bbd1-039e-77f6-9112-519561b61f9e",
    "tags": ["がぶっく"],
    "videoId": "ZeFvqdvutb4"
  },
  {
    "songTitle": "命に嫌われている。",
    "artists": ["ruri-shioriha"],
    "isClipped": false,
    "startTime": "PT7M12S",
    "endTime": "PT11M34S",
    "uuid": "0197bbd1-039e-7b67-8c92-6f557c60f187",
    "videoId": "ZeFvqdvutb4"
  }
]

// videos.json
{
  "ZeFvqdvutb4": {
    "publishedAt": "2023-12-10Z21:00:00Z",
    "privacyStatus": "public",
    "title": "【収益化記念】イ　ン　タ　ー　ネ　ッ　ト　カ　ラ　オ　ケ　T　I　M　E【栞葉るり/にじさんじ】",
    "embeddable": true,
    "tags": ["karaoke", "2d"],
    "modifiedAt": "2025-05-10T12:00:00Z",
  }
}
// どっちも必要かも
[
  {
    "videoId": "ZeFvqdvutb4",
    "publishedAt": "2023-12-10Z21:00:00Z",
    "privacyStatus": "public",
    "title": "【収益化記念】イ　ン　タ　ー　ネ　ッ　ト　カ　ラ　オ　ケ　T　I　M　E【栞葉るり/にじさんじ】",
    "embeddable": true,
    "tags": ["karaoke", "2d"],
    "modifiedAt": "2025-05-10T12:00:00Z",
  }
]

```

videos.json
modifiedAtで変更検出したっていい

clipsをuuidでO(1)にしてvideoIdを引けるように

フロント側での要件(検索, 一括表示)を満たすのに最適になるように考える
逆像法みたいに

フロント側で何がしたい

- 動画の一覧表示: 一次生成そのままでもいける
  - 動画押すと下に"ぬっ"てクリップが展開される
- 検索, フィルタ
  - ライバー名で
    - チャンネル名: 一次生成のままでいける
    - そのクリップにある名: clipのflat便利あると便利
  - タグで: clipのflat便利あると便利
  - 楽曲名(優先度低い)

```json
// 二次生成, これ用改良だがめっちゃいいかも
{
  "videos": [
    {
      "videoId": "ZeFvqdvutb4",
      "title": "【収益化記念】イ　ン　タ　ー　ネ　ッ　ト　カ　ラ　オ　ケ　T　I　M　E【栞葉るり/にじさんじ】",
      "channelId": "UC7_MFM9b8hp5kuTSpa8WyOQ",
      "publishedAt": "2023-12-10Z21:00:00Z",
      "modifiedAt": "2025-05-10T12:00:00Z",
      "privacyStatus": "public",
      "embeddable": true,
      "tags": ["karaoke", "2d"],
      "clips": [
        {
          "uuid": "0197bbd1-039e-77f6-9112-519561b61f9e",
          "songTitle": "おねがいダーリン",
          "artists": ["ruri-shioriha"],
          "externalArtists": ["しぐれうい"],
          "tags": ["がぶっく"],
          "startTimeSec": 70,
          "endTimeSec": 261,
          "startTimeFormatted": "01:10",
          "endTimeFormatted": "04:21"
        },
        {
          "uuid": "0197bbd1-039e-7b67-8c92-6f557c60f187",
          "songTitle": "命に嫌われている。",
          "artists": ["ruri-shioriha"],
          "externalArtists": [],
          "tags": [],
          "startTimeSec": 432,
          "endTimeSec": 694,
          "startTimeFormatted": "07:12",
          "endTimeFormatted": "11:34"
        }
      ]
    }
  ],
  "clipSearchIndex": [
    {
      "videoId": "ZeFvqdvutb4",
      "videoTitle": "【収益化記念】イ　ン　タ　ー　ネ　ッ　ト　カ　ラ　オ　ケ　T　I　M　E【栞葉るり/にじさんじ】",
      "channelId": "UC7_MFM9b8hp5kuTSpa8WyOQ",
      "clipUuid": "0197bbd1-039e-77f6-9112-519561b61f9e",
      "songTitle": "おねがいダーリン",
      "artists": ["ruri-shioriha"],
      "externalArtists": ["しぐれうい"],
      "tags": ["がぶっく"],
      "startTimeSec": 70,
      "endTimeSec": 261
    },
    {
      "videoId": "ZeFvqdvutb4",
      "videoTitle": "【収益化記念】イ　ン　タ　ー　ネ　ッ　ト　カ　ラ　オ　ケ　T　I　M　E【栞葉るり/にじさんじ】",
      "channelId": "UC7_MFM9b8hp5kuTSpa8WyOQ",
      "clipUuid": "0197bbd1-039e-7b67-8c92-6f557c60f187",
      "songTitle": "命に嫌われている。",
      "artists": ["ruri-shioriha"],
      "externalArtists": [],
      "tags": [],
      "startTimeSec": 432,
      "endTimeSec": 694
    }
  ]
}

```
