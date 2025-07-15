pub async fn apply_sync(
    api_key: crate::fetcher::YouTubeApiKey,
    root: crate::music_file::MusicRoot,
    min_path: &crate::util::FilePath,
    min_flat_clips_path: &crate::util::FilePath,
) -> Result<(), String> {
    // あとで作る
    todo!()
}

// 定期的にファイルに書き込む, modified_at活用する
