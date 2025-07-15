/// 既存の楽曲情報に対する更新を適用
///
/// # Arguments
/// - `root`: 楽曲情報のルート
/// - `min_path`: min化した楽曲情報を書き出すパス
/// - `min_flat_clips_path`: フラットなクリップ情報を書き出
///
/// # Returns
/// - `Ok(())`: 正常に更新が適用された場合
/// - `Err(String)`: エラーが発生した場合の整形されたエラーメッセージ
pub fn apply_update(
    root: crate::music_file::MusicRoot,
    min_path: &crate::util::FilePath,
    min_flat_clips_path: &crate::util::FilePath,
) -> Result<(), String> {
    // 楽曲情報をファイルから取得して, そのまま書き込む:
    // - minに書き込みが必要なため
    // - 既存の楽曲情報でもソートされていることを保証するため
    let content = crate::music_file::MusicRootContent::load(&root)
        .map_err(|e| e.to_pretty_string())?;

    super::common::write_all(content, min_path, min_flat_clips_path)
        .map_err(|e| e.to_pretty_string())
}
