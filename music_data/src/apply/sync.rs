#[derive(Debug, Clone)]
pub enum SyncError {
    Fetch(crate::fetcher::YouTubeApiError),
    Save(crate::music_file::MusicFileError),
}

pub async fn apply_sync(
    music_lib: &mut crate::music_file::MusicLibrary,
    api_key: crate::fetcher::YouTubeApiKey,
) -> Result<(), SyncError> {
    let youtube_api = crate::fetcher::YouTubeApi::new(api_key);

    for music_file in music_lib.iter_files_mut() {
        let video_ids = music_file.get_video_ids();
        let details = youtube_api
            .run(video_ids)
            .await
            .map_err(|e| SyncError::Fetch(e))?;
        let new_videos = crate::model::VerifiedVideos::from_details(
            details,
            music_file.get_verified_videos(),
        )?;
        music_file.replace_videos(new_videos);
        music_file.save().map_err(|e| SyncError::Save(e))?;
    }
    music_lib.save_only_min().map_err(|e| SyncError::Save(e))?;
    Ok(())
}

// TODO 動画が削除されてfetch出来なかったときの処理追加 <- video_idミスとの区別がむずい. 手動で/**/archive/とかに移動させるとか...?

// #[derive(Debug)]
// struct VerifyVideosErrors {
//     missing_detail_id: Vec<crate::model::VideoId>,
//     verification_failed: Vec<crate::model::VerifiedVideoError>,
// }

// impl VerifyVideosErrors {
//     fn new() -> Self {
//         Self {
//             missing_detail_id: Vec::new(),
//             verification_failed: Vec::new(),
//         }
//     }

//     fn is_empty(&self) -> bool {
//         self.missing_detail_id.is_empty() && self.verification_failed.is_empty()
//     }

//     /// エラーメッセージを整形して返す
//     ///
//     /// 文字列の最後に`\n`が付与される
//     fn to_pretty_string(&self) -> String {
//         if self.is_empty() {
//             return String::new();
//         }

//         // TODO 修正
//         let mut errors = Vec::new();
//         if !self.missing_detail_id.is_empty() {
//             errors.push(format!(
//                 "Missing detail id(s). This may be a bug: {}\n",
//                 self.missing_detail_id
//                     .iter()
//                     .map(|id| id.to_string())
//                     .collect::<Vec<_>>()
//                     .join(", ")
//             ));
//         }
//         if !self.verification_failed.is_empty() {
//             errors.push(format!(
//                 "Verification failed for video(s): {}\n",
//                 self.verification_failed
//                     .iter()
//                     .map(|e| e.to_pretty_string())
//                     .collect::<Vec<_>>()
//                     .join(", ")
//             ));
//         }
//         errors.concat()
//     }
// }

// async fn part_process(
//     music_root: &crate::util::DirPath,
//     mut music_file: crate::music_file::MusicFile,
//     youtube_api: &crate::fetcher::YouTubeApi,
// ) -> Result<(), String> {
//     let fetched = youtube_api
//         .run(music_file.get_video_ids())
//         .await
//         .map_err(|e| e.to_pretty_string())?;
//     let new_details = super::common::merge_briefs_and_details(
//         &music_file.get_videos().to_briefs(),
//         fetched,
//     )?;

//     // VerifiedVideosの再検証
//     let verified_videos =
//         reverify_videos(new_details, music_file.clone().into_videos())
//             .map_err(|e| e.to_pretty_string())?;

//     music_file
//         .replace_videos(verified_videos)
//         .map_err(|e| e.to_string())?;
//     Ok(())
// }

// fn reverify_videos(
//     mut new_details: crate::model::VideoDetails,
//     videos: crate::model::VerifiedVideos,
// ) -> Result<crate::model::VerifiedVideos, VerifyVideosErrors> {
//     let mut verified_videos = Vec::new();
//     let mut verify_videos_errs = VerifyVideosErrors::new();

//     for video in videos.into_sorted_vec() {
//         if let Some(new_detail) = new_details.inner.remove(video.get_video_id()) {
//             match video.with_new_video_detail(new_detail) {
//                 // 対応するdetailが見つかり, verificationに成功したとき
//                 Ok(verified) => verified_videos.push(verified),
//                 // 対応するdetailが見つかったが, verificationに失敗したとき
//                 Err(e) => {
//                     verify_videos_errs.verification_failed.push(e);
//                 }
//             }
//         // 対応するdetailが見つからなかったとき
//         } else {
//             verify_videos_errs
//                 .missing_detail_id
//                 .push(video.get_video_id().clone());
//         }
//     }

//     if verify_videos_errs.is_empty() {
//         // 引数のvideosが`video_id`が一意であることを保証しているため
//         // `VerifiedVideos`の`video_id`も一意
//         Ok(crate::model::VerifiedVideos::try_from_vec(verified_videos)
//             .expect("will not fail"))
//     } else {
//         Err(verify_videos_errs)
//     }
// }
