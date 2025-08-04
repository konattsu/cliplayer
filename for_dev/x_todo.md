# foo

## back

/music_data/src/apply/直す
uuidv4にする
artists_idをmusic_data.code-snippetsに埋め込むようにする(/music_data/artistに処理追加する). schemaはいらない. どうせcliに食わせるので.
musictl summary 的なもので.mdとかに登録済みの配信一覧出力機能作ったりしたい

- e2e
  - testcontainers-rs + mockserver(wiremock)
  - endpointのconstを #[\cfg(test)]とか#[\cfg(features ...)]で切り替え
  - constでなくlazy_static使う <- portが多分動的なのでenvぐらいから引っ張ってくる
  - tempfileはもちもちもちもちもちろん使う

## front

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

- 動画情報のみで検索したい:
  - videos.jsonのみ使用
- クリップ情報のみで検索したい:
  - clip_search_index.jsonのみ使用
- 動画情報とクリップ情報の両方で検索したい:
  1. videos.jsonで動画を検索し, 適切なvideoIdを取得
  2. clip_search_index.jsonでそのvideoIdに紐づくクリップと任意の条件を満たすcilpを検索

## else

- github actionsのtimeoutを追加, defaultは6hらしい, 逆に言えばWebApi用いた既存の楽曲情報の更新に多少時間がかかっても大丈夫

たぐ

- 公式楽曲にはclipタグとしてnijisanji
- ユニット名などは名前から算出できるけど処理速度が低下しそう
