# foo

タグ関連の設計考える
動画に対して付与したものとクリップごとのものをどうするか...
2次生成で分離, clipsに動画に対してのタグも持たせる <-ありでは?

ユニット名などは名前から算出できるけど処理速度が低下しそう

musictl apply new -i ./input.json: 新しくデータを作成(WebApi有)
musictl apply update -i ./music.json: 既存のデータに対して直接変更を適用
musictl validate new-input -i ./input.json: 新しくデータを生成するための情報が入ったファイルの形式を確認
musictl validate update-input -i ./music.json: 既存のデータに対して直接変更を適用するための情報が入ったファイルの形式を確認
musictl validate exists -i ./music.json: 追加しようとしているデータが重複していないか

上のオプション更新する

musictl summary 的なもので.mdとかに登録済みの配信一覧出力機能作ったりしたい

...

TODO cliから頑張って処理繋げる
music.min.json用の構造体作成 <- fs/にあった, 別の場所に移動してfs/をフォルダごと消す
全ファイルのunittest/comment/documentation見直し

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
