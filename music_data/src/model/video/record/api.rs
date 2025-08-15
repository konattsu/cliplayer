/// apiから得られる動画の詳細情報
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct ApiVideoInfo {
    /// 動画ID
    pub(super) video_id: crate::model::VideoId,
    /// 動画のタイトル
    pub(super) title: String,
    /// チャンネルID
    pub(super) channel_id: crate::model::ChannelId,
    /// 動画の公開日時
    pub(super) published_at: crate::model::VideoPublishedAt,
    /// この情報を取得した日時
    #[serde(with = "crate::util::datetime_serde")]
    pub(super) synced_at: chrono::DateTime<chrono::Utc>,
    /// 動画の長さ
    pub(super) duration: crate::model::Duration,
    /// 動画のプライバシー設定
    pub(super) privacy_status: crate::model::PrivacyStatus,
    /// 動画が埋め込み可能かどうか
    pub(super) embeddable: bool,
}

/// apiから得られる動画の詳細情報のリスト
///
/// 各`ApiVideoInfo`に含まれる動画idが一意であることを保証
#[derive(Debug, Clone)]
pub(crate) struct ApiVideoInfoList {
    pub(crate) inner: std::collections::HashMap<crate::model::VideoId, ApiVideoInfo>,
}

pub(crate) struct ApiVideoInfoInitializer {
    /// 動画ID
    pub(crate) video_id: crate::model::VideoId,
    /// 動画のタイトル
    pub(crate) title: String,
    /// チャンネルID
    pub(crate) channel_id: crate::model::ChannelId,
    /// 動画の公開日時
    pub(crate) published_at: crate::model::VideoPublishedAt,
    /// この情報を取得した日時
    pub(crate) synced_at: chrono::DateTime<chrono::Utc>,
    /// 動画の長さ
    pub(crate) duration: crate::model::Duration,
    /// 動画のプライバシー設定
    pub(crate) privacy_status: crate::model::PrivacyStatus,
    /// 動画が埋め込み可能かどうか
    pub(crate) embeddable: bool,
}

impl ApiVideoInfoInitializer {
    pub(crate) fn init(self) -> ApiVideoInfo {
        ApiVideoInfo {
            video_id: self.video_id,
            title: self.title,
            channel_id: self.channel_id,
            published_at: self.published_at,
            synced_at: self.synced_at,
            duration: self.duration,
            privacy_status: self.privacy_status,
            embeddable: self.embeddable,
        }
    }
}

impl ApiVideoInfo {
    pub(crate) fn get_video_id(&self) -> &crate::model::VideoId {
        &self.video_id
    }
    pub(crate) fn get_title(&self) -> &str {
        &self.title
    }
    pub(crate) fn get_channel_id(&self) -> &crate::model::ChannelId {
        &self.channel_id
    }
    pub(crate) fn get_published_at(&self) -> &crate::model::VideoPublishedAt {
        &self.published_at
    }
    pub(crate) fn get_synced_at(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.synced_at
    }
    pub(crate) fn get_duration(&self) -> &crate::model::Duration {
        &self.duration
    }
    pub(crate) fn get_privacy_status(&self) -> &crate::model::PrivacyStatus {
        &self.privacy_status
    }
    pub(crate) fn is_embeddable(&self) -> bool {
        self.embeddable
    }

    /// `synced_at`を除いて他のフィールドが一致するか比較
    pub(crate) fn is_same_except_synced_at(&self, other: &ApiVideoInfo) -> bool {
        self.video_id == other.video_id
            && self.title == other.title
            && self.channel_id == other.channel_id
            && self.published_at == other.published_at
            && self.duration == other.duration
            && self.privacy_status == other.privacy_status
            && self.embeddable == other.embeddable
    }
}

impl ApiVideoInfoList {
    /// `ApiVideoInfo`のリストから重複を除いて`ApiVideoInfoList`を生成
    pub(crate) fn from_vec_ignore_duplicated(details: Vec<ApiVideoInfo>) -> Self {
        Self {
            inner: details
                .into_iter()
                .map(|api_info| (api_info.video_id.clone(), api_info))
                .collect(),
        }
    }
}

// MARK: For Tests

#[cfg(test)]
impl ApiVideoInfo {
    pub(crate) fn self_a() -> Self {
        use chrono::TimeZone;
        ApiVideoInfoInitializer {
            video_id: crate::model::VideoId::test_id_1(),
            title: "Test Video A".to_string(),
            channel_id: crate::model::ChannelId::test_id_1(),
            published_at: crate::model::VideoPublishedAt::self_1(),
            synced_at: chrono::Utc.with_ymd_and_hms(2025, 1, 1, 1, 1, 1).unwrap(),
            duration: crate::model::Duration::self_3(),
            privacy_status: crate::model::PrivacyStatus::Public,
            embeddable: true,
        }
        .init()
    }

    pub(crate) fn self_b() -> Self {
        use chrono::TimeZone;
        ApiVideoInfoInitializer {
            video_id: crate::model::VideoId::test_id_2(),
            title: "Test Video B".to_string(),
            channel_id: crate::model::ChannelId::test_id_2(),
            published_at: crate::model::VideoPublishedAt::self_2(),
            synced_at: chrono::Utc.with_ymd_and_hms(2025, 7, 7, 7, 7, 7).unwrap(),
            duration: crate::model::Duration::self_2(),
            privacy_status: crate::model::PrivacyStatus::Private,
            embeddable: false,
        }
        .init()
    }

    pub(crate) fn update_synced_at(self, new: chrono::DateTime<chrono::Utc>) -> Self {
        Self {
            synced_at: new,
            ..self
        }
    }

    pub(crate) fn set_duration(self, duration: crate::model::Duration) -> Self {
        Self { duration, ..self }
    }
}

// MARK: Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_api_video_info_for_test_methods() {
        use chrono::TimeZone;

        let a = ApiVideoInfo::self_a();
        assert_eq!(a.video_id, crate::model::VideoId::test_id_1());
        assert_eq!(a.title, "Test Video A");
        assert_eq!(a.channel_id, crate::model::ChannelId::test_id_1());
        assert_eq!(a.published_at, crate::model::VideoPublishedAt::self_1());
        assert_eq!(a.synced_at, chrono::Utc.with_ymd_and_hms(2025, 1, 1, 1, 1, 1).unwrap());
        assert_eq!(a.duration, crate::model::Duration::self_3());
        assert_eq!(a.privacy_status, crate::model::PrivacyStatus::Public);
        assert!(a.embeddable);

        let b = ApiVideoInfo::self_b();
        assert_eq!(b.video_id, crate::model::VideoId::test_id_2());
        assert_eq!(b.title, "Test Video B");
        assert_eq!(b.channel_id, crate::model::ChannelId::test_id_2());
        assert_eq!(b.published_at, crate::model::VideoPublishedAt::self_2());
        assert_eq!(b.synced_at, chrono::Utc.with_ymd_and_hms(2025, 7, 7, 7, 7, 7).unwrap());
        assert_eq!(b.duration, crate::model::Duration::self_2());
        assert_eq!(b.privacy_status, crate::model::PrivacyStatus::Private);
        assert!(!b.embeddable);
    }

    fn make_detail(id: &crate::model::VideoId) -> ApiVideoInfo {
        use chrono::TimeZone;
        ApiVideoInfoInitializer {
            video_id: id.clone(),
            title: format!("Video Title for {id}"),
            channel_id: crate::model::ChannelId::test_id_1(),
            published_at: crate::model::VideoPublishedAt::self_1(),
            synced_at: chrono::Utc.with_ymd_and_hms(2025, 1, 1, 1, 1, 1).unwrap(),
            duration: crate::model::Duration::self_1(),
            privacy_status: crate::model::PrivacyStatus::Public,
            embeddable: true,
        }
        .init()
    }

    #[test]
    fn test_api_video_info_serde_roundtrip() {
        let detail = make_detail(&crate::model::VideoId::test_id_1());
        let s = serde_json::to_string(&detail).unwrap();
        let d2: ApiVideoInfo = serde_json::from_str(&s).unwrap();
        assert_eq!(detail, d2);
    }
}
