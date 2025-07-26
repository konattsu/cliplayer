/// 検証されたクリップ情報
///
/// 以下を保証
/// - `start_time` < `end_time`
/// - `UUIDv7`のタイムスタンプの時間(h:m:s)と`start_time`の時間が一致
/// - `UUIDv7`の日付(Y:M:D)と動画の公開日が一致
/// - `start_time` or `end_time`の時間が動画の長さを超えない
#[derive(serde::Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VerifiedClip {
    /// 曲名
    song_title: String,
    /// 内部アーティストの一覧
    artists: crate::model::InternalArtists,
    /// 外部アーティストの一覧
    #[serde(skip_serializing_if = "Option::is_none")]
    external_artists: Option<crate::model::ExternalArtists>,
    /// 切り抜いた動画が投稿されているか
    is_clipped: bool,
    /// 曲が始まる時間
    start_time: crate::model::Duration,
    /// 曲が終わる時間
    end_time: crate::model::Duration,
    /// タグ
    #[serde(skip_serializing_if = "Option::is_none")]
    clip_tags: Option<crate::model::ClipTags>,
    /// uuid
    uuid: crate::model::UuidVer7,
    /// 音量の正規化時に設定すべき音量
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_percent: Option<crate::model::VolumePercent>,
}

/// `VerifiedClip`のエラー
#[derive(thiserror::Error, Debug, Clone)]
pub(crate) enum VerifiedClipError {
    /// `start_time` >= `end_time`のとき
    #[error(
        "(@song title: {song_title}), invalid clip time range: \
        start({start_time}) must be less than to end({end_time})"
    )]
    InvalidClipTimeRange {
        song_title: String,
        start_time: crate::model::Duration,
        end_time: crate::model::Duration,
    },
    /// UUIDv7にあるタイムスタンプの時間(h:m:s)と`start_time`の時間が一致しないとき
    #[error(
        "(@song title: {song_title}), uuid time({uuid_time}) \
        does not match start time({start_time})"
    )]
    UuidTimeMismatch {
        song_title: String,
        uuid_time: chrono::NaiveTime,
        start_time: crate::model::Duration,
    },
    /// UUIDv7の日付(Y:M:D)と, 与えられた動画情報にある動画の公開日が一致しないとき
    #[error(
        "(@song title: {song_title}), uuid date({uuid_date}) \
        does not match video date({video_date})"
    )]
    UuidDateMismatch {
        song_title: String,
        uuid_date: chrono::NaiveDate,
        video_date: crate::model::VideoPublishedAt,
    },
    /// `start_time`or `end_time`の時間が, 与えられた動画情報にある動画の長さより長いとき
    #[error(
        "(@song title: {song_title}), time exceeds video duration: \
        start({start_time}), end({end_time}), video duration({video_duration})"
    )]
    TimeExceedsVideoDuration {
        song_title: String,
        start_time: crate::model::Duration,
        end_time: crate::model::Duration,
        video_duration: crate::model::Duration,
    },
}

pub(super) struct VerifiedClipInner {
    /// 曲名
    pub(super) song_title: String,
    /// 内部アーティストの一覧
    pub(super) artists: crate::model::InternalArtists,
    /// 外部アーティストの一覧
    pub(super) external_artists: Option<crate::model::ExternalArtists>,
    /// 切り抜いた動画が投稿されているか
    pub(super) is_clipped: bool,
    /// 曲が始まる時間
    pub(super) start_time: crate::model::Duration,
    /// 曲が終わる時間
    pub(super) end_time: crate::model::Duration,
    /// タグ
    pub(super) clip_tags: Option<crate::model::ClipTags>,
    /// uuid
    pub(super) uuid: crate::model::UuidVer7,
    /// 音量の正規化時に設定すべき音量
    pub(super) volume_percent: Option<crate::model::VolumePercent>,
}

pub(crate) struct VerifiedClipInitializer {
    /// 曲名
    pub(crate) song_title: String,
    /// 内部アーティストの一覧
    pub(crate) artists: crate::model::InternalArtists,
    /// 外部アーティストの一覧
    pub(crate) external_artists: Option<crate::model::ExternalArtists>,
    /// 切り抜いた動画が投稿されているか
    pub(crate) is_clipped: bool,
    /// 曲が始まる時間
    pub(crate) start_time: crate::model::Duration,
    /// 曲が終わる時間
    pub(crate) end_time: crate::model::Duration,
    /// タグ
    pub(crate) clip_tags: Option<crate::model::ClipTags>,
    /// uuid
    pub(crate) uuid: crate::model::UuidVer7,
    /// 音量の正規化時に設定すべき音量
    pub(crate) volume_percent: Option<crate::model::VolumePercent>,
}

impl VerifiedClipInitializer {
    /// `VerifiedClip`を作成
    ///
    /// - Error:
    ///   - `start_time` >= `end_time`のとき
    ///   - `uuid`のタイムスタンプの時間(h:m:s)と`start_time`の時間が一致しないとき
    ///   - `uuid`の日付(Y:M:D)と, 与えられた動画情報にある動画の公開日が一致しないとき
    ///   - `start_time`or `end_time`の時間が, 与えられた動画情報にある動画の長さより長いとき
    pub(crate) fn init(
        self,
        video_published_at: &crate::model::VideoPublishedAt,
        video_duration: &crate::model::Duration,
    ) -> Result<VerifiedClip, VerifiedClipError> {
        self.validate_consistency(video_published_at, video_duration)?;

        Ok(VerifiedClip {
            song_title: self.song_title,
            artists: self.artists,
            external_artists: self.external_artists,
            is_clipped: self.is_clipped,
            start_time: self.start_time,
            end_time: self.end_time,
            clip_tags: self.clip_tags,
            uuid: self.uuid,
            volume_percent: self.volume_percent,
        })
    }

    /// `Self`が存在できるか検証
    fn validate_consistency(
        &self,
        video_published_at: &crate::model::VideoPublishedAt,
        video_duration: &crate::model::Duration,
    ) -> Result<(), VerifiedClipError> {
        super::validate_start_end_times(&self.start_time, &self.end_time).map_err(
            |_| VerifiedClipError::InvalidClipTimeRange {
                song_title: self.song_title.clone(),
                start_time: self.start_time.clone(),
                end_time: self.end_time.clone(),
            },
        )?;
        self.validate_uuid_time()?;
        self.validate_uuid_date(video_published_at)?;
        self.validate_video_duration(video_duration)?;
        Ok(())
    }

    /// uuidのタイムスタンプとクリップの開始時間が一致するか検証
    fn validate_uuid_time(&self) -> Result<(), VerifiedClipError> {
        let uuid_time = self.uuid.get_datetime().time();
        let start_time = self.start_time.as_chrono_time();

        if uuid_time != start_time {
            return Err(VerifiedClipError::UuidTimeMismatch {
                song_title: self.song_title.clone(),
                uuid_time,
                start_time: self.start_time.clone(),
            });
        }
        Ok(())
    }

    /// uuidのタイムスタンプと動画の公開日が一致するか検証
    fn validate_uuid_date(
        &self,
        video_published_at: &crate::model::VideoPublishedAt,
    ) -> Result<(), VerifiedClipError> {
        let uuid_date = self.uuid.get_datetime().date_naive();
        let video_date = video_published_at.as_chrono_datetime().date_naive();

        if uuid_date != video_date {
            return Err(VerifiedClipError::UuidDateMismatch {
                song_title: self.song_title.clone(),
                uuid_date,
                video_date: video_published_at.clone(),
            });
        }
        Ok(())
    }

    /// 動画の長さを超えていないか検証
    fn validate_video_duration(
        &self,
        video_duration: &crate::model::Duration,
    ) -> Result<(), VerifiedClipError> {
        if self.start_time >= *video_duration || self.end_time >= *video_duration {
            return Err(VerifiedClipError::TimeExceedsVideoDuration {
                song_title: self.song_title.clone(),
                start_time: self.start_time.clone(),
                end_time: self.end_time.clone(),
                video_duration: video_duration.clone(),
            });
        }
        Ok(())
    }
}

// deserializerは作成しない
// ∵ 単品で`Verified`かどうかを確認できないため

impl VerifiedClip {
    pub(crate) fn get_uuid(&self) -> &crate::model::UuidVer7 {
        &self.uuid
    }
    pub(crate) fn get_song_title(&self) -> &str {
        &self.song_title
    }
    pub(crate) fn get_start_time(&self) -> &crate::model::Duration {
        &self.start_time
    }
    pub(crate) fn get_end_time(&self) -> &crate::model::Duration {
        &self.end_time
    }

    pub(super) fn into_inner(self) -> VerifiedClipInner {
        VerifiedClipInner {
            song_title: self.song_title,
            artists: self.artists,
            external_artists: self.external_artists,
            is_clipped: self.is_clipped,
            start_time: self.start_time,
            end_time: self.end_time,
            clip_tags: self.clip_tags,
            uuid: self.uuid,
            volume_percent: self.volume_percent,
        }
    }
}

// MARK: For Tests

#[cfg(test)]
impl VerifiedClip {
    // Unverifiedに対応するように作成

    fn self_a_initialize(ini_1: VerifiedClipInitializer) -> Self {
        use chrono::TimeZone;
        ini_1
            .init(
                &crate::model::VideoPublishedAt::new(
                    chrono::Utc.with_ymd_and_hms(2024, 12, 12, 0, 0, 0).unwrap(),
                )
                .unwrap(),
                &crate::model::Duration::from_secs(120),
            )
            .expect("Failed to create VerifiedClip A1")
    }
    pub(crate) fn self_a_1() -> Self {
        let ini_1 = VerifiedClipInitializer {
            song_title: "Test Song A1".to_string(),
            artists: crate::model::InternalArtists::test_name_1(),
            external_artists: Some(crate::model::ExternalArtists::test_name_1()),
            is_clipped: false,
            start_time: crate::model::Duration::from_secs(5),
            end_time: crate::model::Duration::from_secs(10),
            clip_tags: None,
            uuid: crate::model::UuidVer7::self_fix_rnd_1(0, 5),
            volume_percent: None,
        };
        Self::self_a_initialize(ini_1)
    }
    pub(crate) fn self_a_2() -> Self {
        let ini_2 = VerifiedClipInitializer {
            song_title: "Test Song A2".to_string(),
            artists: crate::model::InternalArtists::test_name_2(),
            external_artists: None,
            is_clipped: true,
            start_time: crate::model::Duration::from_secs(15),
            end_time: crate::model::Duration::from_secs(20),
            clip_tags: None,
            uuid: crate::model::UuidVer7::self_fix_rnd_1(0, 15),
            volume_percent: None,
        };
        Self::self_a_initialize(ini_2)
    }
    pub(crate) fn self_a_3() -> Self {
        let ini_3 = VerifiedClipInitializer {
            song_title: "Test Song A3".to_string(),
            artists: crate::model::InternalArtists::test_name_3(),
            external_artists: Some(crate::model::ExternalArtists::test_name_2()),
            is_clipped: false,
            start_time: crate::model::Duration::from_secs(25),
            end_time: crate::model::Duration::from_secs(30),
            clip_tags: Some(crate::model::ClipTags::self_2()),
            uuid: crate::model::UuidVer7::self_fix_rnd_1(0, 25),
            volume_percent: None,
        };
        Self::self_a_initialize(ini_3)
    }

    fn self_b_initialize(ini_2: VerifiedClipInitializer) -> Self {
        use chrono::TimeZone;

        ini_2
            .init(
                &crate::model::VideoPublishedAt::new(
                    chrono::Utc.with_ymd_and_hms(2025, 5, 30, 0, 0, 0).unwrap(),
                )
                .unwrap(),
                &crate::model::Duration::from_secs(120),
            )
            .expect("Failed to create VerifiedClip A2")
    }
    pub(crate) fn self_b_1() -> Self {
        let ini_b1 = VerifiedClipInitializer {
            song_title: "Test Song B1".to_string(),
            artists: crate::model::InternalArtists::test_name_1(),
            external_artists: Some(crate::model::ExternalArtists::test_name_3()),
            is_clipped: true,
            start_time: crate::model::Duration::from_secs(7),
            end_time: crate::model::Duration::from_secs(17),
            clip_tags: Some(crate::model::ClipTags::self_3()),
            uuid: crate::model::UuidVer7::self_fix_rnd_2(0, 7),
            volume_percent: None,
        };
        Self::self_b_initialize(ini_b1)
    }
    pub(crate) fn self_b_2() -> Self {
        let ini_b2 = VerifiedClipInitializer {
            song_title: "Test Song B2".to_string(),
            artists: crate::model::InternalArtists::test_name_2(),
            external_artists: None,
            is_clipped: false,
            start_time: crate::model::Duration::from_secs(27),
            end_time: crate::model::Duration::from_secs(37),
            clip_tags: Some(crate::model::ClipTags::self_1()),
            uuid: crate::model::UuidVer7::self_fix_rnd_2(0, 27),
            volume_percent: None,
        };
        Self::self_b_initialize(ini_b2)
    }
    pub(crate) fn self_b_3() -> Self {
        let ini_b3 = VerifiedClipInitializer {
            song_title: "Test Song B3".to_string(),
            artists: crate::model::InternalArtists::test_name_1(),
            external_artists: None,
            is_clipped: true,
            start_time: crate::model::Duration::from_secs(47),
            end_time: crate::model::Duration::from_secs(57),
            clip_tags: None,
            uuid: crate::model::UuidVer7::self_fix_rnd_2(0, 47),
            volume_percent: None,
        };
        Self::self_b_initialize(ini_b3)
    }

    /// Only for tests
    pub(crate) fn set_end_time(&mut self, end_time: crate::model::Duration) {
        self.end_time = end_time;
    }
}

// MARK: Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_verified_clip_for_test_methods() {
        let clip_a_1 = VerifiedClip::self_a_1();
        assert_eq!(clip_a_1.song_title, "Test Song A1");
        assert_eq!(clip_a_1.artists, crate::model::InternalArtists::test_name_1());
        assert_eq!(clip_a_1.external_artists, Some(crate::model::ExternalArtists::test_name_1()));
        assert!(!clip_a_1.is_clipped);
        assert_eq!(clip_a_1.start_time, crate::model::Duration::from_secs(5));
        assert_eq!(clip_a_1.end_time, crate::model::Duration::from_secs(10));
        assert_eq!(clip_a_1.clip_tags, None);
        assert_eq!(clip_a_1.uuid, crate::model::UuidVer7::self_fix_rnd_1(0, 5));
        assert_eq!(clip_a_1.volume_percent, None);

        let clip_a_2 = VerifiedClip::self_a_2();
        assert_eq!(clip_a_2.song_title, "Test Song A2");
        assert_eq!(clip_a_2.artists, crate::model::InternalArtists::test_name_2());
        assert_eq!(clip_a_2.external_artists, None);
        assert!(clip_a_2.is_clipped);
        assert_eq!(clip_a_2.start_time, crate::model::Duration::from_secs(15));
        assert_eq!(clip_a_2.end_time, crate::model::Duration::from_secs(20));
        assert_eq!(clip_a_2.clip_tags, None);
        assert_eq!(clip_a_2.uuid, crate::model::UuidVer7::self_fix_rnd_1(0, 15));
        assert_eq!(clip_a_2.volume_percent, None);

        let clip_a_3 = VerifiedClip::self_a_3();
        assert_eq!(clip_a_3.song_title, "Test Song A3");
        assert_eq!(clip_a_3.artists, crate::model::InternalArtists::test_name_3());
        assert_eq!(clip_a_3.external_artists, Some(crate::model::ExternalArtists::test_name_2()));
        assert!(!clip_a_3.is_clipped);
        assert_eq!(clip_a_3.start_time, crate::model::Duration::from_secs(25));
        assert_eq!(clip_a_3.end_time, crate::model::Duration::from_secs(30));
        assert_eq!(clip_a_3.clip_tags, Some(crate::model::ClipTags::self_2()));
        assert_eq!(clip_a_3.uuid, crate::model::UuidVer7::self_fix_rnd_1(0, 25));
        assert_eq!(clip_a_3.volume_percent, None);

        let clip_b_1 = VerifiedClip::self_b_1();
        assert_eq!(clip_b_1.song_title, "Test Song B1");
        assert_eq!(clip_b_1.artists, crate::model::InternalArtists::test_name_1());
        assert_eq!(clip_b_1.external_artists, Some(crate::model::ExternalArtists::test_name_3()));
        assert!(clip_b_1.is_clipped);
        assert_eq!(clip_b_1.start_time, crate::model::Duration::from_secs(7));
        assert_eq!(clip_b_1.end_time, crate::model::Duration::from_secs(17));
        assert_eq!(clip_b_1.clip_tags, Some(crate::model::ClipTags::self_3()));
        assert_eq!(clip_b_1.uuid, crate::model::UuidVer7::self_fix_rnd_2(0, 7));
        assert_eq!(clip_b_1.volume_percent, None);

        let clip_b_2 = VerifiedClip::self_b_2();
        assert_eq!(clip_b_2.song_title, "Test Song B2");
        assert_eq!(clip_b_2.artists, crate::model::InternalArtists::test_name_2());
        assert_eq!(clip_b_2.external_artists, None);
        assert!(!clip_b_2.is_clipped);
        assert_eq!(clip_b_2.start_time, crate::model::Duration::from_secs(27));
        assert_eq!(clip_b_2.end_time, crate::model::Duration::from_secs(37));
        assert_eq!(clip_b_2.clip_tags, Some(crate::model::ClipTags::self_1()));
        assert_eq!(clip_b_2.uuid, crate::model::UuidVer7::self_fix_rnd_2(0, 27));
        assert_eq!(clip_b_2.volume_percent, None);

        let clip_b_3 = VerifiedClip::self_b_3();
        assert_eq!(clip_b_3.song_title, "Test Song B3");
        assert_eq!(clip_b_3.artists, crate::model::InternalArtists::test_name_1());
        assert_eq!(clip_b_3.external_artists, None);
        assert!(clip_b_3.is_clipped);
        assert_eq!(clip_b_3.start_time, crate::model::Duration::from_secs(47));
        assert_eq!(clip_b_3.end_time, crate::model::Duration::from_secs(57));
        assert_eq!(clip_b_3.clip_tags, None);
        assert_eq!(clip_b_3.uuid, crate::model::UuidVer7::self_fix_rnd_2(0, 47));
        assert_eq!(clip_b_3.volume_percent, None);
    }

    #[test]
    fn test_verified_clip_validate_uuid_time() {
        // 正常
        let verified_initializer = VerifiedClipInitializer {
            song_title: "Test Song".to_string(),
            artists: crate::model::InternalArtists::test_name_1(),
            external_artists: None,
            is_clipped: false,
            start_time: crate::model::Duration::from_secs(30),
            end_time: crate::model::Duration::from_secs(40),
            clip_tags: None,
            uuid: crate::model::UuidVer7::self_4(),
            volume_percent: None,
        };
        verified_initializer
            .validate_uuid_time()
            .expect("UUID time validation should succeed");

        // 異常, `start_time`と`uuid`の時間が一致しない
        let verified_initializer = VerifiedClipInitializer {
            song_title: "Test Song".to_string(),
            artists: crate::model::InternalArtists::test_name_1(),
            external_artists: None,
            is_clipped: false,
            // この値が`uuid`の時間が一致しない
            start_time: crate::model::Duration::from_secs(40),
            end_time: crate::model::Duration::from_secs(50),
            clip_tags: None,
            uuid: crate::model::UuidVer7::self_4(),
            volume_percent: None,
        };
        let result = verified_initializer.validate_uuid_time();
        assert!(result.is_err());
    }

    #[test]
    fn test_verified_clip_validate_uuid_date() {
        use chrono::TimeZone;

        // 正常
        let verified_initializer = VerifiedClipInitializer {
            song_title: "Test Song".to_string(),
            artists: crate::model::InternalArtists::test_name_1(),
            external_artists: None,
            is_clipped: false,
            start_time: crate::model::Duration::from_secs(30),
            end_time: crate::model::Duration::from_secs(40),
            clip_tags: None,
            uuid: crate::model::UuidVer7::self_4(),
            volume_percent: None,
        };
        verified_initializer
            .validate_uuid_date(
                &crate::model::VideoPublishedAt::new(
                    chrono::Utc
                        .with_ymd_and_hms(2024, 12, 12, 0, 0, 30)
                        .unwrap(),
                )
                .unwrap(),
            )
            .expect("UUID date validation should succeed");

        // 異常, `uuid`の日付と動画の公開日が一致しない
        let verified_initializer = VerifiedClipInitializer {
            song_title: "Test Song".to_string(),
            artists: crate::model::InternalArtists::test_name_1(),
            external_artists: None,
            is_clipped: false,
            start_time: crate::model::Duration::from_secs(30),
            end_time: crate::model::Duration::from_secs(40),
            clip_tags: None,
            uuid: crate::model::UuidVer7::self_4(),
            volume_percent: None,
        };
        let result = verified_initializer.validate_uuid_date(
            &crate::model::VideoPublishedAt::new(
                chrono::Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 30).unwrap(),
            )
            .unwrap(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_verified_clip_validate_video_duration() {
        // 正常
        let verified_initializer = VerifiedClipInitializer {
            song_title: "Test Song".to_string(),
            artists: crate::model::InternalArtists::test_name_1(),
            external_artists: None,
            is_clipped: false,
            start_time: crate::model::Duration::from_secs(30),
            end_time: crate::model::Duration::from_secs(40),
            clip_tags: None,
            uuid: crate::model::UuidVer7::self_4(),
            volume_percent: None,
        };
        verified_initializer
            .validate_video_duration(&crate::model::Duration::from_secs(60))
            .expect("Video duration validation should succeed");

        // 異常, `start_time`か`end_time`動画の長さを超えている
        let verified_initializer = VerifiedClipInitializer {
            song_title: "Test Song".to_string(),
            artists: crate::model::InternalArtists::test_name_1(),
            external_artists: None,
            is_clipped: false,
            start_time: crate::model::Duration::from_secs(30),
            end_time: crate::model::Duration::from_secs(50),
            clip_tags: None,
            uuid: crate::model::UuidVer7::self_4(),
            volume_percent: None,
        };
        let result = verified_initializer
            .validate_video_duration(&crate::model::Duration::from_secs(40));
        assert!(result.is_err());
    }
}
