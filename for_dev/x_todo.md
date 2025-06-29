# foo

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
    "channelId": "UC1111111111111111111111",
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
    "title": "おねがいダーリン /栞葉るり Cover",
    "artists": ["ruri-shioriha"],
    "hasVideo": false,
    "videoId": "ZeFvqdvutb4",
    "uuid": "123e4567-e89b-12d3-a456-426614174000",
    "startTime": "PT1M10S",
    "endTime": "PT4M21S"
  },
  {
    "title": "命に嫌われている。 /栞葉るり Cover",
    "artists": ["ruri-shioriha"],
    "hasVideo": false,
    "videoId": "ZeFvqdvutb4",
    "uuid": "123e4567-e89b-12d3-a456-426614174001",
    "startTime": "PT7M12S",
    "endTime": "PT11M34S"
  }
]

// videos.json
{
  "ZeFvqdvutb4": {
    "publishedAt": "2023-12-10",
    "isPrivate": false,
    "title": "【収益化記念】イ　ン　タ　ー　ネ　ッ　ト　カ　ラ　オ　ケ　T　I　M　E【栞葉るり/にじさんじ】",
    "handleName": "ShiorihaRuri",
    "modifiedAt": "2023-12-10T12:00:00Z",
  }
}
```
