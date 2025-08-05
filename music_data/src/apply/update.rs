/// 既存の楽曲情報に対する更新を適用
///
/// # Returns
/// - `Ok(())`: 正常に更新が適用された場合
/// - `Err(String)`: エラーが発生した場合の整形されたエラーメッセージ
pub fn apply_update(music_lib: crate::music_file::MusicLibrary) -> Result<(), String> {
    // 楽曲情報をファイルから取得して, そのまま書き込む:
    // - minに書き込みが必要なため
    // - 既存の楽曲情報でもソートされていることを保証するため

    music_lib
        .save()
        .map_err(|e| format!("Failed to save music files: {e}"))
}
