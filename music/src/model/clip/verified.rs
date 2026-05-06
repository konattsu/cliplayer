/// 検証されたクリップ情報
///
/// 以下を保証
/// - `start_time` < `end_time`
/// - `UUIDv7`のタイムスタンプの時間(h:m:s)と`start_time`の時間が一致
/// - `UUIDv7`の日付(Y:M:D)と動画の公開日が一致
/// - `start_time` or `end_time`の時間が動画の長さを超えない
#[derive(serde::Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VerifiedClip {
    /// 曲名
    song_title: String,
    /// 内部アーティストの一覧
    liver_ids: artistctl::model::LiverIds,
    /// 外部アーティストの一覧
    #[serde(skip_serializing_if = "Option::is_none")]
    external_artists_name: Option<artistctl::model::ExternalArtistsName>,
    /// 切り抜いた動画が存在した場合の動画id
    #[serde(skip_serializing_if = "Option::is_none")]
    clipped_video_id: Option<crate::model::VideoId>,
    /// 曲が始まる時間
    start_time: crate::model::Duration,
    /// 曲が終わる時間
    end_time: crate::model::Duration,
    /// uuid
    uuid: crate::model::UuidVer4,
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
    pub(super) liver_ids: artistctl::model::LiverIds,
    /// 外部アーティストの一覧
    pub(super) external_artists_name: Option<artistctl::model::ExternalArtistsName>,
    /// 切り抜いた動画が存在した場合の動画id
    pub(super) clipped_video_id: Option<crate::model::VideoId>,
    /// 曲が始まる時間
    pub(super) start_time: crate::model::Duration,
    /// 曲が終わる時間
    pub(super) end_time: crate::model::Duration,
    /// uuid
    pub(super) uuid: crate::model::UuidVer4,
}

pub(super) struct VerifiedClipInitializer {
    /// 曲名
    pub(super) song_title: String,
    /// 内部アーティストの一覧
    pub(super) liver_ids: artistctl::model::LiverIds,
    /// 外部アーティストの一覧
    pub(super) external_artists_name: Option<artistctl::model::ExternalArtistsName>,
    /// 切り抜いた動画が存在した場合の動画id
    pub(super) clipped_video_id: Option<crate::model::VideoId>,
    /// 曲が始まる時間
    pub(super) start_time: crate::model::Duration,
    /// 曲が終わる時間
    pub(super) end_time: crate::model::Duration,
    /// uuid
    pub(super) uuid: crate::model::UuidVer4,
}

impl VerifiedClipInitializer {
    /// `VerifiedClip`を作成
    ///
    /// - Error:
    ///   - `start_time` >= `end_time`のとき
    ///   - `start_time`or `end_time`の時間が, 与えられた動画情報にある動画の長さより長いとき
    pub(super) fn init(
        self,
        video_duration: &crate::model::Duration,
    ) -> Result<VerifiedClip, VerifiedClipError> {
        self.validate_consistency(video_duration)?;

        Ok(VerifiedClip {
            song_title: self.song_title,
            liver_ids: self.liver_ids,
            external_artists_name: self.external_artists_name,
            clipped_video_id: self.clipped_video_id,
            start_time: self.start_time,
            end_time: self.end_time,
            uuid: self.uuid,
        })
    }

    /// `Self`が存在できるか検証
    fn validate_consistency(
        &self,
        video_duration: &crate::model::Duration,
    ) -> Result<(), VerifiedClipError> {
        super::validate_start_end_times(&self.start_time, &self.end_time).map_err(
            |_| VerifiedClipError::InvalidClipTimeRange {
                song_title: self.song_title.clone(),
                start_time: self.start_time.clone(),
                end_time: self.end_time.clone(),
            },
        )?;
        self.validate_video_duration(video_duration)?;
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
    pub(crate) fn get_song_title(&self) -> &str {
        &self.song_title
    }
    pub(crate) fn get_liver_ids(&self) -> &artistctl::model::LiverIds {
        &self.liver_ids
    }
    pub(crate) fn get_external_artists_name(
        &self,
    ) -> Option<&artistctl::model::ExternalArtistsName> {
        self.external_artists_name.as_ref()
    }
    pub(crate) fn get_clipped_video_id(&self) -> Option<&crate::model::VideoId> {
        self.clipped_video_id.as_ref()
    }
    pub(crate) fn get_start_time(&self) -> &crate::model::Duration {
        &self.start_time
    }
    pub(crate) fn get_end_time(&self) -> &crate::model::Duration {
        &self.end_time
    }
    pub(crate) fn get_uuid(&self) -> &crate::model::UuidVer4 {
        &self.uuid
    }

    pub fn uuid_string(&self) -> String {
        self.uuid.to_string()
    }

    pub fn artist_ids(&self) -> Vec<&str> {
        self.liver_ids.to_vec()
    }

    pub(super) fn into_inner(self) -> VerifiedClipInner {
        VerifiedClipInner {
            song_title: self.song_title,
            liver_ids: self.liver_ids,
            external_artists_name: self.external_artists_name,
            clipped_video_id: self.clipped_video_id,
            start_time: self.start_time,
            end_time: self.end_time,
            uuid: self.uuid,
        }
    }
}

// MARK: For Tests

#[cfg(test)]
impl VerifiedClip {
    // anonymousに対応するように作成

    fn self_a_initialize(ini_1: VerifiedClipInitializer) -> Self {
        ini_1
            .init(&crate::model::Duration::from_secs_u16(120))
            .expect("Failed to create VerifiedClip A1")
    }
    pub(crate) fn self_a_1() -> Self {
        let ini_1 = VerifiedClipInitializer {
            song_title: "Test Song A1".to_string(),
            liver_ids: artistctl::model::LiverIds::self_1(),
            external_artists_name: Some(artistctl::model::ExternalArtistsName::self_1()),
            clipped_video_id: None,
            start_time: crate::model::Duration::from_secs_u16(5),
            end_time: crate::model::Duration::from_secs_u16(10),
            uuid: crate::model::UuidVer4::self_partly_rand(0xa1),
        };
        Self::self_a_initialize(ini_1)
    }
    pub(crate) fn self_a_2() -> Self {
        let ini_2 = VerifiedClipInitializer {
            song_title: "Test Song A2".to_string(),
            liver_ids: artistctl::model::LiverIds::self_2(),
            external_artists_name: None,
            clipped_video_id: Some(crate::model::VideoId::test_id_3()),
            start_time: crate::model::Duration::from_secs_u16(15),
            end_time: crate::model::Duration::from_secs_u16(20),
            uuid: crate::model::UuidVer4::self_partly_rand(0xa2),
        };
        Self::self_a_initialize(ini_2)
    }
    pub(crate) fn self_a_3() -> Self {
        let ini_3 = VerifiedClipInitializer {
            song_title: "Test Song A3".to_string(),
            liver_ids: artistctl::model::LiverIds::self_3(),
            external_artists_name: Some(artistctl::model::ExternalArtistsName::self_2()),
            clipped_video_id: None,
            start_time: crate::model::Duration::from_secs_u16(25),
            end_time: crate::model::Duration::from_secs_u16(30),
            uuid: crate::model::UuidVer4::self_partly_rand(0xa3),
        };
        Self::self_a_initialize(ini_3)
    }

    fn self_b_initialize(ini_2: VerifiedClipInitializer) -> Self {
        ini_2
            .init(&crate::model::Duration::from_secs_u16(120))
            .expect("Failed to create VerifiedClip A2")
    }
    pub(crate) fn self_b_1() -> Self {
        let ini_b1 = VerifiedClipInitializer {
            song_title: "Test Song B1".to_string(),
            liver_ids: artistctl::model::LiverIds::self_1(),
            external_artists_name: Some(artistctl::model::ExternalArtistsName::self_3()),
            clipped_video_id: Some(crate::model::VideoId::test_id_4()),
            start_time: crate::model::Duration::from_secs_u16(7),
            end_time: crate::model::Duration::from_secs_u16(17),
            uuid: crate::model::UuidVer4::self_partly_rand(0xb1),
        };
        Self::self_b_initialize(ini_b1)
    }
    pub(crate) fn self_b_2() -> Self {
        let ini_b2 = VerifiedClipInitializer {
            song_title: "Test Song B2".to_string(),
            liver_ids: artistctl::model::LiverIds::self_2(),
            external_artists_name: None,
            clipped_video_id: None,
            start_time: crate::model::Duration::from_secs_u16(27),
            end_time: crate::model::Duration::from_secs_u16(37),
            uuid: crate::model::UuidVer4::self_partly_rand(0xb2),
        };
        Self::self_b_initialize(ini_b2)
    }
    pub(crate) fn self_b_3() -> Self {
        let ini_b3 = VerifiedClipInitializer {
            song_title: "Test Song B3".to_string(),
            liver_ids: artistctl::model::LiverIds::self_1(),
            external_artists_name: None,
            clipped_video_id: Some(crate::model::VideoId::test_id_5()),
            start_time: crate::model::Duration::from_secs_u16(47),
            end_time: crate::model::Duration::from_secs_u16(57),
            uuid: crate::model::UuidVer4::self_partly_rand(0xb3),
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
        assert_eq!(clip_a_1.liver_ids, artistctl::model::LiverIds::self_1());
        assert_eq!(clip_a_1.external_artists_name, Some(artistctl::model::ExternalArtistsName::self_1()));
        assert!(clip_a_1.clipped_video_id.is_none());
        assert_eq!(clip_a_1.start_time, crate::model::Duration::from_secs_u16(5));
        assert_eq!(clip_a_1.end_time, crate::model::Duration::from_secs_u16(10));
        assert_eq!(clip_a_1.uuid, crate::model::UuidVer4::self_partly_rand(0xa1));

        let clip_a_2 = VerifiedClip::self_a_2();
        assert_eq!(clip_a_2.song_title, "Test Song A2");
        assert_eq!(clip_a_2.liver_ids, artistctl::model::LiverIds::self_2());
        assert_eq!(clip_a_2.external_artists_name, None);
        assert_eq!(clip_a_2.clipped_video_id, Some(crate::model::VideoId::test_id_3()));
        assert_eq!(clip_a_2.start_time, crate::model::Duration::from_secs_u16(15));
        assert_eq!(clip_a_2.end_time, crate::model::Duration::from_secs_u16(20));
        assert_eq!(clip_a_2.uuid, crate::model::UuidVer4::self_partly_rand(0xa2));

        let clip_a_3 = VerifiedClip::self_a_3();
        assert_eq!(clip_a_3.song_title, "Test Song A3");
        assert_eq!(clip_a_3.liver_ids, artistctl::model::LiverIds::self_3());
        assert_eq!(clip_a_3.external_artists_name, Some(artistctl::model::ExternalArtistsName::self_2()));
        assert!(clip_a_3.clipped_video_id.is_none());
        assert_eq!(clip_a_3.start_time, crate::model::Duration::from_secs_u16(25));
        assert_eq!(clip_a_3.end_time, crate::model::Duration::from_secs_u16(30));
        assert_eq!(clip_a_3.uuid, crate::model::UuidVer4::self_partly_rand(0xa3));

        let clip_b_1 = VerifiedClip::self_b_1();
        assert_eq!(clip_b_1.song_title, "Test Song B1");
        assert_eq!(clip_b_1.liver_ids, artistctl::model::LiverIds::self_1());
        assert_eq!(clip_b_1.external_artists_name, Some(artistctl::model::ExternalArtistsName::self_3()));
        assert_eq!(clip_b_1.clipped_video_id, Some(crate::model::VideoId::test_id_4()));
        assert_eq!(clip_b_1.start_time, crate::model::Duration::from_secs_u16(7));
        assert_eq!(clip_b_1.end_time, crate::model::Duration::from_secs_u16(17));
        assert_eq!(clip_b_1.uuid, crate::model::UuidVer4::self_partly_rand(0xb1));

        let clip_b_2 = VerifiedClip::self_b_2();
        assert_eq!(clip_b_2.song_title, "Test Song B2");
        assert_eq!(clip_b_2.liver_ids, artistctl::model::LiverIds::self_2());
        assert_eq!(clip_b_2.external_artists_name, None);
        assert!(clip_b_2.clipped_video_id.is_none());
        assert_eq!(clip_b_2.start_time, crate::model::Duration::from_secs_u16(27));
        assert_eq!(clip_b_2.end_time, crate::model::Duration::from_secs_u16(37));
        assert_eq!(clip_b_2.uuid, crate::model::UuidVer4::self_partly_rand(0xb2));

        let clip_b_3 = VerifiedClip::self_b_3();
        assert_eq!(clip_b_3.song_title, "Test Song B3");
        assert_eq!(clip_b_3.liver_ids, artistctl::model::LiverIds::self_1());
        assert_eq!(clip_b_3.external_artists_name, None);
        assert_eq!(clip_b_3.clipped_video_id, Some(crate::model::VideoId::test_id_5()));
        assert_eq!(clip_b_3.start_time, crate::model::Duration::from_secs_u16(47));
        assert_eq!(clip_b_3.end_time, crate::model::Duration::from_secs_u16(57));
        assert_eq!(clip_b_3.uuid, crate::model::UuidVer4::self_partly_rand(0xb3));
    }

    #[test]
    fn test_verified_clip_validate_video_duration() {
        // 正常
        let verified_initializer = VerifiedClipInitializer {
            song_title: "Test Song".to_string(),
            liver_ids: artistctl::model::LiverIds::self_1(),
            external_artists_name: None,
            clipped_video_id: None,
            start_time: crate::model::Duration::from_secs_u16(30),
            end_time: crate::model::Duration::from_secs_u16(40),
            uuid: crate::model::UuidVer4::self_4(),
        };
        verified_initializer
            .validate_video_duration(&crate::model::Duration::from_secs_u16(60))
            .expect("Video duration validation should succeed");

        // 異常, `start_time`か`end_time`動画の長さを超えている
        let verified_initializer = VerifiedClipInitializer {
            song_title: "Test Song".to_string(),
            liver_ids: artistctl::model::LiverIds::self_1(),
            external_artists_name: None,
            clipped_video_id: None,
            start_time: crate::model::Duration::from_secs_u16(30),
            end_time: crate::model::Duration::from_secs_u16(50),
            uuid: crate::model::UuidVer4::self_4(),
        };
        let result = verified_initializer
            .validate_video_duration(&crate::model::Duration::from_secs_u16(40));
        assert!(result.is_err());
    }
}
