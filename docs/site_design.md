# cliplayer site design

`cliplayer` の静的 Angular サイトにおける、初版 frontend の詳細設計。
この文書は実装手順ではなく、画面責務、状態モデル、データ契約、永続化契約を定義する。

関連:

- 全体の生成フロー: [`./data_flow.md`](./data_flow.md)
- 検索システム概要: [`../search/overview.md`](../search/overview.md)
- 検索エンジン内部設計: [`../search/engine/design.md`](../search/engine/design.md)
- WASM 境界: [`../search/engine-wasm/usage.md`](../search/engine-wasm/usage.md)
- 検索バイナリ形式: [`../search/binary_schema.md`](../search/binary_schema.md)
- frontend が読む生成データ:
  - [`../metadata/artist/format.md`](../metadata/artist/format.md)
  - [`../metadata/tag/format.md`](../metadata/tag/format.md)
  - [`../music/data/format.md`](../music/data/format.md)

## 1. 目的

初版 frontend の目的は次の 2 点に絞る。

- 歌唱 clip を迷わず探せること
- 見つけた clip を壊れにくく連続再生できること

このサイトでは、検索単位も再生単位も `1 clip` とする。
同一動画に複数 clip が属するため、clip 単体だけでなく動画内の前後関係も辿れる UI にする。

## 2. 前提

- サイトは静的配信する
- frontend は Angular で実装する
- backend API は持たない
- locale は Angular 公式 i18n を使い、locale ごとの静的 build を配信する
- 検索は `public/search/search_index.bin` と `search/engine-wasm` を使ってブラウザ内で実行する
- 表示データは manifest 経由で解決した `*.min.json` を読む
- min JSON 群と検索 index は同じ `datasetBuildId` を共有する

責務分離:

- Rust tools: source-of-truth から `*.min.json` と `search_index.bin` を生成する
- search engine: query 評価、sort、cursor paging、`clip_uuid` 返却を担当する
- frontend: URL と検索状態の同期、join、queue 管理、YouTube player 制御、永続化を担当する

## 3. 設計原則

- 検索条件の正規化は 1 箇所に集約する
- lookup / join / 表示向け整形は component に散らさない
- UI は常に `clipUuid` を主キーにして動く
- queue と cursor は `datasetBuildId` を含む versioned data として扱う
- 壊れた生成物、古い保存状態、埋め込み不可動画、削除動画を通常系として扱う
- build 整合性が取れない状態では、無理に表示や復元を続けない
- レスポンシブ表現は CSS を正とし、TypeScript の viewport 判定は UI 挙動の切り替えに限定する
- breakpoint は少数の共有定義に集約し、component ごとに生の閾値を増やさない
- 色、余白、角丸、文字サイズ、z-index は design token として集約し、component へ生値を散らさない
- 共通 layout primitive を先に定義し、page ごとの独自レイアウト規則を増やしすぎない
- 視認性に責務を持つ semantic color と、雰囲気づくり専用の ambient color を分離する

## 4. 情報アーキテクチャ

初版 route は次に絞る。

| route                     | 役割                              |
| ------------------------- | --------------------------------- |
| `/:locale/`               | home                              |
| `/:locale/search`         | 検索条件の編集と結果一覧          |
| `/:locale/play`           | queue 再生                        |
| `/:locale/clip/:clipUuid` | clip 直リンク                     |
| `/:locale/**`             | not found。`/search` への導線表示 |

locale は少なくとも `ja`, `en` を持つ。
初版では locale prefix を必須とし、locale なし path を正規 URL としない。
`/` へのアクセスは配信設定または最小 bootstrap script で既定 locale へ redirect する。

### 4.1 `/`

- サイト概要を短く伝える
- 主導線は `/search` に寄せる
- 代表タグ、代表アーティスト、最近の clip などの固定導線を少数だけ置ける
- 動画削除や埋め込み不可で再生不能になる可能性は明示する

### 4.2 `/search`

- 検索条件を編集し、URL と同期する
- 検索結果を clip 単位で表示する
- 現在の検索結果から queue を作成する
- `cursor` による次ページ取得を行う

初版で扱う検索条件:

- `artist`
- `tag`
- `channel`
- `embeddable`
- `is_unlisted`
- `published_at` range
- sort

入力 UI 方針:

- `artist` は自由入力の autocomplete で選ばせる
- `artist` 入力は ARIA combobox pattern を満たす component とし、候補 popup、active descendant、キーボード操作、screen reader 読み上げを担保する
- artist 候補解決には `livers_search_index.min.json` を使う
- `tag` は selector で選ばせる
- `channel` は selector または補助付き選択 UI とする
- 初版の通常検索 UI は各条件を `and` で結合する

結果一覧で最低限表示する項目:

- 楽曲名
- 歌唱者
- 元動画タイトル
- 投稿チャンネル
- 公開日時
- 動画タグ
- clip 長
- 再生可否
- 同一動画内 sibling clip への導線

### 4.3 `/play`

- queue を再生する
- 現在 clip の情報を表示する
- 前後 clip への移動を提供する
- 同一動画内 sibling clip への移動を提供する
- 再生不可 item を理由付きでスキップまたは手動選択できる

queue が空の場合は空状態を出し、`/search` へ戻す導線を置く。

### 4.4 `/clip/:clipUuid`

- 単一 clip 直リンクの安定入口
- 内部的には `/play?clip=<uuid>` と同義で扱う
- clip が存在しない、または build 差し替えで参照不能なら専用の not-found 表示を出す

## 5. frontend 全体構成

初版 frontend は概ね次の層に分ける。

### 5.1 App Shell

- route 解決
- 初期データロード開始
- fatal error 表示

### 5.2 Data Layer

- `*.min.json` の fetch / validation
- build metadata 整合性確認
- lookup map 構築
- 表示向け join 用 repository 提供

### 5.3 Search Layer

- URL query string と `SearchUiState` の相互変換
- `SearchCanonicalState` の生成
- artist 自由入力から `artistId` を解決する補助検索
- Worker 上の WASM engine 呼び出し
- search result と local data join

### 5.4 Queue / Player Layer

- queue の生成、更新、復元、保存
- 現在再生位置の管理
- 再生可否判定
- YouTube iframe player 制御

### 5.5 Presentation Layer

- 各 route の container component
- 表示専用 component
- ローディング、空状態、非致命エラー表示

### 5.6 Design System / Theme Layer

- global styles で design token を定義する
- token は semantic color、spacing、radius、typography、shadow、z-index、breakpoint を含む
- component は token を参照して描画し、生の `#hex` や局所的な spacing 値を乱立させない
- responsive は CSS を主とし、`flex`、`grid`、`minmax()`、`clamp()`、必要に応じて container query を優先する
- `BreakpointObserver` などの viewport 判定は、drawer 開閉や常時表示切り替えのような UI 挙動にのみ使う
- page component は直接複雑なレイアウト規則を持たず、`page-shell`、`section`、`stack`、`cluster`、`grid`、`card` 相当の layout primitive を組み合わせる

Angular CDK / Material 方針:

- 初版は Angular Material の見た目を全面採用しない
- 見た目は自前の design token と自前 component で統一する
- 低レベル基盤は Angular CDK を優先して利用する
- 難易度と品質要求が高い部品は、まず Angular Material の実装と API 設計を参考にし、それでもコストや品質が見合わない場合のみ Material component の限定採用を検討する
- Material component を採用する場合でも、見た目の基盤として横展開せず、局所的な例外として扱う

CDK で優先採用する基盤:

- `@angular/cdk/overlay`
  - autocomplete 候補 popup、menu、popover、dialog 相当の重なり制御に使う
- `@angular/cdk/a11y`
  - focus trap、focus monitor、live announcer、keyboard navigation 補助などのアクセシビリティ基盤に使う
- `@angular/cdk/scrolling`
  - 長い一覧や候補リストで virtual scroll が必要になった場合の基盤として使う
- `@angular/cdk/portal`
  - overlay 内への動的 content 差し込み基盤として使う

独自 component で責務を持つ対象:

- button、icon button、chip、badge、card、input、select-like trigger、toolbar、page shell
- 検索条件 editor、clip card、queue panel、player control など domain UI
- combobox、dialog、menu、popover も見た目は自前で統一する

アクセシビリティ方針:

- overlay を使う UI は WAI-ARIA pattern に従う
- 具体的には `combobox`、`dialog`、`menu button`、`listbox` の各 pattern を満たす
- focus の初期位置、Escape での閉鎖、focus return、矢印キー操作、screen reader 向け name/role/state を仕様に含める
- 見た目を自前実装しても a11y 挙動は component ごとに毎回作り直さず、共通 primitive と directive に寄せる

Material component の限定採用候補:

- `MatAutocomplete`
- `MatDialog`
- `MatMenu`

ただし上記は「短期間で安全に出す必要があり、自前実装よりリスクが低い」と判断した場合だけに限る。
採用時は token ベースで外観を調整し、Material 固有の見た目が site 全体の基盤にならないようにする。

ambient color 方針:

- ユーザー設定で任意の 1 色を選べるようにしてよい
- ambient color は背景 tint、淡い gradient、装飾 glow、補助的な surface accent のみに使う
- ambient color は本文 text、境界線、フォーム入力、警告表示、主要 action、状態色の判定には使わない
- 選択色によってコントラストが下がっても問題ない領域だけで使う
- 実描画では 1 色をそのまま多用せず、元色から派生した薄いバリエーションを token 化して使う

theme 責務分離:

- `theme = light | dark | system` は基礎配色の切り替えを担当する
- ambient color は theme とは別軸の装飾設定として扱う
- theme 切り替え後も semantic color の可読性要件は維持し、ambient color はそれを侵食しない

## 6. 生成物の読み込み設計

固定パスの asset を個別に直接読む方式は deploy 途中や CDN キャッシュで build 混在を起こしやすい。
そのため frontend はまず manifest を読み、同一 build に属する asset URL 集合を取得してから本体 asset を読む。

### 6.0 manifest

最初に読む固定パス asset は manifest のみとする。

```ts
type DatasetManifest = {
  datasetBuildId: DatasetBuildId;
  datasetSchemaVersion: number;
  basePath: string;
  assets: {
    livers: DatasetAssetDescriptor;
    liversSearchIndex: DatasetAssetDescriptor;
    officialChannels: DatasetAssetDescriptor;
    channels: DatasetAssetDescriptor;
    tags: DatasetAssetDescriptor;
    clips: DatasetAssetDescriptor;
    videos: DatasetAssetDescriptor;
    searchIndex: SearchIndexAssetDescriptor;
  };
};

type DatasetAssetDescriptor = {
  assetKind: "livers" | "livers_search_index" | "official_channels" | "channels" | "tags" | "clips" | "videos";
  url: string;
  assetSchemaVersion: number;
  sha256?: string;
};

type SearchIndexAssetDescriptor = {
  assetKind: "search_index";
  url: string;
  formatVersion: number;
  sha256?: string;
};
```

配信パス方針:

- manifest 自体は固定パスでよい
- 実 asset URL は `datasetBuildId` を含む path を推奨する
- 例: `/public/datasets/{datasetBuildId}/music/clips.min.json`
- frontend は固定パスの `public/music/*.min.json` を直接参照しない

manifest が指す初版アセットは次の通り。

- `public/music/livers.min.json`
- `public/music/livers_search_index.min.json`
- `public/music/official_channels.min.json`
- `public/music/channels.min.json`
- `public/music/tags.min.json`
- `public/music/clips.min.json`
- `public/music/videos.min.json`
- `public/search/search_index.bin`

`livers_search_index.min.json` は初版必須とする。
ただし役割は clip 検索そのものではなく、artist 入力補助専用の frontend-side index である。
clip 検索の正は引き続き `search_index.bin` とする。

### 6.1 ロード順序

1. manifest を fetch
2. manifest の `datasetBuildId`, `datasetSchemaVersion`, asset descriptor 群を validate
3. manifest が指す JSON 群を並列 fetch
4. 各 JSON の envelope を validate
5. 各 JSON の `datasetBuildId`, `schemaVersion`, `assetKind` が manifest と一致するか確認
6. JSON 同士の参照整合性を validate
7. artist 補助 index をメモリ上の候補検索構造へ変換する
8. join 用の in-memory repository を構築
9. manifest が指す `search_index.bin` を fetch
10. binary header / metadata から `formatVersion` と `datasetBuildId` を確認
11. manifest / JSON 側 `datasetBuildId` と一致したら Worker を初期化

### 6.2 fatal / partial の境界

fatal とする条件:

- manifest fetch 失敗
- manifest schema 不正
- 必須 JSON の fetch 失敗
- JSON parse 失敗
- JSON schema 不正
- JSON 間 `datasetBuildId` 不一致
- manifest と JSON の `assetKind` / `schemaVersion` 不一致
- `search_index.bin` load 失敗
- index 側 `datasetBuildId` 不一致
- index `formatVersion` 非対応

partial degradation とする条件:

- 個別 clip の参照切れ
- queue 内の一部 item 不正
- 個別動画の埋め込み不可
- YouTube runtime error

## 7. frontend が保持する正規化データ

component 直下では raw JSON を読まない。
ロード後は次の repository を構築して参照する。

```ts
type DatasetBuildId = string;
type ClipUuid = string;
type VideoId = string;
type ArtistId = string;
type TagId = string;
type ChannelId = string;
```

### 7.1 共通 envelope

```ts
type MinEnvelope<T> = {
  assetKind: string;
  schemaVersion: number;
  datasetBuildId: DatasetBuildId;
  data: T;
};
```

この文書での `schemaVersion` は asset ごとの version ではなく、frontend が同時に解釈する dataset 全体の互換性 version とする。
そのため同一 build の JSON 群では共通値でなければならない。

asset ごとの進化が必要になった場合は、`DatasetManifest.assets[].assetSchemaVersion` を拡張点として使う。
初版 frontend の互換性判定は `MinEnvelope.schemaVersion` を主とし、`assetSchemaVersion` は descriptor 上の補助情報として扱う。

### 7.2 Raw assets

```ts
type LiverMin = {
  ja: string;
  jah: string;
  en: string;
  color: string;
  channelId: ChannelId;
  isGraduated?: true;
};

type OfficialChannelMin = {
  ja: string;
  jah: string;
  en: string;
  channelId: ChannelId;
};

type ChannelOwnerMin = { kind: "liver"; id: ArtistId } | { kind: "official"; id: string };

type TagMin = {
  ja: string;
  en: string;
  intId: number;
  blocked?: true;
};

type LiverSearchIndexEntryMin = {
  key: string;
  liverId: ArtistId;
  isAlias?: true;
};

type ClipMin = {
  videoId: VideoId;
  startTimeSecs: number;
  endTimeSecs: number;
  songTitle: string;
  liverIds: ArtistId[];
  externalArtistsName?: string[];
  clippedVideoId?: VideoId;
};

type VideoMin = {
  clipUuids: ClipUuid[];
  durationSecs: number;
  title: string;
  channelId: ChannelId;
  uploaderName: string;
  publishedAtIso: string;
  publishedAtEpochSec: number;
  syncedAtIso: string;
  syncedAtEpochSec: number;
  privacyStatus: "public" | "unlisted" | "private";
  embeddable: boolean;
  videoTags: TagId[];
};

type LiversMinData = Record<ArtistId, LiverMin>;
type LiversSearchIndexMinData = LiverSearchIndexEntryMin[];
type OfficialChannelsMinData = Record<string, OfficialChannelMin>;
type ChannelsMinData = Record<ChannelId, ChannelOwnerMin>;
type TagsMinData = Record<TagId, TagMin>;
type ClipsMinData = Record<ClipUuid, ClipMin>;
type VideosMinData = Record<VideoId, VideoMin>;
```

raw JSON shape 規則:

- `livers.min.json`, `official_channels.min.json`, `channels.min.json`, `tags.min.json`, `clips.min.json`, `videos.min.json` は `Record<Id, Value>` を正とする
- `livers_search_index.min.json` は順序付き配列を正とする
- `ClipMin` に `clipUuid` field は持たず、`ClipsMinData` の key が `clipUuid` である
- `VideoMin.clipUuids` の順序を動画内 clip 順の正とする
- `clipOrderByVideo` は `VideoMin.clipUuids` をそのまま採用する
- `VideoMin.clipUuids` と `ClipMin.startTimeSecs` の順序が食い違う場合は build 不整合として fatal にする

時間表現規則:

- `publishedAtIso`, `syncedAtIso` は RFC3339 UTC string とする
- `publishedAtEpochSec`, `syncedAtEpochSec` は整数秒とする
- `startTimeSecs`, `endTimeSecs`, `durationSecs` はすべて整数秒とする
- frontend は表示用に ISO を使ってよいが、比較・sort・再生制御は epoch / integer seconds を正とする

### 7.3 In-memory normalized repository

```ts
type AppDataset = {
  meta: DatasetMeta;
  artists: Record<ArtistId, ArtistEntity>;
  artistSearchIndex: ArtistSearchIndex;
  officialChannels: Record<string, OfficialChannelEntity>;
  channels: Record<ChannelId, ChannelOwnerEntity>;
  tags: Record<TagId, TagEntity>;
  clips: Record<ClipUuid, ClipEntity>;
  videos: Record<VideoId, VideoEntity>;
  clipOrderByVideo: Record<VideoId, ClipUuid[]>;
};

type DatasetMeta = {
  schemaVersion: number;
  datasetBuildId: DatasetBuildId;
  searchIndexFormatVersion: number;
};

type ArtistSearchIndex = {
  entries: LiverSearchEntry[];
};

type ArtistEntity = {
  artistId: ArtistId;
  nameJa: string;
  nameJaHiragana: string;
  nameEn: string;
  colorHex: string;
  channelId: ChannelId;
  isGraduated: boolean;
};

type LiverSearchEntry = {
  key: string;
  normalizedKey: string;
  artistId: ArtistId;
  isAlias: boolean;
};

type OfficialChannelEntity = {
  officialChannelId: string;
  nameJa: string;
  nameJaHiragana: string;
  nameEn: string;
  channelId: ChannelId;
};

type ChannelOwnerEntity = { kind: "liver"; channelId: ChannelId; artistId: ArtistId } | { kind: "official"; channelId: ChannelId; officialChannelId: string } | { kind: "external"; channelId: ChannelId };

type TagEntity = {
  tagId: TagId;
  nameJa: string;
  nameEn: string;
  intId: number;
  blocked: boolean;
};

type ClipEntity = {
  clipUuid: ClipUuid;
  videoId: VideoId;
  videoClipIndex: number;
  startTimeSecs: number;
  endTimeSecs: number;
  durationSecs: number;
  songTitle: string;
  liverIds: ArtistId[];
  externalArtistsName: string[];
  clippedVideoId: VideoId | null;
};

type VideoEntity = {
  videoId: VideoId;
  clipUuids: ClipUuid[];
  durationSecs: number;
  title: string;
  channelId: ChannelId;
  uploaderName: string;
  publishedAtIso: string;
  publishedAtEpochSec: number;
  syncedAtEpochSec: number;
  syncedAtIso: string;
  privacyStatus: "public" | "unlisted" | "private";
  embeddable: boolean;
  videoTags: TagId[];
};
```

補足:

- `ChannelOwnerEntity.kind = "external"` は `channels.min.json` に定義がない `video.channelId` を frontend が補完するときだけ生成する
- external channel の表示名は `VideoEntity.uploaderName` を優先し、空なら `channelId` を退避表示する
- `TagEntity.intId` は frontend 内部の補助データとして保持してよいが、URL・queue・設定保存の stable ID には使わない

### 7.4 派生 view model

画面描画で直接便利な join 済み model は repository から導出する。

```ts
type ClipCardView = {
  clipUuid: ClipUuid;
  songTitle: string;
  artists: Array<{ artistId: ArtistId; nameJa: string; colorHex: string }>;
  externalArtistsName: string[];
  video: {
    videoId: VideoId;
    title: string;
    publishedAtIso: string;
    channelId: ChannelId;
    channelLabel: string;
    embeddable: boolean;
    privacyStatus: "public" | "unlisted" | "private";
  };
  tags: Array<{ tagId: TagId; nameJa: string; blocked: boolean }>;
  startTimeSecs: number;
  endTimeSecs: number;
  durationSecs: number;
  playableState: ClipPlayableState;
  siblingClipUuids: ClipUuid[];
};
```

`ClipCardView` は pure function で生成できる形にし、component が個別 lookup しない。

## 8. 参照整合性ルール

ロード時に最低限次を検証する。

- `clip.videoId` が `videos` に存在する
- `video.clipUuids` に含まれる uuid が `clips` に存在する
- `video.clipUuids` と `clip.videoId` が相互に整合する
- `clip.liverIds` の各値が `artists` に存在する
- `livers_search_index.min.json` の `liverId` が `artists` に存在する
- `video.videoTags` の各値が `tags` に存在する
- `video.channelId` が `channels` に存在しない場合、`uploaderName` が非空であれば external channel として補完できる
- `video.channelId` が `channels` に存在せず、かつ `uploaderName` も空なら fatal
- `clip.startTimeSecs < clip.endTimeSecs`
- `clip.endTimeSecs <= video.durationSecs`
- `video.publishedAtEpochSec` と `video.publishedAtIso` が同じ instant を表す
- `video.syncedAtEpochSec` と `video.syncedAtIso` が同じ instant を表す

不整合時の扱い:

- build 全体が壊れていると判断できるものは fatal
- 一部 record のみ壊れている場合は、その record を除外して warning を蓄積する

初版推奨は、安全側に倒して `clips` と `videos` の相互参照不整合を fatal にすること。
検索結果が `clip_uuid` のみ返すため、この層が壊れると表示の前提が崩れるためである。

## 9. URL と検索状態

検索状態は `/:locale/search` の query string に保持する。
queue 本体は URL に載せない。

### 9.0 locale URL 方針

- locale は URL prefix で表現する
- 初版対象 locale は `ja`, `en`
- locale 切り替えは SPA 内の文言差し替えではなく、対応 locale URL への遷移で行う
- query string は locale 切り替え時に可能な限り維持する
- `/ja/search?...` から `/en/search?...` へ切り替える場合、検索条件は同じでも文言と書式は locale ごとの build に従う

Angular 公式 i18n を使うため、初版では runtime translation による無リロード切り替えは採らない。

### 9.1 URL パラメータ仕様

初版の canonical query string は次を採用する。

| key          | type                                          | 例                                  |
| ------------ | --------------------------------------------- | ----------------------------------- |
| `artist`     | CSV of `ArtistId`                             | `artist=mito-tsukino,ruri-shioriha` |
| `tag`        | CSV of `TagId`                                | `tag=karaoke,duet`                  |
| `channel`    | CSV of `ChannelId`                            | `channel=UCxxxx,UCyyyy`             |
| `embed`      | enum: `any`, `only`, `exclude`                | `embed=only`                        |
| `unlisted`   | enum: `any`, `only`, `exclude`                | `unlisted=exclude`                  |
| `from`       | `YYYY-MM-DD`                                  | `from=2023-01-01`                   |
| `to`         | `YYYY-MM-DD`                                  | `to=2023-12-31`                     |
| `sort`       | enum: `published_at_desc`, `published_at_asc` | `sort=published_at_desc`            |
| `pageCursor` | opaque string                                 | engine 由来 token                   |

補足:

- 複数選択は repeated key ではなく CSV に統一する
- 空値、重複値、未知値は canonicalization で除去する
- `pageCursor` は query 条件が変わった時点で無効化する

### 9.2 UI state

```ts
type SearchUiState = {
  artistIds: ArtistId[];
  artistInput: string;
  tagIds: TagId[];
  channelIds: ChannelId[];
  embeddable: "any" | "only" | "exclude";
  isUnlisted: "any" | "only" | "exclude";
  publishedFrom: string | null;
  publishedTo: string | null;
  sort: "published_at_desc" | "published_at_asc";
  pageCursor: string | null;
};
```

`artistInput` は URL の正ではない。
URL に載せるのは解決済み `artistIds` のみとし、自由入力の途中状態は ephemeral UI state として扱う。

### 9.3 canonical state

`SearchUiState` から検索処理で使う canonical state を生成する。

```ts
type SearchCanonicalState = {
  artistIds: ArtistId[];
  tagIds: TagId[];
  channelIds: ChannelId[];
  embeddable: "any" | "only" | "exclude";
  isUnlisted: "any" | "only" | "exclude";
  publishedFromEpochSec: number | null;
  publishedToEpochSecExclusive: number | null;
  sort: SearchSortSpec;
  pageCursor: string | null;
  fingerprint: string;
};

type SearchSortSpec = {
  field: "published_at";
  order: "asc" | "desc";
};
```

canonicalization 規則:

- array 値は sort + dedup
- 未知 ID は除去する
- `from`, `to` は UTC 日付境界に変換する
- `to` は当日終端を inclusive にせず、翌日 00:00:00 UTC の exclusive bound に変換する
- `from > to` は empty query ではなく invalid UI input として扱い、検索実行を止める
- default 値は URL から省略する
- `fingerprint` は cursor 再利用判定と検索 request cache key に使う

### 9.4 artist 入力補助

artist 条件の入力補助は検索 engine ではなく frontend 側で完結する。

- 入力文字列を正規化して `livers_search_index.min.json` を探索する
- exact match だけでなく前方一致と軽量 fuzzy match を行う
- 候補表示では alias hit より正式名 hit を優先する
- 候補選択後に `artistId` を `SearchUiState.artistIds` へ確定させる
- engine に渡す query には自由入力文字列を含めず、確定済み `artistId` だけを含める

この補助 index は query builder 用であり、clip 検索結果そのものを返す用途には使わない。

## 10. Search Engine 境界

frontend と Worker の間では URL パラメータ列ではなく構造化 request をやり取りする。

### 10.1 frontend request

```ts
type SearchRequestDto = {
  query: QueryNodeDto | null;
  sort: [SortSpecDto];
  page: {
    limit: number;
    cursor: string | null;
  };
  totalMode: "none" | "exact";
};

type SortSpecDto = {
  field: "published_at";
  order: "asc" | "desc";
};

type QueryNodeDto = { type: "and"; children: QueryNodeDto[] } | { type: "or"; children: QueryNodeDto[] } | { type: "not"; child: QueryNodeDto } | { type: "term"; term: QueryTermDto };

type QueryTermDto =
  | { field: "artist_id"; op: "any_in"; values: ArtistId[] }
  | { field: "tag_id"; op: "any_in"; values: TagId[] }
  | { field: "channel_id"; op: "any_in"; values: ChannelId[] }
  | { field: "embeddable"; op: "eq"; value: boolean }
  | { field: "is_unlisted"; op: "eq"; value: boolean }
  | {
      field: "published_at";
      op: "range";
      gte: number | null;
      lt: number | null;
    };
```

### 10.2 query 生成規則

`SearchCanonicalState` から次の規則で AST を作る。

- 条件が 1 つもなければ `query = null`
- `artistIds` は `term(field=artist_id, any_in)`
- `tagIds` は `term(field=tag_id, any_in)`
- `channelIds` は `term(field=channel_id, any_in)`
- `embeddable=only` は `embeddable eq true`
- `embeddable=exclude` は `embeddable eq false`
- `isUnlisted=only` は `is_unlisted eq true`
- `isUnlisted=exclude` は `is_unlisted eq false`
- 日付範囲は `published_at range`
- すべて `and(...)` で結合する

### 10.3 Worker response

```ts
type SearchResponseDto = {
  clipUuids: ClipUuid[];
  nextCursor: string | null;
  totalMode: "none" | "exact";
  total: number | null;
  hasMore: boolean;
  warnings?: QueryWarningDto[];
};

type QueryWarningDto = { kind: "UnknownArtistId"; artistId: ArtistId } | { kind: "UnknownTagId"; tagId: TagId } | { kind: "UnknownChannelId"; channelId: ChannelId };
```

response invariant:

- `hasMore = false` のとき `nextCursor = null`
- `hasMore = true` のとき `nextCursor != null`

## 11. 検索結果の join

検索結果は `clipUuid[]` のみ返るため、frontend 側で表示用情報に join する。

### 11.1 join 手順

1. `clipUuid` から `ClipEntity` を引く
2. `ClipEntity.videoId` から `VideoEntity` を引く
3. `ClipEntity.liverIds` から `ArtistEntity[]` を引く
4. `VideoEntity.videoTags` から `TagEntity[]` を引く
5. `VideoEntity.channelId` から `ChannelOwnerEntity` を引く
6. `video.clipUuids` から sibling clip 一覧を作る
7. `playableState` を計算する

### 11.2 再生可否

再生可否は `embeddable` だけで決めない。
表示上は少なくとも次の状態を区別する。

```ts
type ClipPlayableState = { kind: "playable" } | { kind: "blocked_tag"; tagId: TagId } | { kind: "not_embeddable" } | { kind: "privacy_private" } | { kind: "missing_video" } | { kind: "invalid_clip_range" } | { kind: "youtube_runtime_error"; code: string };
```

初版規則:

- `video.embeddable === false` は `not_embeddable`
- `video.privacyStatus === "private"` は `privacy_private`
- `video.videoTags` に `blocked=true` の tag があれば `blocked_tag`
- clip / video 参照不整合は `missing_video` または `invalid_clip_range`
- YouTube player からの失敗は runtime error として別管理する

## 12. queue 設計

queue は clip UUID の列として管理する。
再生順は queue 順を正とし、検索結果順や動画内順に暗黙同期しない。

### 12.1 in-memory queue model

```ts
type QueueState = {
  queueId: string;
  datasetBuildId: DatasetBuildId;
  items: QueueItem[];
  currentIndex: number | null;
  origin: QueueOrigin;
  updatedAtEpochMs: number;
};

type QueueItem = {
  clipUuid: ClipUuid;
  addedAtEpochMs: number;
};

type QueueOrigin = { kind: "manual" } | { kind: "single_clip"; clipUuid: ClipUuid } | { kind: "search_result"; searchFingerprint: string };
```

規則:

- `items` は重複を許可してよい
- `currentIndex = null` は空 queue のみで許可
- `currentIndex` は常に `0 <= index < items.length` を満たす

### 12.2 queue 操作

初版で必要な操作:

- 単一 clip から queue を新規作成する
- 検索結果ページから queue を新規作成する
- queue の末尾に clip を追加する
- queue 内の item を削除する
- 現在位置を前後に移動する
- 再生不能 item をスキップする

初版では shuffle, repeat, drag-and-drop 並び替えは対象外とする。

## 13. 永続化設計

queue の保存には `IndexedDB` を使う。
`localStorage` は容量と破損耐性の面で採用しない。

ユーザー設定は queue と分離して保存する。
設定サイズが小さく migration も単純なため、初版では `localStorage` または専用 lightweight storage を採用してよい。

### 13.1 保存対象

```ts
type PersistedQueueV1 = {
  version: 1;
  savedAtEpochMs: number;
  datasetBuildId: DatasetBuildId;
  queue: {
    queueId: string;
    items: Array<{ clipUuid: ClipUuid; addedAtEpochMs: number }>;
    currentIndex: number | null;
    origin: QueueOrigin;
    updatedAtEpochMs: number;
  };
};
```

```ts
type UserPreferencesV1 = {
  version: 1;
  locale: "ja" | "en";
  theme: "light" | "dark" | "system";
  ambientColorHex: string | null;
  searchMode: "basic";
  filterGraduatedArtists: "strict" | "loose" | "disabled";
};
```

検索結果 cursor を保存する場合は queue とは別 object store に分ける。
初版では queue 復元を優先し、cursor 永続化は必須ではない。

設定値の意味:

- `locale`
  - 初回アクセス時の redirect 先 locale 候補に使う
  - locale URL が明示されている場合は URL を優先する
- `theme`
  - `light`, `dark`, `system` を持つ
- `ambientColorHex`
  - 雰囲気づくり専用の accent color を `#RRGGBB` で保持する
  - `null` は既定の ambient palette を使う
  - text、border、warning、primary action など semantic color の置き換えには使わない
- `searchMode`
  - 初版は `basic` 固定
  - 次バージョンで `advanced` を追加する前提で versioned field として持つ
- `filterGraduatedArtists`
  - strict: 卒業したライバーが一人でも含まれているclipはすべて検索結果から除外する
  - loose: 卒業したライバーのみで構成されているclipは検索結果から除外し、卒業していないライバーが一人でも含まれていればclipは表示する
  - disabled: 卒業したライバーを含むclipも検索結果に表示する

### 13.2 復元規則

復元時は次の順で検証する。

1. `version` が対応範囲内か
2. `datasetBuildId` が現在ロード済み dataset と一致するか
3. `items[].clipUuid` が現在 dataset に存在するか
4. `currentIndex` が有効範囲か

対応:

- `version` 不一致: 全破棄
- `datasetBuildId` 不一致: 全破棄
- 一部 clip 不存在: その item だけ除外
- `currentIndex` 不正: 先頭に補正。空なら `null`

### 13.3 `/:locale/play` 初期化規則

- `/:locale/play?clip=<uuid>` は単一 clip queue を新規作成する
- `/:locale/clip/:clipUuid` も同様に単一 clip queue を新規作成する
- `/:locale/play` を query なしで開いた場合は保存済み queue を復元する
- 復元不能なら空 queue にする

`/:locale/play?clip=<uuid>` を開いたときは保存済み queue より URL を優先する。

## 14. 検索ページングと cursor

cursor は engine が返す opaque token として扱う。
frontend は token 内部構造に依存しない。

### 14.1 cursor 再利用条件

現在の `pageCursor` は次の条件をすべて満たす場合だけ使う。

- `datasetBuildId` が同じ build を前提にした画面である
- `SearchCanonicalState.fingerprint` が前回と同じ
- sort が同じ

実際の build / query / sort 照合は engine 側でも検証されるが、
frontend でも条件変更時に `pageCursor = null` へ落とす。

### 14.2 ページング UX

- 初回検索は `limit = 50`
- 次ページ取得は明示操作で実行する
- query 条件変更時は結果一覧、cursor、エラー表示をリセットする
- 検索中は古い response で上書きしないよう request identity を持つ

## 15. YouTube player 連携

YouTube iframe player は `/:locale/play` のみで初期化する。
検索ページでは再生 preview を持たない。

### 15.1 player に渡す値

- `videoId`
- `startSeconds`
- `endSeconds`

値は常に `ClipEntity` と `VideoEntity` から導出し、URL 断片や DOM 状態から逆算しない。

### 15.2 player 状態

```ts
type PlayerRuntimeState = { kind: "idle" } | { kind: "loading"; clipUuid: ClipUuid } | { kind: "playing"; clipUuid: ClipUuid } | { kind: "paused"; clipUuid: ClipUuid } | { kind: "ended"; clipUuid: ClipUuid } | { kind: "error"; clipUuid: ClipUuid; code: string };
```

### 15.3 clip 終了時の規則

- clip の `endTimeSecs` 到達を検知したら次 item へ進む
- 次 item が再生不能なら順にスキップする
- 到達可能な playable item がなければ queue 終端として停止する

## 16. State Machine

初版では route ごとに無秩序な `boolean` 群を持たず、有限状態で扱う。

### 16.1 app load

```ts
type AppLoadState = { kind: "booting" } | { kind: "loading_assets" } | { kind: "ready"; dataset: AppDataset } | { kind: "fatal_error"; error: FatalAppError };
```

### 16.2 search page

```ts
type SearchPageState =
  | { kind: "idle"; ui: SearchUiState }
  | { kind: "searching"; ui: SearchUiState; requestKey: string }
  | {
      kind: "ready";
      ui: SearchUiState;
      result: SearchJoinedResult;
    }
  | {
      kind: "error";
      ui: SearchUiState;
      error: SearchUiError;
    };

type SearchJoinedResult = {
  clipCards: ClipCardView[];
  nextCursor: string | null;
  totalMode: "none" | "exact";
  total: number | null;
  hasMore: boolean;
  warnings: QueryWarningDto[];
};
```

### 16.3 play page

```ts
type PlayPageState =
  | { kind: "empty_queue" }
  | {
      kind: "ready";
      queue: QueueState;
      currentClip: ClipCardView;
      player: PlayerRuntimeState;
    }
  | {
      kind: "recoverable_error";
      queue: QueueState;
      error: PlayUiError;
    };
```

## 17. エラー分類

### 17.1 fatal app error

```ts
type FatalAppError = { kind: "asset_fetch_failed"; asset: string } | { kind: "asset_parse_failed"; asset: string } | { kind: "asset_schema_invalid"; asset: string } | { kind: "dataset_build_mismatch" } | { kind: "search_index_incompatible"; formatVersion: number } | { kind: "search_engine_init_failed" };
```

### 17.2 search ui error

```ts
type SearchUiError = { kind: "invalid_date_range" } | { kind: "worker_request_failed" } | { kind: "cursor_rejected" } | { kind: "result_join_failed"; clipUuid: ClipUuid };
```

### 17.3 play ui error

```ts
type PlayUiError = { kind: "clip_not_found"; clipUuid: ClipUuid } | { kind: "queue_restore_failed" } | { kind: "youtube_api_error"; code: string };
```

## 18. 初版で固定する非機能要件

- 主要検索処理は Web Worker 上で行い main thread を塞がない
- component は immutable な入力 model を受け取る
- URL から復元できるのは検索条件のみとし、queue 本体は永続化ストアから復元する
- 生成物 schema の変更は `schemaVersion` または `formatVersion` で検出する
- build の差し替えをまたいだ queue / cursor 再利用は保証しない
- locale 切り替えは locale URL 間の遷移で行い、同一 build 内の runtime 翻訳切り替えは前提にしない

## 19. 初版でやらないこと

- backend API
- ログイン
- プレイリスト共有 URL
- 全文検索
- facet 集計
- 任意 AST を直接編集する詳細検索 UI
- 検索結果全件 queue 化
- offline 対応
- 複数 player 同時管理
- queue の複雑な編集機能
- 再生履歴同期

詳細検索 UI は初版の次バージョンで追加前提とするが、この文書の初版範囲には含めない。

初版は、clip を安全に検索し、安全に連続再生できることに責務を絞る。
