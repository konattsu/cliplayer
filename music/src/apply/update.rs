/// 既存の楽曲情報に対する更新を適用
///
/// # Returns
/// - `Ok(())`: 正常に更新が適用された場合
/// - `Err(_)`: エラーが発生した場合
pub fn apply_update(
    music_lib: crate::music_file::MusicLibrary,
) -> Result<(), crate::apply::ApplyError> {
    crate::music_file::MusicLibraryRepository::save_month_files(&music_lib)?;
    Ok(())
}
