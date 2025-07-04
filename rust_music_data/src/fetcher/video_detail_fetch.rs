/// VideoIdとそれに基づくVideoDetailWthoutTagsのペアを格納する構造体
#[derive(Debug, Clone)]
pub struct VideoDetailFetchResult(
    pub std::collections::HashMap<crate::model::VideoId, Option<VideoDetailWithoutTags>>,
);

impl FromIterator<(crate::model::VideoId, Option<VideoDetailWithoutTags>)>
    for VideoDetailFetchResult
{
    fn from_iter<
        I: IntoIterator<Item = (crate::model::VideoId, Option<VideoDetailWithoutTags>)>,
    >(
        iter: I,
    ) -> Self {
        let map = iter.into_iter().collect();
        Self(map)
    }
}

/// VideoDetailのタグが無い版
#[derive(Debug, Clone)]
pub struct VideoDetailWithoutTags {
    /// 動画ID
    video_id: crate::model::VideoId,
    /// 動画のタイトル
    title: String,
    /// チャンネルID
    channel_id: crate::model::ChannelId,
    /// チャンネル名
    channel_name: crate::model::ChannelName,
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

pub struct VideoDetailWithoutTagsInitializer {
    /// 動画ID
    pub video_id: crate::model::VideoId,
    /// 動画のタイトル
    pub title: String,
    /// チャンネルID
    pub channel_id: crate::model::ChannelId,
    /// チャンネル名
    pub channel_name: crate::model::ChannelName,
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

impl VideoDetailWithoutTagsInitializer {
    pub fn init(self) -> VideoDetailWithoutTags {
        VideoDetailWithoutTags {
            video_id: self.video_id,
            title: self.title,
            channel_id: self.channel_id,
            channel_name: self.channel_name,
            published_at: self.published_at,
            modified_at: self.modified_at,
            duration: self.duration,
            privacy_status: self.privacy_status,
            embeddable: self.embeddable,
        }
    }
}

impl VideoDetailWithoutTags {
    pub fn into_video_detail(
        self,
        tags: crate::model::TagList,
    ) -> crate::model::VideoDetail {
        crate::model::VideoDetailInitializer {
            video_id: self.video_id,
            title: self.title,
            channel_id: self.channel_id,
            channel_name: self.channel_name,
            published_at: self.published_at,
            modified_at: self.modified_at,
            duration: self.duration,
            privacy_status: self.privacy_status,
            embeddable: self.embeddable,
            tags,
        }
        .init()
    }
}
