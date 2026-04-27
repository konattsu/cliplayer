/// エラー種別。`apply_sync`のループ内で
/// - `SyncError::Continue` は当該ファイルを飛ばして次へ
/// - `SyncError::Fatal` は即時リターンを指示する
#[derive(Debug)]
enum SyncError {
    Continue(std::path::PathBuf, String),
    Fatal(crate::apply::ApplyError),
}

/// 音楽ライブラリのすべての月ファイルを順に YouTube API で
/// 更新し、最終的に `min` ファイル群を出力する。
#[tracing::instrument(level = tracing::Level::DEBUG, skip(music_lib))]
pub async fn apply_sync(
    music_lib: crate::music_file::MusicLibrary,
    api_key: crate::fetcher::YouTubeApiKey,
    min_clips_path: &std::path::Path,
    min_videos_path: &std::path::Path,
) -> Result<(), crate::apply::ApplyError> {
    let youtube_api = crate::fetcher::YouTubeApi::new(api_key);

    apply_sync_with_fetcher(
        music_lib,
        |video_ids| youtube_api.run(video_ids),
        min_clips_path,
        min_videos_path,
    )
    .await
}

async fn apply_sync_with_fetcher<F, Fut>(
    mut music_lib: crate::music_file::MusicLibrary,
    mut fetch_video_info: F,
    min_clips_path: &std::path::Path,
    min_videos_path: &std::path::Path,
) -> Result<(), crate::apply::ApplyError>
where
    F: FnMut(crate::model::VideoIds) -> Fut,
    Fut: std::future::Future<
            Output = Result<
                crate::model::ApiVideoInfoList,
                crate::fetcher::YouTubeApiError,
            >,
        >,
{
    let mut failed_files: Vec<String> = Vec::new();

    for music_file in music_lib.iter_files_mut() {
        let path_buf = music_file.get_path().to_path_buf();
        tracing::debug!("Syncing music file: {}", path_buf.display());
        match sync_one_file(music_file, &mut fetch_video_info).await {
            Ok(()) => {}
            Err(SyncError::Continue(p, msg)) => {
                failed_files.push(format!("{}: {}", p.display(), msg));
                continue;
            }
            Err(SyncError::Fatal(e)) => {
                return Err(e);
            }
        }
    }

    if failed_files.is_empty() {
        tracing::info!("All music files synced successfully.");
        // minファイルを更新
        super::min_file::save_min_files(music_lib, min_clips_path, min_videos_path)
            .map_err(Into::into)
    } else {
        Err(crate::apply::ApplyError::SyncPartialFailure(
            failed_files.join("\n"),
        ))
    }
}

/// 単一の`MusicFile`を同期
///
/// - `Ok(())`: 成功したとき
/// - `SyncError`: 失敗したとき
#[tracing::instrument(level = tracing::Level::DEBUG, skip(fetch_video_info))]
async fn sync_one_file<F, Fut>(
    music_file: &mut crate::music_file::MusicFile,
    fetch_video_info: &mut F,
) -> Result<(), SyncError>
where
    F: FnMut(crate::model::VideoIds) -> Fut,
    Fut: std::future::Future<
            Output = Result<
                crate::model::ApiVideoInfoList,
                crate::fetcher::YouTubeApiError,
            >,
        >,
{
    let path_buf = music_file.get_path().to_path_buf();
    let video_ids = music_file.get_video_ids();

    let api_video_info_list = fetch_video_info(video_ids)
        .await
        // youtube api呼べなかったときは, 別ファイルでfetchしても
        // 失敗する可能性が高いので, 即時リターンするように戻り値を与える
        .map_err(|e| SyncError::Fatal(e.into()))?;

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

#[cfg(test)]
mod tests {
    use super::*;

    const MONTH_2024_01_JSON: &str = r#"[
    {
        "videoId": "11111111111",
        "title": "sync test video 1",
        "channelId": "UCivwPlOp0ojnMPZj5pNOPPA",
        "publishedAt": "2024-01-01T01:01:01Z",
        "syncedAt": "2025-01-01T01:01:01Z",
        "duration": "PT1H0M0S",
        "privacyStatus": "public",
        "embeddable": true,
        "videoTags": ["karaoke"],
        "clips": [
            {
                "songTitle": "song-1",
                "liverIds": ["riku-tazumi"],
                "startTime": "PT1M0S",
                "endTime": "PT2M0S",
                "uuid": "11786ebd-4b42-428b-81f8-ecf791887326"
            }
        ]
    }
]
"#;

    const MONTH_2024_02_JSON: &str = r#"[
    {
        "videoId": "22222222222",
        "title": "sync test video 2",
        "channelId": "UC7_MFM9b8hp5kuTSpa8WyOQ",
        "publishedAt": "2024-02-02T02:02:02Z",
        "syncedAt": "2025-01-01T01:01:01Z",
        "duration": "PT1H0M0S",
        "privacyStatus": "public",
        "embeddable": true,
        "videoTags": ["karaoke"],
        "clips": [
            {
                "songTitle": "song-2",
                "liverIds": ["yugamin"],
                "startTime": "PT2M0S",
                "endTime": "PT3M0S",
                "uuid": "3fc31423-b991-4c88-8de7-71d2ed9b50c5"
            }
        ]
    }
]
"#;

    fn write_month_file(root: &std::path::Path, year: usize, month: usize, json: &str) {
        let path = root.join(format!("{year:04}/{month:02}.json"));
        std::fs::create_dir_all(path.parent().expect("parent dir exists")).unwrap();
        std::fs::write(path, json).unwrap();
    }

    fn build_music_library(root: &std::path::Path) -> crate::music_file::MusicLibrary {
        crate::music_file::MusicLibrary::load(root).unwrap()
    }

    fn api_info_for_id(id: &crate::model::VideoId) -> crate::model::ApiVideoInfo {
        use chrono::TimeZone;

        let published_at = if *id == crate::model::VideoId::test_id_1() {
            crate::model::VideoPublishedAt::self_1()
        } else {
            crate::model::VideoPublishedAt::self_2()
        };

        crate::model::ApiVideoInfoInitializer {
            video_id: id.clone(),
            title: format!("synced-{id}"),
            channel_id: crate::model::ChannelId::test_id_1(),
            published_at,
            synced_at: chrono::Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap(),
            duration: crate::model::Duration::from_secs_u16(3600),
            privacy_status: crate::model::PrivacyStatus::Public,
            embeddable: true,
        }
        .init()
    }

    #[tokio::test]
    async fn test_apply_sync_with_fetcher_success() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();

        write_month_file(root, 2024, 1, MONTH_2024_01_JSON);
        write_month_file(root, 2024, 2, MONTH_2024_02_JSON);

        let min_clips = root.join("clips.min.json");
        let min_videos = root.join("videos.min.json");

        let lib = build_music_library(root);
        let res = apply_sync_with_fetcher(
            lib,
            |video_ids| async move {
                let infos = video_ids
                    .into_vec()
                    .into_iter()
                    .map(|id| api_info_for_id(&id))
                    .collect::<Vec<_>>();
                Ok(crate::model::ApiVideoInfoList::from_vec_ignore_duplicated(
                    infos,
                ))
            },
            &min_clips,
            &min_videos,
        )
        .await;

        assert!(res.is_ok());
        assert!(min_clips.exists());
        assert!(min_videos.exists());
    }

    #[tokio::test]
    async fn test_apply_sync_with_fetcher_fatal_network_error() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();

        write_month_file(root, 2024, 1, MONTH_2024_01_JSON);

        let min_clips = root.join("clips.min.json");
        let min_videos = root.join("videos.min.json");

        let lib = build_music_library(root);
        let res = apply_sync_with_fetcher(
            lib,
            |_video_ids| async {
                Err(crate::fetcher::YouTubeApiError::NetworkError(
                    "network down".to_string(),
                ))
            },
            &min_clips,
            &min_videos,
        )
        .await;

        assert!(matches!(
            res,
            Err(crate::apply::ApplyError::YouTubeApi(
                crate::fetcher::YouTubeApiError::NetworkError(_)
            ))
        ));
    }

    #[tokio::test]
    async fn test_apply_sync_with_fetcher_partial_failure() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();

        write_month_file(root, 2024, 1, MONTH_2024_01_JSON);

        let min_clips = root.join("clips.min.json");
        let min_videos = root.join("videos.min.json");

        let lib = build_music_library(root);
        let res = apply_sync_with_fetcher(
            lib,
            |_video_ids| async {
                Ok(crate::model::ApiVideoInfoList::from_vec_ignore_duplicated(
                    Vec::new(),
                ))
            },
            &min_clips,
            &min_videos,
        )
        .await;

        match res {
            Err(crate::apply::ApplyError::SyncPartialFailure(message)) => {
                assert!(message.contains("Failed to apply API info to videos"));
            }
            other => panic!("expected SyncPartialFailure, got {other:?}"),
        }
    }
}

// cloneやりすぎかもしれんけど一旦無視

// TODO 動画が削除されてfetch出来なかったときの処理追加 <- video_idミスとの区別がむずい. 手動で/**/archive/とかに移動させるとか...?
