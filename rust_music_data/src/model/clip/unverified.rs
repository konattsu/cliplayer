/// 検証されていないクリップ情報
///
/// 以下を保証
/// - `start_time` < `end_time`
/// - `UUIDv7`のタイムスタンプの時間(h:m:s)と`start_time`の時間が一致
#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct UnverifiedClip {
    /// 曲名
    song_title: String,
    /// 曲名の平仮名表記
    song_title_jah: String,
    /// 内部アーティストの一覧
    artists: crate::model::InternalArtists,
    /// 外部アーティストの一覧
    external_artists: Option<crate::model::ExternalArtists>,
    /// 切り抜いた動画が投稿されているか
    is_clipped: bool,
    /// 曲が始まる時間
    start_time: crate::model::Duration,
    /// 曲が終わる時間
    end_time: crate::model::Duration,
    /// タグ
    tags: Option<crate::model::TagList>,
    /// uuid
    uuid: crate::model::UuidVer7,
    /// 音量の正規化時に設定すべき音量
    volume_percent: Option<crate::model::VolumePercent>,
}

/// `UnverifiedClip`のエラー
#[derive(thiserror::Error, Debug, Clone)]
pub enum UnverifiedClipError {
    /// `start_time` >= `end_time`のとき
    #[error(
        "invalid clip time range: start({start_time}) must be less than to end({end_time})"
    )]
    InvalidClipTimeRange {
        start_time: crate::model::Duration,
        end_time: crate::model::Duration,
    },
    /// UUIDv7にあるタイムスタンプの時間(h:m:s)と`start_time`の時間が一致しないとき
    #[error("uuid time({uuid_time}) does not match start time({start_time})")]
    UuidTimeMismatch {
        uuid_time: chrono::NaiveTime,
        start_time: crate::model::Duration,
    },
}

pub struct UnverifiedClipInitializer {
    /// 曲名
    pub song_title: String,
    /// 曲名の平仮名表記
    pub song_title_jah: String,
    /// 内部アーティストの一覧
    pub artists: crate::model::InternalArtists,
    /// 外部アーティストの一覧
    pub external_artists: Option<crate::model::ExternalArtists>,
    /// 切り抜いた動画が投稿されているか
    pub is_clipped: bool,
    /// 曲が始まる時間
    pub start_time: crate::model::Duration,
    /// 曲が終わる時間
    pub end_time: crate::model::Duration,
    /// タグ
    pub tags: Option<crate::model::TagList>,
    /// uuid
    pub uuid: crate::model::UuidVer7,
    /// 音量の正規化時に設定すべき音量
    pub volume_percent: Option<crate::model::VolumePercent>,
}

impl UnverifiedClipInitializer {
    /// `UnverifiedClip`を作成
    ///
    /// - Error: `start_time` >= `end_time`のとき
    ///   - e.g. `start_time`: 5秒, `end_time`: 3秒
    pub fn init(self) -> Result<UnverifiedClip, UnverifiedClipError> {
        UnverifiedClip::validate_consistency(
            &self.uuid,
            &self.start_time,
            &self.end_time,
        )?;

        Ok(UnverifiedClip {
            song_title: self.song_title,
            song_title_jah: self.song_title_jah,
            artists: self.artists,
            external_artists: self.external_artists,
            is_clipped: self.is_clipped,
            start_time: self.start_time,
            end_time: self.end_time,
            tags: self.tags,
            uuid: self.uuid,
            volume_percent: self.volume_percent,
        })
    }
}

// デシリアライズ時に`UnverifiedClip`の保証内容を満たしているか確認するためのカスタムデシリアライザ
impl<'de> serde::Deserialize<'de> for UnverifiedClip {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct RawUnverifiedClip {
            song_title: String,
            song_title_jah: String,
            artists: crate::model::InternalArtists,
            external_artists: Option<crate::model::ExternalArtists>,
            is_clipped: bool,
            start_time: crate::model::Duration,
            end_time: crate::model::Duration,
            tags: Option<crate::model::TagList>,
            uuid: crate::model::UuidVer7,
            volume_percent: Option<crate::model::VolumePercent>,
        }

        let raw: RawUnverifiedClip = serde::Deserialize::deserialize(deserializer)
            .map_err(serde::de::Error::custom)?;

        // `Self`の存在条件の検証
        UnverifiedClip::validate_consistency(&raw.uuid, &raw.start_time, &raw.end_time)
            .map_err(serde::de::Error::custom)?;

        Ok(UnverifiedClip {
            song_title: raw.song_title,
            song_title_jah: raw.song_title_jah,
            artists: raw.artists,
            external_artists: raw.external_artists,
            is_clipped: raw.is_clipped,
            start_time: raw.start_time,
            end_time: raw.end_time,
            tags: raw.tags,
            uuid: raw.uuid,
            volume_percent: raw.volume_percent,
        })
    }
}

impl UnverifiedClip {
    /// 与えられた`datetime`と`start_time`を基に`VerifiedClip`を生成する
    ///
    /// - Error:
    ///   - `start_time` >= `end_time`のとき
    ///   - `uuid`のタイムスタンプの時間(h:m:s)と`start_time`の時間が一致しないとき
    ///   - `uuid`の日付(Y:M:D)と, 与えられた動画情報にある動画の公開日が一致しないとき
    ///   - `start_time`or `end_time`の時間が, 与えられた動画情報にある動画の長さより長いとき
    pub fn try_into_verified_clip(
        self,
        video_published_at: &crate::model::VideoPublishedAt,
        video_duration: &crate::model::Duration,
    ) -> Result<super::VerifiedClip, super::VerifiedClipError> {
        super::VerifiedClipInitializer {
            song_title: self.song_title,
            song_title_jah: self.song_title_jah,
            artists: self.artists,
            external_artists: self.external_artists,
            is_clipped: self.is_clipped,
            start_time: self.start_time,
            end_time: self.end_time,
            tags: self.tags,
            uuid: self.uuid,
            volume_percent: self.volume_percent,
        }
        .init(video_published_at, video_duration)
    }

    /// `Self`が存在できるか検証
    ///
    /// Error:
    /// - `start_time` >= `end_time`のとき
    /// - `uuid`のタイムスタンプの時間(h:m:s)と`start_time`の時間が一致しないとき
    fn validate_consistency(
        uuid: &crate::model::UuidVer7,
        start_time: &crate::model::Duration,
        end_time: &crate::model::Duration,
    ) -> Result<(), UnverifiedClipError> {
        // `start_time` >= `end_time`の検証
        super::validate_start_end_times(start_time, end_time).map_err(|_| {
            UnverifiedClipError::InvalidClipTimeRange {
                start_time: start_time.clone(),
                end_time: end_time.clone(),
            }
        })?;

        // `uuid`のタイムスタンプの時間(h:m:s)と`start_time`の時間が一致するか検証
        let uuid_time = uuid.get_datetime().time();
        let start_time_chrono = start_time.as_chrono_time();
        if uuid_time != start_time_chrono {
            return Err(UnverifiedClipError::UuidTimeMismatch {
                uuid_time,
                start_time: start_time.clone(),
            });
        }

        Ok(())
    }
}

// MARK: Tests

#[cfg(test)]
mod tests {
    use super::*;

    // 2024-12-12T12:12:12Z
    const UNVERIFIED_CLIP_JSON_VALID: &str = r#"
    {
        "songTitle": "Test Song 1",
        "songTitleJah": "てすとそんぐいち",
        "artists": ["Aimer Test"],
        "externalArtists": ["Apple Mike"],
        "isClipped": false,
        "startTime": "PT12H12M12S",
        "endTime": "PT12H12M20S",
        "tags": ["tag1", "tag2"],
        "uuid": "0193bac8-a560-7000-8000-000000000000"
    }"#;

    // `startTime` >= `endTime`
    const UNVERIFIED_CLIP_JSON_INVALID: &str = r#"
    {
        "songTitle": "Test Song 2",
        "songTitleJah": "てすとそんぐに",
        "artists": ["Aimer Test"],
        "externalArtists": ["Apple Mike"],
        "isClipped": false,
        "startTime": "PT12H12M12S",
        "endTime": "PT12H12M5S",
        "tags": ["tag1", "tag2"],
        "uuid": "0193bac8-a560-7000-8000-000000000000"
    }"#;

    const SEC_12H_12M_12S: u16 = 12 * 3600 + 12 * 60 + 12;

    fn dur_12h_12m_12s() -> crate::model::Duration {
        crate::model::Duration::from_secs(SEC_12H_12M_12S)
    }

    /// 引数の値が大きすぎるとpanic
    fn dur_12h_12m_12s_plus(secs: i8) -> crate::model::Duration {
        let total_secs = SEC_12H_12M_12S as i32 + secs as i32;
        crate::model::Duration::from_secs(total_secs.try_into().unwrap())
    }

    // deserializeでも存在条件が確認されているか確認
    #[test]
    fn test_unverified_clip_deserialize() {
        let clip: UnverifiedClip = serde_json::from_str(UNVERIFIED_CLIP_JSON_VALID)
            .expect("Failed to deserialize valid UnverifiedClip JSON");
        assert_eq!(clip.song_title, "Test Song 1");
        assert_eq!(clip.song_title_jah, "てすとそんぐいち");
        assert_eq!(clip.artists, crate::model::InternalArtists::test_name_1());
        assert_eq!(
            clip.external_artists,
            Some(crate::model::ExternalArtists::test_name_1())
        );
        assert!(!clip.is_clipped);
        assert_eq!(clip.start_time, dur_12h_12m_12s());
        assert_eq!(clip.end_time, dur_12h_12m_12s_plus(8));
        assert_eq!(clip.tags, Some(crate::model::TagList::test_tag_list_1()));
        assert_eq!(
            clip.uuid.to_string(),
            "0193bac8-a560-7000-8000-000000000000"
        );
        assert_eq!(clip.volume_percent, None);

        let result =
            serde_json::from_str::<UnverifiedClip>(UNVERIFIED_CLIP_JSON_INVALID);
        assert!(result.is_err());
    }

    #[test]
    fn test_unverified_clip_validate_consistency() {
        let uuid = crate::model::UuidVer7::self_1();
        // 正常
        {
            let start_time = dur_12h_12m_12s();
            let end_time = dur_12h_12m_12s_plus(8);
            UnverifiedClip::validate_consistency(&uuid, &start_time, &end_time)
                .expect("Failed to validate consistency");
        }
        // 異常, start_time >= end_time
        {
            let start_time = dur_12h_12m_12s();
            let end_time_invalid = dur_12h_12m_12s_plus(-5);
            let result = UnverifiedClip::validate_consistency(
                &uuid,
                &start_time,
                &end_time_invalid,
            );
            assert!(matches!(
                result,
                Err(UnverifiedClipError::InvalidClipTimeRange { .. })
            ));
        }
        // 異常, uuidのタイムスタンプの時間(h:m:s)とstart_timeの時間が一致しない
        {
            let start_time_invalid = dur_12h_12m_12s_plus(10);
            let end_time = dur_12h_12m_12s_plus(20);
            let result = UnverifiedClip::validate_consistency(
                &uuid,
                &start_time_invalid,
                &end_time,
            );
            assert!(matches!(
                result,
                Err(UnverifiedClipError::UuidTimeMismatch { .. })
            ));
        }
    }

    #[test]
    fn test_unverified_clip_initializer_init() {
        // 正常
        let initializer = UnverifiedClipInitializer {
            song_title: "Test Song 1".to_string(),
            song_title_jah: "てすとそんぐいち".to_string(),
            artists: crate::model::InternalArtists::test_name_1(),
            external_artists: Some(crate::model::ExternalArtists::test_name_1()),
            is_clipped: false,
            start_time: dur_12h_12m_12s(),
            end_time: dur_12h_12m_12s_plus(8),
            tags: Some(crate::model::TagList::test_tag_list_1()),
            uuid: crate::model::UuidVer7::self_1(),
            volume_percent: None,
        };
        let _clip = initializer.init().expect("Failed to create UnverifiedClip");
        // 異常, start_time < end_time でない
        let initializer = UnverifiedClipInitializer {
            song_title: "Test Song 2".to_string(),
            song_title_jah: "てすとそんぐに".to_string(),
            artists: crate::model::InternalArtists::test_name_1(),
            external_artists: Some(crate::model::ExternalArtists::test_name_1()),
            is_clipped: false,
            start_time: dur_12h_12m_12s(),
            end_time: dur_12h_12m_12s_plus(-5),
            tags: Some(crate::model::TagList::test_tag_list_1()),
            uuid: crate::model::UuidVer7::self_1(),
            volume_percent: None,
        };
        let result = initializer.init();
        assert!(matches!(
            result,
            Err(UnverifiedClipError::InvalidClipTimeRange { .. })
        ));
        // 異常, uuidのタイムスタンプの時間(h:m:s)とstart_timeの時間が一致しない
        let initializer = UnverifiedClipInitializer {
            song_title: "Test Song 3".to_string(),
            song_title_jah: "てすとそんぐさん".to_string(),
            artists: crate::model::InternalArtists::test_name_1(),
            external_artists: Some(crate::model::ExternalArtists::test_name_1()),
            is_clipped: false,
            start_time: dur_12h_12m_12s_plus(10),
            end_time: dur_12h_12m_12s_plus(20),
            tags: Some(crate::model::TagList::test_tag_list_1()),
            uuid: crate::model::UuidVer7::self_1(),
            volume_percent: None,
        };
        let result = initializer.init();
        assert!(matches!(
            result,
            Err(UnverifiedClipError::UuidTimeMismatch { .. })
        ));
    }
}
