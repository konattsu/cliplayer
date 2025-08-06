pub async fn apply_sync(
    mut music_lib: crate::music_file::MusicLibrary,
    api_key: crate::fetcher::YouTubeApiKey,
) -> Result<(), String> {
    let youtube_api = crate::fetcher::YouTubeApi::new(api_key);

    for music_file in music_lib.iter_files_mut() {
        tracing::debug!("Syncing music file: {}", music_file.get_path());
        let video_ids = music_file.get_video_ids();

        let api_video_info_list = youtube_api
            .run(video_ids)
            .await
            .map_err(|e| e.to_pretty_string())?;

        let new_videos = music_file
            .clone()
            .into_videos()
            .with_new_api_info_list(api_video_info_list)
            .map_err(|e| e.to_pretty_string())?;

        music_file
            .replace_videos(new_videos)
            .map_err(|e| e.to_pretty_string())?;
        music_file.save().map_err(|e| e.to_pretty_string())?;
    }

    music_lib
        .save_only_min()
        .map_err(|e| e.to_pretty_string())?;
    Ok(())
}

// cloneやりすぎかもしれんけど一旦無視

// TODO 動画が削除されてfetch出来なかったときの処理追加 <- video_idミスとの区別がむずい. 手動で/**/archive/とかに移動させるとか...?
