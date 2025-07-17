pub async fn apply_sync(
    api_key: crate::fetcher::YouTubeApiKey,
    root: crate::music_file::MusicRoot,
    min_path: &crate::util::FilePath,
    min_flat_clips_path: &crate::util::FilePath,
) -> Result<(), String> {
    let contents = crate::music_file::MusicRootContent::load(&root)
        .map_err(|e| e.to_pretty_string())?;

    let youtube_api = crate::fetcher::YouTubeApi::new(api_key);

    for content in contents.into_inner() {
        let videos = content.videos;

        //
    }

    // あとで作る
    todo!()
}

#[derive(Debug)]
struct VerifyVideosErrors {
    missing_detail_id: Vec<crate::model::VideoId>,
    verification_failed: Vec<crate::model::VerifiedVideoError>,
}

impl VerifyVideosErrors {
    fn new() -> Self {
        Self {
            missing_detail_id: Vec::new(),
            verification_failed: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.missing_detail_id.is_empty() && self.verification_failed.is_empty()
    }

    /// エラーメッセージを整形して返す
    ///
    /// 文字列の最後に`\n`が付与される
    fn to_pretty_string(&self) -> String {
        if self.is_empty() {
            return String::new();
        }

        let mut errors = Vec::new();
        if !self.missing_detail_id.is_empty() {
            errors.push(format!(
                "Missing detail id(s). This may be a bug: {}\n",
                self.missing_detail_id
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        if !self.verification_failed.is_empty() {
            errors.push(format!(
                "Verification failed for video(s): {}\n",
                self.verification_failed
                    .iter()
                    .map(|e| e.to_pretty_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        errors.concat()
    }
}

// 定期的にファイルに書き込む, modified_at活用する

fn reverify_videos(
    details: Vec<crate::model::VideoDetail>,
    videos: crate::model::VerifiedVideos,
) -> Result<crate::model::VerifiedVideos, VerifyVideosErrors> {
    let mut verified_videos = Vec::new();
    let mut verify_videos_errs = VerifyVideosErrors::new();

    // O(n^2)を避けるため, VideoIdをキーにしてdetailにO(1)でアクセスできるようにする
    let mut details: std::collections::HashMap<_, _> = details
        .into_iter()
        .map(|d| (d.get_video_id().clone(), d))
        .collect();

    for video in videos.into_inner() {
        if let Some(detail) = details.remove(video.get_video_id()) {
            match video.with_new_video_detail(detail) {
                // 対応するdetailが見つかり, verificationに成功したとき
                Ok(verified) => verified_videos.push(verified),
                // 対応するdetailが見つかったが, verificationに失敗したとき
                Err(e) => {
                    verify_videos_errs.verification_failed.push(e);
                }
            }
        // 対応するdetailが見つからなかったとき
        } else {
            verify_videos_errs
                .missing_detail_id
                .push(video.get_video_id().clone());
        }
    }

    if verify_videos_errs.is_empty() {
        Ok(crate::model::VerifiedVideos::new(verified_videos))
    } else {
        Err(verify_videos_errs)
    }
}
