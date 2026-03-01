pub async fn apply_new(
    mut music_lib: crate::music_file::MusicLibrary,
    anonymous_videos: crate::model::AnonymousVideos,
    api_key: crate::fetcher::YouTubeApiKey,
    min_clips_path: &std::path::Path,
    min_videos_path: &std::path::Path,
) -> Result<(), String> {
    // api呼ぶ
    tracing::info!(
        "Fetching video info for new videos: {:?}",
        anonymous_videos.to_video_ids()
    );
    let video_ids = anonymous_videos.to_video_ids();
    let api_video_info_list = crate::fetcher::YouTubeApi::new(api_key)
        .run(video_ids)
        .await
        .map_err(|e| e.to_pretty_string())?;

    let verified_videos = crate::model::VerifiedVideos::from_anonymous_video(
        anonymous_videos,
        api_video_info_list,
    )
    .map_err(|e| e.to_pretty_string())?;

    // 既存の音楽ファイルの情報に追加
    music_lib
        .extend_videos(verified_videos)
        .map_err(|e| e.to_pretty_string())?;

    // データベースを更新
    music_lib
        .save_month_files()
        .map_err(|e| e.to_pretty_string())?;

    // minファイルを更新
    super::min_file::save_min_files(music_lib, min_clips_path, min_videos_path)
        .map_err(|e| e.to_pretty_string())
}
