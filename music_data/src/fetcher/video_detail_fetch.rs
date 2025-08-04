/// VideoIdとそれに基づくFetchedVideoDetailのペアを格納する構造体
#[derive(Debug, Clone)]
pub(crate) struct VideoApiFetchResult(
    pub(crate)  std::collections::HashMap<
        crate::model::VideoId,
        Option<crate::model::ApiVideoInfo>,
    >,
);

impl VideoApiFetchResult {
    pub(super) fn new(video_ids: crate::model::VideoIds) -> Self {
        VideoApiFetchResult(video_ids.into_iter().map(|id| (id, None)).collect())
    }

    /// 渡されたVideoBriefのリストと, fetchしてきたデータから, 適切なVideoDetailを取得する
    ///
    /// # Arguments:
    /// - `video_briefs`: VideoDetailを取得するためのVideoBrief
    ///
    /// # Returns:
    /// - `Ok`: Briefを昇格させたVideoDetailのリスト
    /// - `Err`: 存在しないVideoIdのリスト
    ///   - 1つでも存在しないVideoIdが含まれている場合はErrを返す
    pub(crate) fn try_into_video_details(
        mut self,
        video_briefs: &crate::model::VideoBriefs,
    ) -> Result<crate::model::VideoDetails, Vec<crate::model::VideoId>> {
        let mut details = Vec::new();
        let mut non_exist_ids = Vec::new();

        for video_brief in video_briefs.inner.values() {
            match self.0.remove(video_brief.get_video_id()) {
                Some(detail) => match detail {
                    // 存在するvideo_idを指定した場合
                    Some(d) => {
                        details.push(
                            // 両者のvideo_idが必ず一致するためunwrapで落とす
                            d.try_into_video_detail(video_brief).expect("Impl Err"),
                        );
                    }
                    // 存在しないvideo_idを指定した場合
                    None => non_exist_ids.push(video_brief.get_video_id().clone()),
                },
                // このvideo_idをfetchするように指定しなかった場合
                // 設計/呼び出しが正しければunreachableなはずなのでwarnだしておく
                None => {
                    tracing::warn!(
                        "VideoBrief (video_id: {}) was passed to `try_into_video_detail`, \
                        but this video_id was not included in the fetch targets. \
                        This should be unreachable if the design and call are correct.",
                        video_brief.get_video_id()
                    );
                    // 呼び出し元は全てのVideoBrief(引数)に対応したVideoDetail(戻り値)を
                    // 期待しているので, その戻り値を返せない=>存在しないidとして扱う
                    non_exist_ids.push(video_brief.get_video_id().clone())
                }
            }
        }

        if non_exist_ids.is_empty() {
            Ok(crate::model::VideoDetails::try_from_vec(details).expect(
                "Impl Err: VideoDetails should be created from a non-empty Vec",
            ))
        } else {
            Err(non_exist_ids)
        }
    }
}

/// VideoDetailのうちfetchできる情報を集めたもの
#[derive(Debug, Clone)]
pub(super) struct FetchedVideoDetail {
    /// 動画ID
    video_id: crate::model::VideoId,
    /// 動画のタイトル
    title: String,
    /// チャンネルID
    channel_id: crate::model::ChannelId,
    /// 動画の公開日時
    published_at: crate::model::VideoPublishedAt,
    /// この情報を取得した日時
    modified_at: chrono::DateTime<chrono::Utc>,
    /// 動画の長さ
    duration: crate::model::Duration,
    /// 動画のプライバシー設定
    privacy_status: crate::model::PrivacyStatus,
    /// 動画が埋め込み可能かどうか
    embeddable: bool,
}

pub(super) struct FetchedVideoDetailInitializer {
    /// 動画ID
    pub(super) video_id: crate::model::VideoId,
    /// 動画のタイトル
    pub(super) title: String,
    /// チャンネルID
    pub(super) channel_id: crate::model::ChannelId,
    /// 動画の公開日時
    pub(super) published_at: crate::model::VideoPublishedAt,
    /// この情報を取得した日時
    pub(super) modified_at: chrono::DateTime<chrono::Utc>,
    /// 動画の長さ
    pub(super) duration: crate::model::Duration,
    /// 動画のプライバシー設定
    pub(super) privacy_status: crate::model::PrivacyStatus,
    /// 動画が埋め込み可能かどうか
    pub(super) embeddable: bool,
}

impl FetchedVideoDetailInitializer {
    pub(super) fn init(self) -> FetchedVideoDetail {
        FetchedVideoDetail {
            video_id: self.video_id,
            title: self.title,
            channel_id: self.channel_id,
            published_at: self.published_at,
            modified_at: self.modified_at,
            duration: self.duration,
            privacy_status: self.privacy_status,
            embeddable: self.embeddable,
        }
    }
}

impl FetchedVideoDetail {
    fn try_into_video_detail(
        self,
        video_brief: &crate::model::VideoBrief,
    ) -> Result<crate::model::VideoDetail, String> {
        if video_brief.get_video_id() != &self.video_id {
            return Err(format!(
                "VideoBrief's video_id ({}) does not match FetchedVideoDetail's video_id ({})",
                video_brief.get_video_id(),
                self.video_id
            ));
        }

        Ok(crate::model::VideoDetailInitializer {
            video_id: self.video_id,
            title: self.title,
            channel_id: self.channel_id,
            uploader_name: video_brief.get_uploader_name().cloned(),
            published_at: self.published_at,
            modified_at: self.modified_at,
            duration: self.duration,
            privacy_status: self.privacy_status,
            embeddable: self.embeddable,
            video_tags: video_brief.get_tags().clone(),
        }
        .init())
    }
}
