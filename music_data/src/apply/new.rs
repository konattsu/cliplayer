pub async fn apply_new(
    anonymous_videos: crate::model::AnonymousVideos,
    api_key: crate::fetcher::YouTubeApiKey,
    mut music_lib: crate::music_file::MusicLibrary,
) -> Result<(), String> {
    // api呼ぶ
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

    // 書き出し
    music_lib.save().map_err(|e| e.to_pretty_string())?;

    Ok(())
}
