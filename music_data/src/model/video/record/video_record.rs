#[derive(Debug, Clone, PartialEq)]
pub(crate) struct VideoRecord {
    video_id: crate::model::VideoId,
    local: crate::model::video::record::local::LocalVideoInfo,
    api: crate::model::video::record::api::ApiVideoInfo,
}

#[derive(Debug, PartialEq)]
pub(crate) struct VideoRecordError {
    pub(crate) local: crate::model::VideoId,
    pub(crate) api: crate::model::VideoId,
}

/// この子にserde担当してもらう
#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct RawVideoRecord {
    /// 動画id
    video_id: crate::model::VideoId,

    // api
    /// 動画のタイトル
    title: String,
    /// チャンネルID
    channel_id: crate::model::ChannelId,
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

    // local
    /// チャンネル名, 箱外で行われた配信/動画の時に付与
    uploader_name: Option<crate::model::UploaderName>,
    /// 動画のタグ
    #[serde(default)]
    video_tags: crate::model::VideoTags,
}

impl From<VideoRecord> for RawVideoRecord {
    fn from(value: VideoRecord) -> Self {
        RawVideoRecord {
            video_id: value.video_id,
            title: value.api.title,
            channel_id: value.api.channel_id,
            published_at: value.api.published_at,
            modified_at: value.api.modified_at,
            duration: value.api.duration,
            privacy_status: value.api.privacy_status,
            embeddable: value.api.embeddable,
            uploader_name: value.local.uploader_name,
            video_tags: value.local.video_tags,
        }
    }
}

impl<'de> serde::Deserialize<'de> for VideoRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = RawVideoRecord::deserialize(deserializer)?;

        let local = crate::model::LocalVideoInfo::new(
            raw.video_id.clone(),
            raw.uploader_name,
            raw.video_tags,
        );
        let api = crate::model::ApiVideoInfoInitializer {
            video_id: raw.video_id,
            title: raw.title,
            channel_id: raw.channel_id,
            published_at: raw.published_at,
            modified_at: raw.modified_at,
            duration: raw.duration,
            privacy_status: raw.privacy_status,
            embeddable: raw.embeddable,
        }
        .init();

        VideoRecord::new(local, api)
            .map_err(|e| serde::de::Error::custom(e.to_pretty_string()))
    }
}

impl serde::Serialize for VideoRecord {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let raw: RawVideoRecord = self.clone().into();
        raw.serialize(serializer)
    }
}

impl VideoRecord {
    pub(crate) fn get_video_id(&self) -> &crate::model::VideoId {
        &self.video_id
    }
    pub(crate) fn get_api(&self) -> &super::ApiVideoInfo {
        &self.api
    }

    /// 新しい`VideoRecord`を作成
    ///
    /// Err: `local`と`api`の動画idが一致しない場合
    pub(crate) fn new(
        local: crate::model::video::record::local::LocalVideoInfo,
        api: crate::model::video::record::api::ApiVideoInfo,
    ) -> Result<Self, VideoRecordError> {
        Self::ensure_same_video_id(&local, &api)?;

        Ok(Self {
            video_id: local.video_id.clone(),
            local,
            api,
        })
    }

    /// 新しいapiから取得した動画の詳細情報を適用
    ///
    /// # Returns:
    /// - Ok(true): 動画idが一致し, modified_at以外に変更点が無かったとき
    /// - Ok(false): 動画idが一致し, modified_at以外にも変更があったとき
    /// - Err(VideoRecordError): 動画idが一致しなかったとき
    pub(crate) fn with_new_api_info(
        &mut self,
        api: crate::model::video::record::api::ApiVideoInfo,
    ) -> Result<bool, VideoRecordError> {
        if self.api.is_same_except_modified_at(&api) {
            // modified_at以外に変更点が無かったとき(動画idも一致していることを確認できる)
            // modified_atを更新
            self.api = api;
            Ok(true)
        } else {
            // modified_at以外にも変更があったとき
            // 動画idが一致していないとErr
            Self::ensure_same_video_id(&self.local, &api)?;
            // 動画idが一致しているが, modified_at以外にも変更があったとき
            self.api = api;
            Ok(false)
        }
    }

    fn ensure_same_video_id(
        local: &crate::model::video::record::local::LocalVideoInfo,
        api: &crate::model::video::record::api::ApiVideoInfo,
    ) -> Result<(), VideoRecordError> {
        if local.video_id != api.video_id {
            Err(VideoRecordError {
                local: local.video_id.clone(),
                api: api.video_id.clone(),
            })
        } else {
            Ok(())
        }
    }
}

impl VideoRecordError {
    pub(crate) fn to_pretty_string(&self) -> String {
        format!(
            "Failed to create Video Record: no match video_id: local:{}, api:{}",
            self.local, self.api
        )
    }
}

// MARK: For Tests

#[cfg(test)]
impl VideoRecord {
    pub(crate) fn self_a() -> Self {
        let local = super::LocalVideoInfo::self_a();
        let api = super::ApiVideoInfo::self_a();
        Self::new(local, api).unwrap()
    }
    pub(crate) fn self_b() -> Self {
        let local = super::LocalVideoInfo::self_b();
        let api = super::ApiVideoInfo::self_b();
        Self::new(local, api).unwrap()
    }

    pub(crate) fn update_modified_at(self, new: chrono::DateTime<chrono::Utc>) -> Self {
        let api = self.api.clone().update_modified_at(new);
        Self { api, ..self }
    }

    pub(crate) fn set_duration(self, duration: crate::model::Duration) -> Self {
        let api = self.api.clone().set_duration(duration);
        Self { api, ..self }
    }
}

// MARK: Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_record_new_success() {
        let local = crate::model::LocalVideoInfo::self_a();
        let api = crate::model::ApiVideoInfo::self_a();
        let record = VideoRecord::new(local.clone(), api.clone());
        assert!(record.is_ok());
        let record = record.unwrap();
        assert_eq!(record.get_video_id(), &local.video_id);
        assert_eq!(record.get_api().video_id, api.video_id);
    }

    #[test]
    fn test_video_record_new_error_video_id_mismatch() {
        let local = crate::model::LocalVideoInfo::self_a();
        // video_idが違う
        let api = crate::model::ApiVideoInfo::self_b();

        let result = VideoRecord::new(local, api);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert_eq!(
            err,
            super::VideoRecordError {
                local: crate::model::VideoId::test_id_1(),
                api: crate::model::VideoId::test_id_2(),
            }
        );
    }

    #[test]
    fn test_video_record_with_new_api_info_true() {
        let mut record = VideoRecord::self_a();
        let api = record.api.clone();
        let result = record.with_new_api_info(api).unwrap();
        // 変更がないのでtrue
        assert!(result);
    }

    #[test]
    fn test_video_record_with_new_api_info_false() {
        let mut record = VideoRecord::self_a();
        let mut api = record.api.clone();
        // durationを変更
        api = api.set_duration(crate::model::Duration::from_secs_u16(65000));
        let result = record.with_new_api_info(api).unwrap();
        // 変更があったのでfalse
        assert!(!result);
    }

    #[test]
    fn test_video_record_serde_roundtrip() {
        let record = VideoRecord::self_a();
        let json = serde_json::to_string(&record).unwrap();
        let de_ser: VideoRecord = serde_json::from_str(&json).unwrap();
        assert_eq!(record, de_ser);
    }

    #[test]
    fn test_video_record_update_modified_at() {
        let record = VideoRecord::self_a();
        let new_time = chrono::Utc::now();
        let updated = record.update_modified_at(new_time);
        assert_eq!(updated.api.modified_at, new_time);
    }

    #[test]
    fn test_video_record_set_duration() {
        let record = VideoRecord::self_a();
        let new_duration = crate::model::Duration::from_secs_u16(1234);
        let updated = record.set_duration(new_duration.clone());
        assert_eq!(updated.api.duration, new_duration);
    }
}
