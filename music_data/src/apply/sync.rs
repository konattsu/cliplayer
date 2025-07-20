pub async fn apply_sync(
    api_key: crate::fetcher::YouTubeApiKey,
    root: crate::music_file::MusicRoot,
    min_path: &crate::util::FilePath,
    min_flat_clips_path: &crate::util::FilePath,
) -> Result<(), String> {
    let stale_contents = crate::music_file::MusicRootContent::load(&root)
        .map_err(|e| e.to_pretty_string())?;
    let youtube_api = crate::fetcher::YouTubeApi::new(api_key);
    let mut new_contents = Vec::new();

    // ファイル毎に実行
    for stale_content in stale_contents.into_inner() {
        new_contents.push(part_process(stale_content, &youtube_api).await?);
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

async fn part_process(
    inner: crate::music_file::MusicFileEntryWithVideosInner,
    youtube_api: &crate::fetcher::YouTubeApi,
) -> Result<crate::music_file::MusicFileEntryWithVideosInner, String> {
    let stale_details: Vec<crate::model::VideoDetail> = inner
        .videos
        .inner
        .values()
        .map(|v| v.get_detail())
        .cloned()
        .collect();
    let stale_details =
        crate::model::VideoDetails::try_from_vec(stale_details).expect("will not fail");
    let ids = stale_details.inner.keys().cloned().collect::<Vec<_>>();
    let fetched = youtube_api
        .run(ids)
        .await
        .map_err(|e| e.to_pretty_string())?;
    let new_details =
        super::common::merge_briefs_and_details(&stale_details.into_briefs(), fetched)?;

    let verified_videos =
        reverify_videos(new_details, inner.videos).map_err(|e| e.to_pretty_string())?;

    Ok(crate::music_file::MusicFileEntryWithVideosInner {
        videos: verified_videos,
        ..inner
    })
}

fn reverify_videos(
    mut new_details: crate::model::VideoDetails,
    videos: crate::model::VerifiedVideos,
) -> Result<crate::model::VerifiedVideos, VerifyVideosErrors> {
    let mut verified_videos = Vec::new();
    let mut verify_videos_errs = VerifyVideosErrors::new();

    for video in videos.into_sorted_vec() {
        if let Some(new_detail) = new_details.inner.remove(video.get_video_id()) {
            match video.with_new_video_detail(new_detail) {
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
        // 引数のvideosが`video_id`が一意であることを保証しているため
        // `VerifiedVideos`の`video_id`も一意
        Ok(crate::model::VerifiedVideos::try_from_vec(verified_videos)
            .expect("will not fail"))
    } else {
        Err(verify_videos_errs)
    }
}
