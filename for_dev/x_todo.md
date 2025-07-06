# foo

タグ関連の設計考える
動画に対して付与したものとクリップごとのものをどうするか...
2次生成で分離, clipsに動画に対してのタグも持たせる <-ありでは?

ユニット名などは名前から算出できるけど処理速度が低下しそう

...

TODO 絶対見て

- このファイルTODO目的に直す
- rust_music_data: rename to music_data
- rust_music_data/data/README.md, このファイルの内容を移動する
- そのあとREADME.md綺麗にする
- rust_music_data/artists/, fn generate以外をartistsまで(下から見て)の公開範囲に設定pub(in crate::artists(多分こんな形式))
- artistsで生成したファイルのハッシュ値算出してどっかに保存 <--いらんかも
- rust_music_data/artists/README.mdつくる

...

手動で書いてプログラムに食わせる

初回追加時は入力ファイル(input.json)を作成し, プルリク出してマージされると一次生成に追加され, 自動的に(input.json)は消える

```json
[
  {
    "video_id": "ZeFvqdvutb4",
    // 箱外のチャンネルでは任意の名称を付与する, 箱内だとkeyは無くていい(空文字列はno)
    "uploaderName ": "(例示用)",
    "videoTags": ["karaoke", "2d"],
    "clips": [
      {
        "songTitle": "おねがいダーリン",
        "songTitleJah": "おねがいだーりん",
        "artists": ["ruri-shioriha"],
        "externalArtists": ["しぐれうい"],
        "isClipped": false,
        "startTime": "PT1M10S",
        "endTime": "PT4M21S",
        "clipTags": ["がぶっく"]
      },
      {
        "songTitle": "命に嫌われている。",
        // 読まない記号などは省略, 但し例えば"+"を"プラス"と発音する場合は"ぷらす"と書く
        "songTitleJah": "いのちにきらわれている",
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
// music.json
[
  {
    "videoId": "ZeFvqdvutb4",
    "title": "【収益化記念】イ　ン　タ　ー　ネ　ッ　ト　カ　ラ　オ　ケ　T　I　M　E【栞葉るり/にじさんじ】",
    "channelId": "UC7_MFM9b8hp5kuTSpa8WyOQ",
    "uploaderName": "(例示用)",
    "publishedAt": "2023-12-10Z21:00:00Z",
    "modifiedAt": "2025-05-10T12:00:00Z",
    "duration": "PT59M22S",
    "privacyStatus": "public",
    "embeddable": true,
    "videoTags": ["karaoke", "2d"],
    "clips": [
      {
        "songTitle": "おねがいダーリン",
        "songTitleJah": "おねがいだーりん",
        "artists": ["ruri-shioriha"],
        "externalArtists": ["しぐれうい"],
        "isClipped": false,
        "startTime": "PT1M10S",
        "endTime": "PT4M21S",
        "uuid": "0197bbd1-039e-77f6-9112-519561b61f9e",
        "clipTags": ["がぶっく"],
        // 今は未実装
        "volumePercent": 50,
      },
      {
        "songTitle": "命に嫌われている。",
        "songTitleJah": "いのちにきらわれている",
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

二次生成, public以下に配置, min化

```json
// videos.min.json
// 一次生成をそのままmin化

// フロント側で内部的にuuidをkeyにしたHashMapを作成して, uuidからO(1)で色々な情報を引けるようにもする
[
  {
    "videoId": "ZeFvqdvutb4",
    "title": "【収益化記念】イ　ン　タ　ー　ネ　ッ　ト　カ　ラ　オ　ケ　T　I　M　E【栞葉るり/にじさんじ】",
    "channelId": "UC7_MFM9b8hp5kuTSpa8WyOQ",
    "uploaderName": "(例示用)",
    "publishedAt": "2023-12-10Z21:00:00Z",
    "modifiedAt": "2025-05-10T12:00:00Z",
    "duration": "PT59M22S",
    "privacyStatus": "public",
    "embeddable": true,
    "videoTags": ["karaoke", "2d"],
    "clips": [
      {
        "songTitle": "おねがいダーリン",
        "songTitleJah": "おねがいだーりん",
        "artists": ["ruri-shioriha"],
        "externalArtists": ["しぐれうい"],
        "isClipped": false,
        "startTime": "PT1M10S",
        "endTime": "PT4M21S",
        "uuid": "0197bbd1-039e-77f6-9112-519561b61f9e",
        "clipTags": ["がぶっく"]
      },
      {
        "songTitle": "命に嫌われている。",
        "songTitleJah": "いのちにきらわれている",
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

```json
// flat_clips.min.json
[
  {
    "uuid": "0197bbd1-039e-77f6-9112-519561b61f9e",
    "songTitle": "おねがいダーリン",
    "songTitleJah": "おねがいだーりん",
    "artists": ["ruri-shioriha"],
    "externalArtists": ["しぐれうい"],
    "clipTags": ["がぶっく"],
    "startTime": "PT1M10S",
    "endTime": "PT4M21S"
    // volumePercentは含めない
  },
  {
    "uuid": "0197bbd1-039e-7b67-8c92-6f557c60f187",
    "songTitle": "命に嫌われている。",
    "songTitleJah": "いのちにきらわれている",
    "artists": ["ruri-shioriha"],
    "startTime": "PT7M12S",
    "endTime": "PT11M34S"
  }
]
```

- 動画情報のみで検索したい:
  - videos.jsonのみ使用
- クリップ情報のみで検索したい:
  - clip_search_index.jsonのみ使用
- 動画情報とクリップ情報の両方で検索したい:
  1. videos.jsonで動画を検索し, 適切なvideoIdを取得
  2. clip_search_index.jsonでそのvideoIdに紐づくクリップと任意の条件を満たすcilpを検索

...

...

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
