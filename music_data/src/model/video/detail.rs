/// 動画の詳細情報
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct VideoDetail {
    /// 動画ID
    video_id: crate::model::VideoId,
    /// 動画のタイトル
    title: String,
    /// チャンネルID
    channel_id: crate::model::ChannelId,
    /// チャンネル名, 箱外で行われた配信/動画の時に付与
    #[serde(skip_serializing_if = "Option::is_none")]
    uploader_name: Option<crate::model::UploaderName>,
    /// 動画の公開日時
    published_at: crate::model::VideoPublishedAt,
    /// この情報を取得した日時
    #[serde(with = "crate::util::datetime_serde")]
    modified_at: chrono::DateTime<chrono::Utc>,
    /// 動画の長さ
    duration: crate::model::Duration,
    /// 動画のプライバシー設定
    privacy_status: crate::model::PrivacyStatus,
    /// 動画が埋め込み可能かどうか
    embeddable: bool,
    /// 動画のタグ
    video_tags: crate::model::VideoTags,
}

/// 動画の詳細情報のリスト
///
/// 各`VideoDetail`に含まれる動画idが一意であることを保証
#[derive(Debug, Clone)]
pub struct VideoDetails {
    pub inner: std::collections::HashMap<crate::model::VideoId, VideoDetail>,
}

pub struct VideoDetailInitializer {
    /// 動画ID
    pub video_id: crate::model::VideoId,
    /// 動画のタイトル
    pub title: String,
    /// チャンネルID
    pub channel_id: crate::model::ChannelId,
    /// チャンネル名, 箱外で行われた配信/動画の時に付与
    pub uploader_name: Option<crate::model::UploaderName>,
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
    /// 動画のタグ
    pub video_tags: crate::model::VideoTags,
}

impl VideoDetailInitializer {
    pub fn init(self) -> VideoDetail {
        VideoDetail {
            video_id: self.video_id,
            title: self.title,
            channel_id: self.channel_id,
            uploader_name: self.uploader_name,
            published_at: self.published_at,
            modified_at: self.modified_at,
            duration: self.duration,
            privacy_status: self.privacy_status,
            embeddable: self.embeddable,
            video_tags: self.video_tags,
        }
    }
}

impl VideoDetail {
    pub fn get_published_at(&self) -> &crate::model::VideoPublishedAt {
        &self.published_at
    }
    pub fn get_duration(&self) -> &crate::model::Duration {
        &self.duration
    }
    pub fn get_video_id(&self) -> &crate::model::VideoId {
        &self.video_id
    }
}

impl VideoDetails {
    /// `VideoDetail`のリストを`VideoDetails`に変換
    ///
    /// Err: 動画idが重複しているとき
    pub fn try_from_vec(
        details: Vec<VideoDetail>,
    ) -> Result<Self, Vec<crate::model::VideoId>> {
        use std::collections::{HashMap, HashSet};

        let mut inner = HashMap::new();
        let mut duplicated_ids = HashSet::new();

        for detail in details {
            if let Some(prev_detail) =
                inner.insert(detail.get_video_id().clone(), detail)
            {
                // 重複の有無のみ検出したく, すでに重複しているか(3回,同じ動画IDが来たとき)どうかは
                // 気にしないのでinsertの結果は無視
                let _res = duplicated_ids.insert(prev_detail.get_video_id().clone());
            }
        }

        if duplicated_ids.is_empty() {
            Ok(Self { inner })
        } else {
            Err(duplicated_ids.into_iter().collect())
        }
    }
}
