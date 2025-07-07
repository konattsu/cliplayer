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
    pub fn into_video_detail(
        self,
        video_brief: &crate::model::VideoBrief,
    ) -> crate::model::VideoDetail {
        crate::model::VideoDetailInitializer {
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
        .init()
    }
}
