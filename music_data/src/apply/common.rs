/// 楽曲情報を動画に書き込む
pub(super) fn write_all(
    content: crate::music_file::MusicRootContent,
    min_path: &crate::util::FilePath,
    min_flat_clips: &crate::util::FilePath,
) -> Result<(), crate::music_file::MusicFileErrors> {
    let into_errs = |e: crate::music_file::MusicFileError| e.into_errors();

    content.write()?;
    content
        .clone()
        .write_minified(min_path)
        .map_err(into_errs)?;
    content
        .write_flat_clips(min_flat_clips)
        .map_err(into_errs)?;

    Ok(())
}

/// 動画の概要とレスポンスを照合し, 動画の詳細情報を作成する
///
/// # Errors
/// - `String`: 指定された動画IDが存在しない場合. 整形した文字列を返却
pub(super) fn merge_briefs_and_details(
    briefs: &crate::model::VideoBriefs,
    fetch_res: crate::fetcher::VideoDetailFetchResult,
) -> Result<crate::model::VideoDetails, String> {
    fetch_res.try_into_video_detail(briefs).map_err(|ids| {
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
    missing_detail_id: Vec<crate::model::VideoId>,
    /// 動画の検証に失敗
    verification_failed: Vec<crate::model::VerifiedVideoError>,
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
