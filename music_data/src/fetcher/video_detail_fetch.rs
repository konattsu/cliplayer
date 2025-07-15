/// VideoIdとそれに基づくVideoDetailWthoutTagsのペアを格納する構造体
#[derive(Debug, Clone)]
pub struct VideoDetailFetchResult(
    pub std::collections::HashMap<crate::model::VideoId, Option<FetchedVideoDetail>>,
);

impl FromIterator<(crate::model::VideoId, Option<FetchedVideoDetail>)>
    for VideoDetailFetchResult
{
    fn from_iter<
        I: IntoIterator<Item = (crate::model::VideoId, Option<FetchedVideoDetail>)>,
    >(
        iter: I,
    ) -> Self {
        let map = iter.into_iter().collect();
        Self(map)
    }
}

impl VideoDetailFetchResult {
    /// 渡されたVideoBriefのリストと, fetchしてきたデータから, 適切なVideoDetailを取得する
    ///
    /// # Arguments:
    /// - `video_briefs`: VideoDetailを取得するためのVideoBriefのリスト
    ///
    /// # Returns:
    /// - `Ok`: Briefを昇格させたVideoDetailのリスト
    /// - `Err`: 存在しないVideoIdのリスト
    ///   - 1つでも存在しないVideoIdが含まれている場合はErrを返す
    pub fn try_into_video_detail(
        mut self,
        video_briefs: &[crate::model::VideoBrief],
    ) -> Result<Vec<crate::model::VideoDetail>, Vec<crate::model::VideoId>> {
        let mut details = Vec::new();
        let mut non_exist_ids = Vec::new();

        for video_brief in video_briefs {
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
                // 設計,呼び出しが正しければunreachableなはずなのでwarnだしておく
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
            Ok(details)
        } else {
            Err(non_exist_ids)
        }
    }
}

/// VideoDetailのうちfetchできる情報を集めたもの
#[derive(Debug, Clone)]
pub struct FetchedVideoDetail {
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

pub struct FetchedVideoDetailInitializer {
    /// 動画ID
    pub video_id: crate::model::VideoId,
    /// 動画のタイトル
    pub title: String,
    /// チャンネルID
    pub channel_id: crate::model::ChannelId,
    /// 動画の公開日時
    pub published_at: crate::model::VideoPublishedAt,
    /// この情報を取得した日時
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// 動画の長さ
    pub duration: crate::model::Duration,
    /// 動画のプライバシー設定
    pub privacy_status: crate::model::PrivacyStatus,
    /// 動画が埋め込み可能かどうか
    pub embeddable: bool,
}

impl FetchedVideoDetailInitializer {
    pub fn init(self) -> FetchedVideoDetail {
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
