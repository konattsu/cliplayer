# foo

- 日本語での楽曲名消す
- 公式楽曲にはclipタグとしてnijisanji
- channelIdからuploaderNameの存在の妥当性を検証 <- artistsでlazy_staticとは別に外部ファイルをもう一つ読み取る方がいいと思う, O(1)のファイルが適切だと

タグ関連の設計考える
動画に対して付与したものとクリップごとのものをどうするか...
2次生成で分離, clipsに動画に対してのタグも持たせる <-ありでは?

ユニット名などは名前から算出できるけど処理速度が低下しそう

musictl summary 的なもので.mdとかに登録済みの配信一覧出力機能作ったりしたい

...

TODO :

- anonymousを作成する処理
  - verifiedみたいにvideosをanonymousにも作る
  - fileからserde_json::from_readerをもつ関数をどっかに配置 <- どこがいい?, エラーは正味Vec<\String>とかでprettierしてあげてもいい, 専用のエラー型作ってもいい <- 使いどころそんなにないけど

- fetchにつなげる処理
  - sync(最新の動画情報と同期)の時は定期的にファイルに書き込むようにする

- channelIdからuploaderNameの存在の妥当性を検証 <- ファイル先頭に書いてあった. 何のことか思い出す

...

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

github actionsのtimeoutを追加, defaultは6hらしい, 逆に言えばWebApi用いた既存の楽曲情報の更新に多少時間がかかっても大丈夫
