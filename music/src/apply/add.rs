pub async fn apply_add(
    mut music_lib: crate::music_file::MusicLibrary,
    anonymous_videos: crate::model::AnonymousVideos,
    api_key: crate::fetcher::YouTubeApiKey,
    duplicate_video_policy: crate::music_file::DuplicateVideoPolicy,
    min_clips_path: &std::path::Path,
    min_videos_path: &std::path::Path,
) -> Result<(), crate::apply::ApplyError> {
    let youtube_api = crate::fetcher::YouTubeApi::new(api_key);

    apply_add_with_fetcher(
        &mut music_lib,
        anonymous_videos,
        |video_ids| youtube_api.run(video_ids),
        duplicate_video_policy,
        min_clips_path,
        min_videos_path,
    )
    .await
}

async fn apply_add_with_fetcher<F, Fut>(
    music_lib: &mut crate::music_file::MusicLibrary,
    anonymous_videos: crate::model::AnonymousVideos,
    fetch_video_info: F,
    duplicate_video_policy: crate::music_file::DuplicateVideoPolicy,
    min_clips_path: &std::path::Path,
    min_videos_path: &std::path::Path,
) -> Result<(), crate::apply::ApplyError>
where
    F: FnOnce(crate::model::VideoIds) -> Fut,
    Fut: std::future::Future<
            Output = Result<
                crate::model::ApiVideoInfoList,
                crate::fetcher::YouTubeApiError,
            >,
        >,
{
    let video_ids = anonymous_videos.to_video_ids();
    tracing::info!("Fetching video info for new videos: {}", video_ids);
    let api_video_info_list = fetch_video_info(video_ids).await?;

    let verified_videos = crate::model::VerifiedVideos::from_anonymous_video(
        anonymous_videos,
        api_video_info_list,
    )?;

    music_lib.extend_videos(verified_videos, duplicate_video_policy)?;

    persist_add_outputs(music_lib, min_clips_path, min_videos_path)
}

fn persist_add_outputs(
    music_lib: &crate::music_file::MusicLibrary,
    min_clips_path: &std::path::Path,
    min_videos_path: &std::path::Path,
) -> Result<(), crate::apply::ApplyError> {
    music_lib.save_month_files()?;

    // `save_min_files` は所有権を取るため clone する。
    // ここでは月ファイル保存後に min ファイル生成だけを行いたい。
    super::min_file::save_min_files(music_lib.clone(), min_clips_path, min_videos_path)
        .map_err(Into::into)
}
