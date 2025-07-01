/// ドラフト段階の動画情報とfetchした動画情報のペア
#[derive(Debug, Clone)]
pub(super) struct DraftVideoWithFetched {
    pub(super) draft: crate::model::DraftVideo,
    pub(super) fetched: Option<super::response::YouTubeApiItem>,
}

impl DraftVideoWithFetched {
    pub(super) fn init(draft: crate::model::DraftVideo) -> Self {
        Self {
            draft,
            fetched: None,
        }
    }

    /// 最終的な結果にfinalizeする
    pub(super) fn finalize_result(self) -> crate::fetcher::FetchResult {
        use crate::fetcher::FetchResult;

        let fetched = match self.fetched {
            Some(f) => f,
            None => {
                tracing::warn!(
                    video_id = %self.draft.get_video_id(), "video not found",
                );
                return FetchResult::NotExistVideo(self.draft.into_video_id());
            }
        };

        let video_details =
            fetched.into_video_details(Some(self.draft.get_tags().clone()));

        match crate::model::FinalizedVideo::finalize_from_unidentified_clips(
            video_details,
            self.draft.into_unidentified(),
        ) {
            Ok(f) => FetchResult::Ok(f),
            Err(e) => {
                tracing::error!("failed to finalize video: {}", e);
                FetchResult::FinalizationError(e)
            }
        }
    }
}
