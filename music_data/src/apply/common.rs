/// 動画の概要とレスポンスを照合し, 動画の詳細情報を作成する
///
/// # Errors
/// - `String`: 指定された動画IDが存在しない場合. 整形した文字列を返却
pub(super) fn merge_briefs_and_details(
    briefs: &crate::model::VideoBriefs,
    fetch_res: crate::fetcher::VideoDetailFetchResult,
) -> Result<crate::model::VideoDetails, String> {
    fetch_res.try_into_video_details(briefs).map_err(|ids| {
        format!(
            "Specified non-existent video id(s): {}\n",
            ids.iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    })
}

/// 動画の情報をverificationする際に生じるエラー
#[derive(Debug)]
pub(super) struct VerifyVideosErrors {
    /// 動画の詳細情報が欠落している動画ID
    pub(super) missing_detail_id: Vec<crate::model::VideoId>,
    /// 動画の検証に失敗
    pub(super) verification_failed: Vec<crate::model::VerifiedVideoError>,
}

impl VerifyVideosErrors {
    pub(super) fn new() -> Self {
        Self {
            missing_detail_id: Vec::new(),
            verification_failed: Vec::new(),
        }
    }

    pub(super) fn is_empty(&self) -> bool {
        self.missing_detail_id.is_empty() && self.verification_failed.is_empty()
    }

    /// エラーメッセージを整形して返す
    ///
    /// 文字列の最後に`\n`が付与される
    pub(super) fn to_pretty_string(&self) -> String {
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
pub(super) fn verify_videos(
    mut details: crate::model::VideoDetails,
    videos: crate::model::AnonymousVideos,
) -> Result<crate::model::VerifiedVideos, VerifyVideosErrors> {
    let mut verified_videos = Vec::new();
    let mut verify_videos_errs = VerifyVideosErrors::new();

    for video in videos.into_vec() {
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
