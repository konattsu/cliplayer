pub async fn apply_new(
    anonymous_videos: crate::model::AnonymousVideos,
    api_key: crate::fetcher::YouTubeApiKey,
    root: crate::music_file::MusicRoot,
    min_path: &crate::util::FilePath,
    min_flat_clips_path: &crate::util::FilePath,
) -> Result<(), String> {
    // api呼ぶ
    let video_ids = anonymous_videos.to_video_ids();
    let fetched_res = crate::fetcher::YouTubeApi::new(api_key)
        .run(video_ids)
        .await
        .map_err(|e| format!("{e}\n"))?;

    // briefsとfetched(detail)くっつける
    let details = super::common::merge_briefs_and_details(
        &anonymous_videos.to_briefs(),
        fetched_res,
    )?;

    // detailからverified clip/video作成
    let verified_videos =
        verify_videos(details, anonymous_videos).map_err(|e| e.to_pretty_string())?;

    // begin: 既存の音楽ファイルの情報に追加
    let mut content = crate::music_file::MusicRootContent::load(&root).unwrap();
    content.append_videos(verified_videos).unwrap();

    // begin: 書き出し
    super::common::write_all(content, min_path, min_flat_clips_path)
        .map_err(|e| e.to_pretty_string())?;
    Ok(())
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

/// 動画の概要と詳細情報を照合し, 動画のクリップを検証する
fn verify_videos(
    mut details: crate::model::VideoDetails,
    videos: crate::model::AnonymousVideos,
) -> Result<crate::model::VerifiedVideos, VerifyVideosErrors> {
    let mut verified_videos = Vec::new();
    let mut verify_videos_errs = VerifyVideosErrors::new();

    for video in videos.inner.into_values() {
        if let Some(detail) = details.inner.remove(video.get_video_id()) {
            match crate::model::VerifiedVideo::from_anonymous_video(video, detail) {
                // 対応するdetailが見つかり, verificationに成功したとき
                Ok(verified) => {
                    verified_videos.push(verified);
                }
                // 対応するdetailが見つかったが, verificationに失敗したとき
                Err(e) => verify_videos_errs.verification_failed.push(e),
            }
        // 対応するdetailが見つからなかったとき
        } else {
            verify_videos_errs
                .missing_detail_id
                .push(video.get_video_id().clone());
        }
    }

    if verify_videos_errs.is_empty() {
        // 引数の`details`に対応する`VerifiedVideos`を作成しており, 元の`details`は
        // `video_id`が一意であることを保証しているため`VerifiedVideos`の`video_id`も一意
        Ok(crate::model::VerifiedVideos::try_from_vec(verified_videos)
            .expect("will not fail"))
    } else {
        Err(verify_videos_errs)
    }
}
