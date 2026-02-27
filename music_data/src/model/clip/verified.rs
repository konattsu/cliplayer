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
    /// 切り抜いた動画が存在した場合の動画id
    #[serde(skip_serializing_if = "Option::is_none")]
    clipped_video_id: Option<crate::model::VideoId>,
    /// 曲が始まる時間
    start_time: crate::model::Duration,
    /// 曲が終わる時間
    end_time: crate::model::Duration,
    /// タグ
    #[serde(skip_serializing_if = "Option::is_none")]
    clip_tags: Option<crate::model::ClipTags>,
    /// uuid
    uuid: crate::model::UuidVer4,
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
    /// 切り抜いた動画が存在した場合の動画id
    pub(super) clipped_video_id: Option<crate::model::VideoId>,
    /// 曲が始まる時間
    pub(super) start_time: crate::model::Duration,
    /// 曲が終わる時間
    pub(super) end_time: crate::model::Duration,
    /// タグ
    pub(super) clip_tags: Option<crate::model::ClipTags>,
    /// uuid
    pub(super) uuid: crate::model::UuidVer4,
    /// 音量の正規化時に設定すべき音量
    pub(super) volume_percent: Option<crate::model::VolumePercent>,
}

pub(super) struct VerifiedClipInitializer {
    /// 曲名
    pub(super) song_title: String,
    /// 内部アーティストの一覧
    pub(super) artists: crate::model::InternalArtists,
    /// 外部アーティストの一覧
    pub(super) external_artists: Option<crate::model::ExternalArtists>,
    /// 切り抜いた動画が存在した場合の動画id
    pub(super) clipped_video_id: Option<crate::model::VideoId>,
    /// 曲が始まる時間
    pub(super) start_time: crate::model::Duration,
    /// 曲が終わる時間
    pub(super) end_time: crate::model::Duration,
    /// タグ
    pub(super) clip_tags: Option<crate::model::ClipTags>,
    /// uuid
    pub(super) uuid: crate::model::UuidVer4,
    /// 音量の正規化時に設定すべき音量
    pub(super) volume_percent: Option<crate::model::VolumePercent>,
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
            artists: self.artists,
            external_artists: self.external_artists,
            clipped_video_id: self.clipped_video_id,
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
    pub(crate) fn get_artists(&self) -> &crate::model::InternalArtists {
        &self.artists
    }
    pub(crate) fn get_external_artists(
        &self,
    ) -> Option<&crate::model::ExternalArtists> {
        self.external_artists.as_ref()
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
    pub(crate) fn get_clip_tags(&self) -> Option<&crate::model::ClipTags> {
        self.clip_tags.as_ref()
    }
    pub(crate) fn get_uuid(&self) -> &crate::model::UuidVer4 {
        &self.uuid
    }
    pub(crate) fn get_volume_percent(&self) -> Option<&crate::model::VolumePercent> {
        self.volume_percent.as_ref()
    }

    pub(super) fn into_inner(self) -> VerifiedClipInner {
        VerifiedClipInner {
            song_title: self.song_title,
            artists: self.artists,
            external_artists: self.external_artists,
            clipped_video_id: self.clipped_video_id,
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
    // anonymousに対応するように作成

    fn self_a_initialize(ini_1: VerifiedClipInitializer) -> Self {
        ini_1
            .init(&crate::model::Duration::from_secs_u16(120))
            .expect("Failed to create VerifiedClip A1")
    }
    pub(crate) fn self_a_1() -> Self {
        let ini_1 = VerifiedClipInitializer {
            song_title: "Test Song A1".to_string(),
            artists: crate::model::InternalArtists::self_1(),
            external_artists: Some(crate::model::ExternalArtists::self_1()),
            clipped_video_id: None,
            start_time: crate::model::Duration::from_secs_u16(5),
            end_time: crate::model::Duration::from_secs_u16(10),
            clip_tags: None,
            uuid: crate::model::UuidVer4::self_partly_rand(0xa1),
            volume_percent: None,
        };
        Self::self_a_initialize(ini_1)
    }
    pub(crate) fn self_a_2() -> Self {
        let ini_2 = VerifiedClipInitializer {
            song_title: "Test Song A2".to_string(),
            artists: crate::model::InternalArtists::self_2(),
            external_artists: None,
            clipped_video_id: Some(crate::model::VideoId::test_id_3()),
            start_time: crate::model::Duration::from_secs_u16(15),
            end_time: crate::model::Duration::from_secs_u16(20),
            clip_tags: None,
            uuid: crate::model::UuidVer4::self_partly_rand(0xa2),
            volume_percent: None,
        };
        Self::self_a_initialize(ini_2)
    }
    pub(crate) fn self_a_3() -> Self {
        let ini_3 = VerifiedClipInitializer {
            song_title: "Test Song A3".to_string(),
            artists: crate::model::InternalArtists::self_3(),
            external_artists: Some(crate::model::ExternalArtists::self_2()),
            clipped_video_id: None,
            start_time: crate::model::Duration::from_secs_u16(25),
            end_time: crate::model::Duration::from_secs_u16(30),
            clip_tags: Some(crate::model::ClipTags::self_2()),
            uuid: crate::model::UuidVer4::self_partly_rand(0xa3),
            volume_percent: None,
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
            artists: crate::model::InternalArtists::self_1(),
            external_artists: Some(crate::model::ExternalArtists::self_3()),
            clipped_video_id: Some(crate::model::VideoId::test_id_4()),
            start_time: crate::model::Duration::from_secs_u16(7),
            end_time: crate::model::Duration::from_secs_u16(17),
            clip_tags: Some(crate::model::ClipTags::self_3()),
            uuid: crate::model::UuidVer4::self_partly_rand(0xb1),
            volume_percent: None,
        };
        Self::self_b_initialize(ini_b1)
    }
    pub(crate) fn self_b_2() -> Self {
        let ini_b2 = VerifiedClipInitializer {
            song_title: "Test Song B2".to_string(),
            artists: crate::model::InternalArtists::self_2(),
            external_artists: None,
            clipped_video_id: None,
            start_time: crate::model::Duration::from_secs_u16(27),
            end_time: crate::model::Duration::from_secs_u16(37),
            clip_tags: Some(crate::model::ClipTags::self_1()),
            uuid: crate::model::UuidVer4::self_partly_rand(0xb2),
            volume_percent: None,
        };
        Self::self_b_initialize(ini_b2)
    }
    pub(crate) fn self_b_3() -> Self {
        let ini_b3 = VerifiedClipInitializer {
            song_title: "Test Song B3".to_string(),
            artists: crate::model::InternalArtists::self_1(),
            external_artists: None,
            clipped_video_id: Some(crate::model::VideoId::test_id_5()),
            start_time: crate::model::Duration::from_secs_u16(47),
            end_time: crate::model::Duration::from_secs_u16(57),
            clip_tags: None,
            uuid: crate::model::UuidVer4::self_partly_rand(0xb3),
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
        assert_eq!(clip_a_1.artists, crate::model::InternalArtists::self_1());
        assert_eq!(clip_a_1.external_artists, Some(crate::model::ExternalArtists::self_1()));
        assert!(clip_a_1.clipped_video_id.is_none());
        assert_eq!(clip_a_1.start_time, crate::model::Duration::from_secs_u16(5));
        assert_eq!(clip_a_1.end_time, crate::model::Duration::from_secs_u16(10));
        assert_eq!(clip_a_1.clip_tags, None);
        assert_eq!(clip_a_1.uuid, crate::model::UuidVer4::self_partly_rand(0xa1));
        assert_eq!(clip_a_1.volume_percent, None);

        let clip_a_2 = VerifiedClip::self_a_2();
        assert_eq!(clip_a_2.song_title, "Test Song A2");
        assert_eq!(clip_a_2.artists, crate::model::InternalArtists::self_2());
        assert_eq!(clip_a_2.external_artists, None);
        assert_eq!(clip_a_2.clipped_video_id, Some(crate::model::VideoId::test_id_3()));
        assert_eq!(clip_a_2.start_time, crate::model::Duration::from_secs_u16(15));
        assert_eq!(clip_a_2.end_time, crate::model::Duration::from_secs_u16(20));
        assert_eq!(clip_a_2.clip_tags, None);
        assert_eq!(clip_a_2.uuid, crate::model::UuidVer4::self_partly_rand(0xa2));
        assert_eq!(clip_a_2.volume_percent, None);

        let clip_a_3 = VerifiedClip::self_a_3();
        assert_eq!(clip_a_3.song_title, "Test Song A3");
        assert_eq!(clip_a_3.artists, crate::model::InternalArtists::self_3());
        assert_eq!(clip_a_3.external_artists, Some(crate::model::ExternalArtists::self_2()));
        assert!(clip_a_3.clipped_video_id.is_none());
        assert_eq!(clip_a_3.start_time, crate::model::Duration::from_secs_u16(25));
        assert_eq!(clip_a_3.end_time, crate::model::Duration::from_secs_u16(30));
        assert_eq!(clip_a_3.clip_tags, Some(crate::model::ClipTags::self_2()));
        assert_eq!(clip_a_3.uuid, crate::model::UuidVer4::self_partly_rand(0xa3));
        assert_eq!(clip_a_3.volume_percent, None);

        let clip_b_1 = VerifiedClip::self_b_1();
        assert_eq!(clip_b_1.song_title, "Test Song B1");
        assert_eq!(clip_b_1.artists, crate::model::InternalArtists::self_1());
        assert_eq!(clip_b_1.external_artists, Some(crate::model::ExternalArtists::self_3()));
        assert_eq!(clip_b_1.clipped_video_id, Some(crate::model::VideoId::test_id_4()));
        assert_eq!(clip_b_1.start_time, crate::model::Duration::from_secs_u16(7));
        assert_eq!(clip_b_1.end_time, crate::model::Duration::from_secs_u16(17));
        assert_eq!(clip_b_1.clip_tags, Some(crate::model::ClipTags::self_3()));
        assert_eq!(clip_b_1.uuid, crate::model::UuidVer4::self_partly_rand(0xb1));
        assert_eq!(clip_b_1.volume_percent, None);

        let clip_b_2 = VerifiedClip::self_b_2();
        assert_eq!(clip_b_2.song_title, "Test Song B2");
        assert_eq!(clip_b_2.artists, crate::model::InternalArtists::self_2());
        assert_eq!(clip_b_2.external_artists, None);
        assert!(clip_b_2.clipped_video_id.is_none());
        assert_eq!(clip_b_2.start_time, crate::model::Duration::from_secs_u16(27));
        assert_eq!(clip_b_2.end_time, crate::model::Duration::from_secs_u16(37));
        assert_eq!(clip_b_2.clip_tags, Some(crate::model::ClipTags::self_1()));
        assert_eq!(clip_b_2.uuid, crate::model::UuidVer4::self_partly_rand(0xb2));
        assert_eq!(clip_b_2.volume_percent, None);

        let clip_b_3 = VerifiedClip::self_b_3();
        assert_eq!(clip_b_3.song_title, "Test Song B3");
        assert_eq!(clip_b_3.artists, crate::model::InternalArtists::self_1());
        assert_eq!(clip_b_3.external_artists, None);
        assert_eq!(clip_b_3.clipped_video_id, Some(crate::model::VideoId::test_id_5()));
        assert_eq!(clip_b_3.start_time, crate::model::Duration::from_secs_u16(47));
        assert_eq!(clip_b_3.end_time, crate::model::Duration::from_secs_u16(57));
        assert_eq!(clip_b_3.clip_tags, None);
        assert_eq!(clip_b_3.uuid, crate::model::UuidVer4::self_partly_rand(0xb3));
        assert_eq!(clip_b_3.volume_percent, None);
    }

    #[test]
    fn test_verified_clip_validate_video_duration() {
        // 正常
        let verified_initializer = VerifiedClipInitializer {
            song_title: "Test Song".to_string(),
            artists: crate::model::InternalArtists::self_1(),
            external_artists: None,
            clipped_video_id: None,
            start_time: crate::model::Duration::from_secs_u16(30),
            end_time: crate::model::Duration::from_secs_u16(40),
            clip_tags: None,
            uuid: crate::model::UuidVer4::self_4(),
            volume_percent: None,
        };
        verified_initializer
            .validate_video_duration(&crate::model::Duration::from_secs_u16(60))
            .expect("Video duration validation should succeed");

        // 異常, `start_time`か`end_time`動画の長さを超えている
        let verified_initializer = VerifiedClipInitializer {
            song_title: "Test Song".to_string(),
            artists: crate::model::InternalArtists::self_1(),
            external_artists: None,
            clipped_video_id: None,
            start_time: crate::model::Duration::from_secs_u16(30),
            end_time: crate::model::Duration::from_secs_u16(50),
            clip_tags: None,
            uuid: crate::model::UuidVer4::self_4(),
            volume_percent: None,
        };
        let result = verified_initializer
            .validate_video_duration(&crate::model::Duration::from_secs_u16(40));
        assert!(result.is_err());
    }
}
