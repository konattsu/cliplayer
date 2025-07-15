/// 楽曲情報を動画に書き込む
pub(super) fn write_all(
    content: crate::music_file::MusicRootContent,
    min_path: &crate::util::FilePath,
    min_flat_clips: &crate::util::FilePath,
) -> Result<(), crate::music_file::MusicFileErrors> {
    let into_errs = |e: crate::music_file::MusicFileError| e.into_errors();

    content.write()?;
    content
        .clone()
        .write_minified(min_path)
        .map_err(into_errs)?;
    content
        .write_flat_clips(min_flat_clips)
        .map_err(into_errs)?;

    Ok(())
}

/// 動画の概要とレスポンスを照合し, 動画の詳細情報を作成する
///
/// # Errors
/// - `String`: 指定された動画IDが存在しない場合. 整形した文字列を返却
pub(super) fn merge_briefs_and_details(
    briefs: &[crate::model::VideoBrief],
    fetch_res: crate::fetcher::VideoDetailFetchResult,
) -> Result<Vec<crate::model::VideoDetail>, String> {
    fetch_res.try_into_video_detail(briefs).map_err(|ids| {
        format!(
            "Specified non-existent video id(s): {}\n",
            ids.iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    })
}
