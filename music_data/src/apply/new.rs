pub async fn apply_new(
    anonymous_videos: crate::model::AnonymousVideos,
    api_key: crate::fetcher::YouTubeApiKey,
    root: crate::music_file::MusicRoot,
    min_path: &crate::util::FilePath,
    min_flat_clips_path: &crate::util::FilePath,
) -> Result<(), String> {
    // api呼ぶ
    let video_ids = anonymous_videos.to_video_ids();
    let fetched_res = crate::fetcher::YouTubeApi::new(api_key)
        .run(video_ids)
        .await
        .map_err(|e| format!("{e}\n"))?;

    // briefsとfetched_detailくっつけてvideo_detail作成
    let details = super::common::merge_briefs_and_details(
        &anonymous_videos.to_briefs(),
        fetched_res,
    )?;

    // detailからverified clip/video作成
    let verified_videos = super::common::verify_videos(details, anonymous_videos)
        .map_err(|e| e.to_pretty_string())?;

    // 既存の音楽ファイルの情報に追加
    let mut content = crate::music_file::MusicRootContent::load(&root)
        .map_err(|e| e.to_pretty_string())?;
    content.append_videos(verified_videos).unwrap();

    // 書き出し
    super::common::write_all(content, min_path, min_flat_clips_path)
        .map_err(|e| e.to_pretty_string())?;
    Ok(())
}
