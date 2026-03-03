/// エラー種別。`apply_sync`のループ内で
/// - `SyncError::Continue` は当該ファイルを飛ばして次へ
/// - `SyncError::Fatal` は即時リターンを指示する
#[derive(Debug)]
enum SyncError {
    Continue(std::path::PathBuf, String),
    Fatal(String),
}

/// 音楽ライブラリのすべての月ファイルを順に YouTube API で
/// 更新し、最終的に `min` ファイル群を出力する。
#[tracing::instrument(level = tracing::Level::DEBUG, skip(music_lib))]
pub async fn apply_sync(
    mut music_lib: crate::music_file::MusicLibrary,
    api_key: crate::fetcher::YouTubeApiKey,
    min_clips_path: &std::path::Path,
    min_videos_path: &std::path::Path,
) -> Result<(), ()> {
    let youtube_api = crate::fetcher::YouTubeApi::new(api_key);

    let mut failed_files: Vec<String> = Vec::new();

    for music_file in music_lib.iter_files_mut() {
        let path_buf = music_file.get_path().to_path_buf();
        tracing::debug!("Syncing music file: {}", path_buf.display());
        match sync_one_file(music_file, &youtube_api).await {
            Ok(()) => {}
            Err(SyncError::Continue(p, msg)) => {
                failed_files.push(format!("{}: {}", p.display(), msg));
                continue;
            }
            Err(SyncError::Fatal(msg)) => {
                tracing::error!("Fatal sync error: {msg}");
                return Err(());
            }
        }
    }

    if failed_files.is_empty() {
        tracing::info!("All music files synced successfully.");
        // minファイルを更新
        super::min_file::save_min_files(music_lib, min_clips_path, min_videos_path)
            .map_err(|e| {
                tracing::error!("Failed to save min files: {e}");
            })
    } else {
        tracing::error!(
            "Some music files failed to sync:\n{}",
            failed_files.join("\n")
        );
        Err(())
    }
}

/// 単一の`MusicFile`を同期
///
/// - `Ok(())`: 成功したとき
/// - `SyncError`: 失敗したとき
#[tracing::instrument(level = tracing::Level::DEBUG, skip(youtube_api))]
async fn sync_one_file(
    music_file: &mut crate::music_file::MusicFile,
    youtube_api: &crate::fetcher::YouTubeApi,
) -> Result<(), SyncError> {
    let path_buf = music_file.get_path().to_path_buf();
    let video_ids = music_file.get_video_ids();

    let api_video_info_list = youtube_api
        .run(video_ids)
        .await
        // youtube api呼べなかったときは, 別ファイルでfetchしても
        // 失敗する可能性が高いので, 即時リターンするように戻り値を与える
        .map_err(|e| {
            let msg = format!("Failed to call YouTube API: {e}");
            tracing::error!("{msg}");
            SyncError::Fatal(msg)
        })?;

    let new_videos = music_file
        .clone()
        .into_videos()
        .with_new_api_info_list(api_video_info_list)
        .map_err(|e| {
            let msg = format!("Failed to apply API info to videos: {e}");
            tracing::error!("{msg}");
            SyncError::Continue(path_buf.clone(), msg)
        })?;

    music_file.replace_videos(new_videos).map_err(|e| {
        let msg = format!("Failed to replace videos in music file: {e}");
        tracing::error!("{msg}");
        SyncError::Continue(path_buf.clone(), msg)
    })?;

    music_file.save().map_err(|e| {
        let msg = format!("Failed to save music file: {e}");
        tracing::error!("{msg}");
        SyncError::Continue(path_buf.clone(), msg)
    })
}

// cloneやりすぎかもしれんけど一旦無視

// TODO 動画が削除されてfetch出来なかったときの処理追加 <- video_idミスとの区別がむずい. 手動で/**/archive/とかに移動させるとか...?
