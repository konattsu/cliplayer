# データの扱われ方

## 動画以外のデータ

- 手動で記述
  - `artists_data.json`: アーティストの全ての情報
- 自動で生成(public/以下に配置, min化)
  - `artists_search_index.json`: アーティスト名の検索インデックス
  - `artists.json`: アーティストの一部の情報を
  - `channels.json`: チャンネルidからアーティストidの対応

### `artists_data.json`

アーティスト情報を一元管理

名前の表記ゆれ防止のため[公式タレント一覧ページ](https://www.nijisanji.jp/talents)要参照

ぎばさんのチャンネルは削除されているのでapiでfetchしたときのデータと整合性を取る処理などは要注意

```json
// artists_data.json
// 手動で入力
{
  "ruri-shioriha": {
    // 日本語での名前
    "ja": "栞葉るり",
    // 日本語の平仮名表記
    "jah": "しおりはるり",
    // 英語(ローマ字)での表記
    "en": "Shioriha Ruri",
    // 表記ゆれ, 呼称を許容するための配列. ファジー検索用
    "aliases": [],
    // YouTubeチャンネルID. ハンドルネームではない
    "channelId": "UC7_MFM9b8hp5kuTSpa8WyOQ",
    // モチーフカラー. 公式ページか非公式wiki参照
    "color": "2887FF"
    // "isGraduated": false  // 卒業済みかどうか, falseは記述しなくていい
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

### `artists_search_index.json`

アーティスト名の検索インデックス (ファジー検索用)

`artists_data.json`にある, `ja`, `jah`, `en`, `aliases`全ての値をキーにし, アーティストidを値にした配列. アーティスト名からアーティストidをO(n)で抽出できるため. アーティストidがあればそれを基にした`clip`検索などを行える.

```json
// artists_search_index.min.json
// 自動生成, public以下に配置, 実際はmin化
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

### `artists.json`

アーティストの一部の情報をまとめたもの

`artists_data.json`の一部を抜粋して, フロントエンドでの表示に必要な情報のみを含む.

```json
// artists.min.json
// 自動生成, public以下に配置, 実際はmin化
{
  "ruri-shioriha": {
    "ja": "栞葉るり",
    "jah": "しおりはるり",
    "en": "Shioriha Ruri",
    "color": "2887FF"
    // "isGraduated": false  // 卒業済みかどうか, falseは記述しなくていい
  },
  "meruto-kuramochi": {
    "ja": "倉持めると",
    "jah": "くらもちめると",
    "en": "Kuramochi Meruto",
    "color": "EB4682"
  }
  //
}
```

### `channels.json`

YouTubeチャンネルIDからアーティストidの対応リスト

O(1)で引くため, 辞書形式を使用.

```json
// channels.min.json
// 自動生成, public以下に配置, 実際はmin化
{
  "UC7_MFM9b8hp5kuTSpa8WyOQ": "ruri-shioriha",
  "UCiA-trSZfB0i92V_-dyDqBw": "meruto-kuramochi"
}
```
