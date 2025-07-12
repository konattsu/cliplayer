# データの扱われ方

## 動画のデータ

- 手動で記述
  - `input.json`: 動画の情報を一時的に記述
- 一次生成
  - `(年)/(月).json`: 年月ごとに動画の情報をまとめたもの
- 二次生成, 頻繁に変更されるのでpublic/に配置
  - `videos.min.json`: 一次生成を全てまとめたもの
  - `flat_clips.min.json`: 一次生成のclipsをflattenして, 検索/フィルタetc.に使えそうなもののみ抽出したもの

### `input.json`

新規動画(楽曲)データを追加する

初回追加時は入力ファイル(input.json)を作成し, プルリク出してactionsが動作すると一次生成に追加され, 自動的に入力ファイル(input.json)は消える. その後マージする.

```json
[
  {
    "video_id": "ZeFvqdvutb4",
    "uploaderName": "(例示用)",
    "videoTags": ["karaoke"],
    "clips": [
      {
        "songTitle": "おねがいダーリン",
        "artists": ["ruri-shioriha"],
        "externalArtists": ["(例示用)"],
        "isClipped": false,
        "startTime": "PT1M10S",
        "endTime": "PT4M21S",
        "clipTags": ["(例示用)"]
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

フィールドについて

- `video_id`: YouTubeの動画ID
- `uploaderName`: 動画のアップロード者名. 箱外のチャンネルのときのみ任意の適切な名称を付与する. 空文字列は絶対にno
- `videoTags`: 動画のタグ
  - e.g. "karaoke", "3d", "sitr-nagoya"
  - 命名規則
    - lower kebab-case
    - 子要素が親要素のタグを含む場合は親要素のタグも付与
      - e.g. Sing'in the Rainbow 名古屋公演 であれば `sitr`, `sitr-nagoya`の二つを付与
    - 以下のものは付与しない:
      - "2d"

- `clips`: 動画のクリップ情報の配列
  - `songTitle`: 楽曲のタイトル
    - e.g. "おねがいダーリン", "命に嫌われている。"
  - `artists`: 楽曲の歌唱者のアーティストID(箱内)の配列
  - `externalArtists`: 楽曲の外部アーティスト名(箱外)の配列
  - `isClipped`: このクリップが公式で切り抜かれているか
    - e.g. [この配信](https://youtu.be/6gKIA3_ihCY?t=1h4m20s)で最後から2番目に歌われている曲は, ご本人さんのチャンネルに[動画](https://youtu.be/NNVQm3qtkOY)として投稿されているので `true`
  - `startTime`: クリップの開始時間. 音(イントロ||声)が流れた時間を指定
    - e.g. 1分30.4秒など中途半端であれば0.4秒早めて`PT1M30S`を指定.
  - `endTime`: クリップの終了時間. 同上
    - e.g. 3分24.7秒であれば0.3秒遅くして`PT3M25S`を指定
  - `clipTags`: クリップのタグ. 現在のところ未使用

### `(年)/(月).json`

実際に動画の情報を格納するファイル

`publishedAt`の日付を基にフォルダを決定. 先ほどの日付(`publishedAt`)の古いほうが先頭になるように記述. また, 同じ動画(videoIdが同一)は同時に存在しないことを保証

TODO 上の内容を現段階では保証できてない

```json
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
        "artists": ["ruri-shioriha"],
        "externalArtists": ["(例示用)"],
        "isClipped": false,
        "startTime": "PT1M10S",
        "endTime": "PT4M21S",
        // publishedAtの日付(!日時) + startTimeを基に生成
        "uuid": "0197bbd1-039e-77f6-9112-519561b61f9e",
        "clipTags": ["(例示用)"],
        // 今は未実装. 実装しても, actionsで直接付与できるものではなくローカルからのPRになりそう
        "volumePercent": 50,
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

### `videos.min.json`

`(年)/(月).json`を全てまとめてmin化したもの

### `flat_clips.min.json`

`(年)/(月).json`のclipsをflattenして, 検索/フィルタetc.に使えそうなもののみ抽出したもの

```json
[
  {
    "uuid": "0197bbd1-039e-77f6-9112-519561b61f9e",
    "songTitle": "おねがいダーリン",
    "artists": ["ruri-shioriha"],
    "externalArtists": ["(例示用)"],
    "clipTags": ["(例示用)"],
    "startTime": "PT1M10S",
    "endTime": "PT4M21S"
    // volumePercentは含めない
  },
  {
    "uuid": "0197bbd1-039e-7b67-8c92-6f557c60f187",
    "songTitle": "命に嫌われている。",
    "artists": ["ruri-shioriha"],
    "startTime": "PT7M12S",
    "endTime": "PT11M34S"
  }
]
```

## 動画以外のデータ

- 手動で記述
  - `artists_data.json`: アーティストの全ての情報
- 自動で生成, 変更頻度が低いのでsrc/に配置
  - `artist_search_index.json`: アーティスト名の検索インデックス
  - `artists.json`: アーティストの一部の情報をflattenしてまとめたもの
  - `channels.json`: チャンネルidからアーティストidの対応

### `artists_data.json`

アーティスト情報を一元管理

非公式wikiの左側の順番で記述, セグメント分離されている場合は空行を挟む.

[スキーマ](/tools/artist.schema.json)を[設定](/.vscode/settings.json)済み

一旦jpのみ

```json
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
    "color": "2887FF"
    // "isGraduated": false  // 卒業済みかどうか, falseは記述しなくてよい
  },
  "meruto-kuramochi": {
    "ja": "倉持めると",
    "jah": "くらもちめると",
    "aliases": [ "めるち" ],
    "en": "Kuramochi Meruto",
    "channelId": "UCiA-trSZfB0i92V_-dyDqBw",
    "color": "EB4682"
  }
}
```

### `artist_search_index.min.json`

アーティスト名の検索インデックス

`artist_data.json`にある, `ja`, `jah`, `en`, `aliases`全ての値をキーにし, アーティストidを値にした配列. アーティスト名からアーティストidをO(n)で抽出できる

```json
[
  { "key": "栞葉るり", "artistId": "ruri-shioriha" },
  { "key": "しおりはるり", "artistId": "ruri-shioriha" },
  { "key": "Shioriha Ruri", "artistId": "ruri-shioriha" },
  { "key": "倉持めると", "artistId": "meruto-kuramochi" },
  { "key": "くらもちめると", "artistId": "meruto-kuramochi" },
  { "key": "Kuramochi Meruto", "artistId": "meruto-kuramochi" },
  // aliasは表示の優先度を下げたいので, aliasかどうかを判断できるようにしておく
  { "key": "めるち", "artistId": "meruto-kuramochi" , "isAlias": true }
]
```

### `artists.min.json`

アーティストの一部の情報をまとめたもの

`artists_data.json`の一部を抜粋して, フロントエンドでの表示に必要な情報のみを含む.

```json
{
  "ruri-shioriha": {
    "ja": "栞葉るり",
    "jah": "しおりはるり",
    "en": "Shioriha Ruri",
    "color": "2887FF"
    // "isGraduated": false  // 卒業済みかどうか, falseは記述されない
  },
  "meruto-kuramochi": {
    "ja": "倉持めると",
    "jah": "くらもちめると",
    "en": "Kuramochi Meruto",
    "color": "EB4682"
  }
}
```

### `channels.min.json`

YouTubeチャンネルIDとアーティストidの対応リスト

O(1)で引くため, 辞書形式を使用.

```json
{
  "UC7_MFM9b8hp5kuTSpa8WyOQ": "ruri-shioriha",
  "UCiA-trSZfB0i92V_-dyDqBw": "meruto-kuramochi"
}
```
