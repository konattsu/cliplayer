/// 動画の詳細情報
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct VideoDetail {
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
    #[serde(default)]
    video_tags: crate::model::VideoTags,
}

/// 動画の詳細情報のリスト
///
/// 各`VideoDetail`に含まれる動画idが一意であることを保証
#[derive(Debug, Clone)]
pub(crate) struct VideoDetails {
    pub(crate) inner: std::collections::HashMap<crate::model::VideoId, VideoDetail>,
}

pub(crate) struct VideoDetailInitializer {
    /// 動画ID
    pub(crate) video_id: crate::model::VideoId,
    /// 動画のタイトル
    pub(crate) title: String,
    /// チャンネルID
    pub(crate) channel_id: crate::model::ChannelId,
    /// チャンネル名, 箱外で行われた配信/動画の時に付与
    pub(crate) uploader_name: Option<crate::model::UploaderName>,
    /// 動画の公開日時
    pub(crate) published_at: crate::model::VideoPublishedAt,
    /// この情報を取得した日時
    pub(crate) modified_at: chrono::DateTime<chrono::Utc>,
    /// 動画の長さ
    pub(crate) duration: crate::model::Duration,
    /// 動画のプライバシー設定
    pub(crate) privacy_status: crate::model::PrivacyStatus,
    /// 動画が埋め込み可能かどうか
    pub(crate) embeddable: bool,
    /// 動画のタグ
    pub(crate) video_tags: crate::model::VideoTags,
}

impl VideoDetailInitializer {
    pub(crate) fn init(self) -> VideoDetail {
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
    pub(crate) fn get_published_at(&self) -> &crate::model::VideoPublishedAt {
        &self.published_at
    }
    pub(crate) fn get_duration(&self) -> &crate::model::Duration {
        &self.duration
    }
    pub(crate) fn get_video_id(&self) -> &crate::model::VideoId {
        &self.video_id
    }

    pub(crate) fn is_same_except_modified_at(&self, other: &Self) -> bool {
        self.video_id == other.video_id
            && self.title == other.title
            && self.channel_id == other.channel_id
            && self.uploader_name == other.uploader_name
            && self.published_at == other.published_at
            && self.duration == other.duration
            && self.privacy_status == other.privacy_status
            && self.embeddable == other.embeddable
            && self.video_tags == other.video_tags
    }

    pub(crate) fn into_briefs(self) -> crate::model::VideoBrief {
        crate::model::VideoBrief::new(
            self.video_id,
            self.uploader_name,
            self.video_tags,
        )
    }
}

impl VideoDetails {
    /// `VideoDetail`のリストを`VideoDetails`に変換
    ///
    /// Err: 動画idが重複しているとき
    pub(crate) fn try_from_vec(
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

    pub(crate) fn into_briefs(self) -> crate::model::VideoBriefs {
        let inner = self
            .inner
            .into_iter()
            .map(|(id, detail)| (id, detail.into_briefs()))
            .collect();
        crate::model::VideoBriefs { inner }
    }
}

// MARK: For Tests

#[cfg(test)]
impl VideoDetail {
    // clip/verified に対応するように作成

    pub(crate) fn self_a() -> Self {
        use chrono::TimeZone;
        VideoDetailInitializer {
            video_id: crate::model::VideoId::test_id_1(),
            title: "Test Video A".to_string(),
            channel_id: crate::model::ChannelId::test_id_1(),
            uploader_name: Some(crate::model::UploaderName::test_uploader_name_1()),
            published_at: crate::model::VideoPublishedAt::self_1(),
            modified_at: chrono::Utc.with_ymd_and_hms(2025, 1, 1, 1, 1, 1).unwrap(),
            duration: crate::model::Duration::self_3(),
            privacy_status: crate::model::PrivacyStatus::Public,
            embeddable: true,
            video_tags: crate::model::VideoTags::self_1(),
        }
        .init()
    }

    pub(crate) fn self_b() -> Self {
        use chrono::TimeZone;
        VideoDetailInitializer {
            video_id: crate::model::VideoId::test_id_2(),
            title: "Test Video B".to_string(),
            channel_id: crate::model::ChannelId::test_id_2(),
            uploader_name: Some(crate::model::UploaderName::test_uploader_name_2()),
            published_at: crate::model::VideoPublishedAt::self_2(),
            modified_at: chrono::Utc.with_ymd_and_hms(2025, 7, 7, 7, 7, 7).unwrap(),
            duration: crate::model::Duration::self_2(),
            privacy_status: crate::model::PrivacyStatus::Private,
            embeddable: false,
            video_tags: crate::model::VideoTags::self_2(),
        }
        .init()
    }

    pub(crate) fn update_modified_at(self, new: chrono::DateTime<chrono::Utc>) -> Self {
        Self {
            modified_at: new,
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
    fn test_video_detail_for_test_methods() {
        use chrono::TimeZone;

        let a = VideoDetail::self_a();
        assert_eq!(a.video_id, crate::model::VideoId::test_id_1());
        assert_eq!(a.title, "Test Video A");
        assert_eq!(a.channel_id, crate::model::ChannelId::test_id_1());
        assert_eq!(a.uploader_name.as_ref().unwrap(), &crate::model::UploaderName::test_uploader_name_1());
        assert_eq!(a.published_at, crate::model::VideoPublishedAt::self_1());
        assert_eq!(a.modified_at, chrono::Utc.with_ymd_and_hms(2025, 1, 1, 1, 1, 1).unwrap());
        assert_eq!(a.duration, crate::model::Duration::self_3());
        assert_eq!(a.privacy_status, crate::model::PrivacyStatus::Public);
        assert!(a.embeddable);
        assert_eq!(a.video_tags, crate::model::VideoTags::self_1());

        let b = VideoDetail::self_b();
        assert_eq!(b.video_id, crate::model::VideoId::test_id_2());
        assert_eq!(b.title, "Test Video B");
        assert_eq!(b.channel_id, crate::model::ChannelId::test_id_2());
        assert_eq!(b.uploader_name.as_ref().unwrap(), &crate::model::UploaderName::test_uploader_name_2());
        assert_eq!(b.published_at, crate::model::VideoPublishedAt::self_2());
        assert_eq!(b.modified_at, chrono::Utc.with_ymd_and_hms(2025, 7, 7, 7, 7, 7).unwrap());
        assert_eq!(b.duration, crate::model::Duration::self_2());
        assert_eq!(b.privacy_status, crate::model::PrivacyStatus::Private);
        assert!(!b.embeddable);
        assert_eq!(b.video_tags, crate::model::VideoTags::self_2());
    }

    fn base_json(id: &crate::model::VideoId) -> serde_json::Value {
        serde_json::json!({
            "videoId": id.to_string(),
            "title": format!("Video Title for {id}"),
            "channelId": crate::model::ChannelId::test_id_1().to_string(),
            "publishedAt": crate::model::VideoPublishedAt::self_1().to_string(),
            "modifiedAt": crate::model::VideoPublishedAt::self_1().to_string(),
            "duration": crate::model::Duration::self_1().to_string(),
            "privacyStatus": "public",
            "embeddable": true,
            "videoTags": ["Test Video Tag1"]
        })
    }

    fn make_detail(id: &crate::model::VideoId) -> VideoDetail {
        use chrono::TimeZone;
        VideoDetailInitializer {
            video_id: id.clone(),
            title: format!("Video Title for {id}"),
            channel_id: crate::model::ChannelId::test_id_1(),
            uploader_name: None,
            published_at: crate::model::VideoPublishedAt::self_1(),
            modified_at: chrono::Utc.with_ymd_and_hms(2025, 1, 1, 1, 1, 1).unwrap(),
            duration: crate::model::Duration::self_1(),
            privacy_status: crate::model::PrivacyStatus::Public,
            embeddable: true,
            video_tags: crate::model::VideoTags::self_1(),
        }
        .init()
    }

    // serde

    #[test]
    fn test_video_detail_deserialize_with_uploader_name() {
        let id = crate::model::VideoId::test_id_1();
        let mut v = base_json(&id);
        v["uploaderName"] = serde_json::json!("Uploader");
        let detail: VideoDetail = serde_json::from_value(v).unwrap();
        assert_eq!(detail.video_id, id);
        assert_eq!(detail.uploader_name.as_ref().unwrap().as_str(), "Uploader");
    }

    #[test]
    fn test_video_detail_deserialize_without_uploader_name() {
        let id = crate::model::VideoId::test_id_1();
        let v = base_json(&id);
        let detail: VideoDetail = serde_json::from_value(v).unwrap();
        assert_eq!(detail.video_id, id);
        assert!(detail.uploader_name.is_none());
    }

    #[test]
    fn test_video_detail_deserialize_with_uploader_name_null() {
        let id = crate::model::VideoId::test_id_1();
        let mut v = base_json(&id);
        v["uploaderName"] = serde_json::Value::Null;
        let detail: VideoDetail = serde_json::from_value(v).unwrap();
        assert!(detail.uploader_name.is_none());
    }

    #[test]
    fn test_video_detail_deserialize_with_empty_tags() {
        let id = crate::model::VideoId::test_id_1();
        let mut v = base_json(&id);
        v["videoTags"] = serde_json::json!([]);
        let detail: VideoDetail = serde_json::from_value(v).unwrap();
        assert!(detail.video_tags.is_empty() || detail.video_tags.is_empty());
    }

    #[test]
    fn test_video_detail_with_uploader_name_none() {
        let id = crate::model::VideoId::test_id_1();
        let mut detail = make_detail(&id);
        detail.uploader_name = None;
        let s = serde_json::to_string(&detail).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert!(!v.as_object().unwrap().contains_key("uploaderName"));
    }

    #[test]
    fn test_video_detail_serialize_with_uploader_name_some() {
        let id = crate::model::VideoId::test_id_1();
        let mut detail = make_detail(&id);
        detail.uploader_name = Some(crate::model::UploaderName::test_uploader_name_1());
        let s = serde_json::to_string(&detail).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["uploaderName"], serde_json::json!("Test Channel 1"));
    }

    #[test]
    fn test_video_detail_serialize_with_empty_tags() {
        let id = crate::model::VideoId::test_id_1();
        let mut detail = make_detail(&id);
        // 空タグ
        detail.video_tags = crate::model::VideoTags::new(vec![]).unwrap();
        let s = serde_json::to_string(&detail).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert!(v["videoTags"].as_array().unwrap().is_empty());
    }

    #[test]
    fn test_video_detail_serialize_with_multiple_tags() {
        let id = crate::model::VideoId::test_id_1();
        let mut detail = make_detail(&id);
        detail.video_tags =
            crate::model::VideoTags::new(vec!["foo".to_string(), "bar".to_string()])
                .unwrap();
        let s = serde_json::to_string(&detail).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        let tags = v["videoTags"].as_array().unwrap();
        assert!(
            tags.contains(&serde_json::json!("foo"))
                && tags.contains(&serde_json::json!("bar"))
        );
    }

    // other

    #[test]
    fn test_video_details_try_from_vec_success() {
        let id1 = crate::model::VideoId::test_id_1();
        let id2 = crate::model::VideoId::test_id_2();

        let d1 = make_detail(&id1);
        let d2 = make_detail(&id2);
        let details = vec![d1.clone(), d2.clone()];
        let result = VideoDetails::try_from_vec(details);
        assert!(result.is_ok());
        let vd = result.unwrap();
        assert_eq!(vd.inner.len(), 2);
        assert!(vd.inner.contains_key(&id1));
        assert!(vd.inner.contains_key(&id2));
    }

    #[test]
    fn test_video_details_try_from_vec_duplicate_error() {
        let id1 = crate::model::VideoId::test_id_1();

        let d1 = make_detail(&id1);
        let d2 = make_detail(&id1);
        let details = vec![d1, d2];
        let result = VideoDetails::try_from_vec(details);
        assert!(result.is_err());
        let ids = result.err().unwrap();
        assert_eq!(ids.len(), 1);
        assert_eq!(ids[0], id1);
    }

    #[test]
    fn test_video_details_into_briefs() {
        let d1 = make_detail(&crate::model::VideoId::test_id_1());
        let d2 = make_detail(&crate::model::VideoId::test_id_2());
        let details = vec![d1, d2];
        let vd = VideoDetails::try_from_vec(details).unwrap();
        let briefs = vd.into_briefs();
        assert_eq!(briefs.inner.len(), 2);
    }

    #[test]
    fn test_video_detail_serde_roundtrip() {
        let detail = make_detail(&crate::model::VideoId::test_id_1());
        let s = serde_json::to_string(&detail).unwrap();
        let d2: VideoDetail = serde_json::from_str(&s).unwrap();
        assert_eq!(detail, d2);
    }
}
