/// 構造と型だけ適しているクリップ情報
///
/// - `start_time` < `end_time`のみの保証
/// - 外部の値との整合性の確認をしていない
#[derive(serde::Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct AnonymousClip {
    /// 曲名
    song_title: String,
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
    clip_tags: Option<crate::model::ClipTags>,
}

#[cfg(test)]
struct AnonymousClipInitializer {
    /// 曲名
    song_title: String,
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
    clip_tags: Option<crate::model::ClipTags>,
}

#[cfg(test)]
impl AnonymousClipInitializer {
    /// `AnonymousClip`を作成
    ///
    /// - Error: `start_time` >= `end_time`のとき
    ///   - e.g. `start_time`: 5秒, `end_time`: 3秒
    fn init(self) -> Result<AnonymousClip, String> {
        super::validate_start_end_times(&self.start_time, &self.end_time)?;

        Ok(AnonymousClip {
            song_title: self.song_title,
            artists: self.artists,
            external_artists: self.external_artists,
            is_clipped: self.is_clipped,
            start_time: self.start_time,
            end_time: self.end_time,
            clip_tags: self.clip_tags,
        })
    }
}

// デシリアライズ時に `start_time` < `end_time` のバリデーションを行うためのカスタムデシリアライザ
impl<'de> serde::Deserialize<'de> for AnonymousClip {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[serde(deny_unknown_fields)]
        struct RawAnonymousClip {
            song_title: String,
            artists: crate::model::InternalArtists,
            external_artists: Option<crate::model::ExternalArtists>,
            is_clipped: bool,
            start_time: crate::model::Duration,
            end_time: crate::model::Duration,
            clip_tags: Option<crate::model::ClipTags>,
        }

        let raw: RawAnonymousClip = serde::Deserialize::deserialize(deserializer)
            .map_err(serde::de::Error::custom)?;

        super::validate_start_end_times(&raw.start_time, &raw.end_time)
            .map_err(serde::de::Error::custom)?;

        Ok(AnonymousClip {
            song_title: raw.song_title,
            artists: raw.artists,
            external_artists: raw.external_artists,
            is_clipped: raw.is_clipped,
            start_time: raw.start_time,
            end_time: raw.end_time,
            clip_tags: raw.clip_tags,
        })
    }
}

impl AnonymousClip {
    pub(crate) fn get_start_time(&self) -> &crate::model::Duration {
        &self.start_time
    }
    pub(crate) fn get_end_time(&self) -> &crate::model::Duration {
        &self.end_time
    }
    pub(crate) fn get_song_title(&self) -> &str {
        &self.song_title
    }

    /// `AnonymousClip`を`VerifiedClip`に変換
    pub(crate) fn try_into_verified_clip(
        self,
        video_duration: &crate::model::Duration,
    ) -> Result<super::VerifiedClip, super::VerifiedClipError> {
        let uuid = crate::model::UuidVer4::generate();
        super::VerifiedClipInitializer {
            song_title: self.song_title,
            artists: self.artists,
            external_artists: self.external_artists,
            is_clipped: self.is_clipped,
            start_time: self.start_time,
            end_time: self.end_time,
            clip_tags: self.clip_tags,
            uuid,
            // データを保持していないのでNone
            volume_percent: None,
        }
        .init(video_duration)
    }
}

// MARK: For Tests
#[cfg(test)]
impl AnonymousClip {
    pub(crate) fn self_a_1() -> Self {
        AnonymousClipInitializer {
            song_title: "Test Song A1".to_string(),
            artists: crate::model::InternalArtists::test_name_1(),
            external_artists: Some(crate::model::ExternalArtists::test_name_1()),
            is_clipped: false,
            start_time: crate::model::Duration::from_secs_u16(5),
            end_time: crate::model::Duration::from_secs_u16(10),
            clip_tags: None,
        }
        .init()
        .unwrap()
    }
    pub(crate) fn self_a_2() -> Self {
        AnonymousClipInitializer {
            song_title: "Test Song A2".to_string(),
            artists: crate::model::InternalArtists::test_name_2(),
            external_artists: None,
            is_clipped: true,
            start_time: crate::model::Duration::from_secs_u16(15),
            end_time: crate::model::Duration::from_secs_u16(20),
            clip_tags: None,
        }
        .init()
        .unwrap()
    }
    pub(crate) fn self_a_3() -> Self {
        AnonymousClipInitializer {
            song_title: "Test Song A3".to_string(),
            artists: crate::model::InternalArtists::test_name_3(),
            external_artists: Some(crate::model::ExternalArtists::test_name_2()),
            is_clipped: false,
            start_time: crate::model::Duration::from_secs_u16(25),
            end_time: crate::model::Duration::from_secs_u16(30),
            clip_tags: Some(crate::model::ClipTags::self_2()),
        }
        .init()
        .unwrap()
    }
    pub(crate) fn self_b_1() -> Self {
        AnonymousClipInitializer {
            song_title: "Test Song B1".to_string(),
            artists: crate::model::InternalArtists::test_name_1(),
            external_artists: Some(crate::model::ExternalArtists::test_name_3()),
            is_clipped: true,
            start_time: crate::model::Duration::from_secs_u16(7),
            end_time: crate::model::Duration::from_secs_u16(17),
            clip_tags: Some(crate::model::ClipTags::self_3()),
        }
        .init()
        .unwrap()
    }
    pub(crate) fn self_b_2() -> Self {
        AnonymousClipInitializer {
            song_title: "Test Song B2".to_string(),
            artists: crate::model::InternalArtists::test_name_2(),
            external_artists: None,
            is_clipped: false,
            start_time: crate::model::Duration::from_secs_u16(27),
            end_time: crate::model::Duration::from_secs_u16(37),
            clip_tags: Some(crate::model::ClipTags::self_1()),
        }
        .init()
        .unwrap()
    }
    pub(crate) fn self_b_3() -> Self {
        AnonymousClipInitializer {
            song_title: "Test Song B3".to_string(),
            artists: crate::model::InternalArtists::test_name_1(),
            external_artists: None,
            is_clipped: true,
            start_time: crate::model::Duration::from_secs_u16(47),
            end_time: crate::model::Duration::from_secs_u16(57),
            clip_tags: None,
        }
        .init()
        .unwrap()
    }
}

// MARK: Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_anonymous_clip_for_test_methods() {
        let clip_a_1 = AnonymousClip::self_a_1();
        assert_eq!(clip_a_1.song_title, "Test Song A1");
        assert_eq!(clip_a_1.artists, crate::model::InternalArtists::test_name_1());
        assert_eq!(clip_a_1.external_artists, Some(crate::model::ExternalArtists::test_name_1()));
        assert!(!clip_a_1.is_clipped);
        assert_eq!(clip_a_1.start_time, crate::model::Duration::from_secs_u16(5));
        assert_eq!(clip_a_1.end_time, crate::model::Duration::from_secs_u16(10));
        assert_eq!(clip_a_1.clip_tags, None);

        let clip_a_2 = AnonymousClip::self_a_2();
        assert_eq!(clip_a_2.song_title, "Test Song A2");
        assert_eq!(clip_a_2.artists, crate::model::InternalArtists::test_name_2());
        assert_eq!(clip_a_2.external_artists, None);
        assert!(clip_a_2.is_clipped);
        assert_eq!(clip_a_2.start_time, crate::model::Duration::from_secs_u16(15));
        assert_eq!(clip_a_2.end_time, crate::model::Duration::from_secs_u16(20));
        assert_eq!(clip_a_2.clip_tags, None);

        let clip_a_3 = AnonymousClip::self_a_3();
        assert_eq!(clip_a_3.song_title, "Test Song A3");
        assert_eq!(clip_a_3.artists, crate::model::InternalArtists::test_name_3());
        assert_eq!(clip_a_3.external_artists, Some(crate::model::ExternalArtists::test_name_2()));
        assert!(!clip_a_3.is_clipped);
        assert_eq!(clip_a_3.start_time, crate::model::Duration::from_secs_u16(25));
        assert_eq!(clip_a_3.end_time, crate::model::Duration::from_secs_u16(30));
        assert_eq!(clip_a_3.clip_tags, Some(crate::model::ClipTags::self_2()));

        let clip_b_1 = AnonymousClip::self_b_1();
        assert_eq!(clip_b_1.song_title, "Test Song B1");
        assert_eq!(clip_b_1.artists, crate::model::InternalArtists::test_name_1());
        assert_eq!(clip_b_1.external_artists, Some(crate::model::ExternalArtists::test_name_3()));
        assert!(clip_b_1.is_clipped);
        assert_eq!(clip_b_1.start_time, crate::model::Duration::from_secs_u16(7));
        assert_eq!(clip_b_1.end_time, crate::model::Duration::from_secs_u16(17));
        assert_eq!(clip_b_1.clip_tags, Some(crate::model::ClipTags::self_3()));

        let clip_b_2 = AnonymousClip::self_b_2();
        assert_eq!(clip_b_2.song_title, "Test Song B2");
        assert_eq!(clip_b_2.artists, crate::model::InternalArtists::test_name_2());
        assert_eq!(clip_b_2.external_artists, None);
        assert!(!clip_b_2.is_clipped);
        assert_eq!(clip_b_2.start_time, crate::model::Duration::from_secs_u16(27));
        assert_eq!(clip_b_2.end_time, crate::model::Duration::from_secs_u16(37));
        assert_eq!(clip_b_2.clip_tags, Some(crate::model::ClipTags::self_1()));

        let clip_b_3 = AnonymousClip::self_b_3();
        assert_eq!(clip_b_3.song_title, "Test Song B3");
        assert_eq!(clip_b_3.artists, crate::model::InternalArtists::test_name_1());
        assert_eq!(clip_b_3.external_artists, None);
        assert!(clip_b_3.is_clipped);
        assert_eq!(clip_b_3.start_time, crate::model::Duration::from_secs_u16(47));
        assert_eq!(clip_b_3.end_time, crate::model::Duration::from_secs_u16(57));
        assert_eq!(clip_b_3.clip_tags, None);
    }

    const ANONYMOUS_CLIP_JSON_VALID: &str = r#"
    {
        "songTitle": "Test Song 1",
        "artists": ["Aimer Test"],
        "externalArtists": ["Apple Mike"],
        "isClipped": false,
        "startTime": "PT5S",
        "endTime": "PT10S",
        "clipTags": ["Test Clip Tag1"]
    }"#;

    // `startTime` >= `endTime`
    const ANONYMOUS_CLIP_JSON_INVALID: &str = r#"
    {
        "songTitle": "Test Song 2",
        "artists": ["Aimer Test"],
        "externalArtists": ["Apple Mike"],
        "isClipped": false,
        "startTime": "PT10S",
        "endTime": "PT5S",
        "ClipTags": ["Test Clip Tag1"]
    }"#;

    #[test]
    fn test_anonymous_clip_deserialize() {
        // 正常なデシリアライズ
        let clip: AnonymousClip =
            serde_json::from_str(ANONYMOUS_CLIP_JSON_VALID).unwrap();
        assert_eq!(clip.song_title, "Test Song 1");
        assert_eq!(clip.artists, crate::model::InternalArtists::test_name_1());
        assert_eq!(
            clip.external_artists,
            Some(crate::model::ExternalArtists::test_name_1())
        );
        assert!(!clip.is_clipped);
        assert_eq!(clip.start_time, crate::model::Duration::from_secs_u16(5));
        assert_eq!(clip.end_time, crate::model::Duration::from_secs_u16(10));
        assert_eq!(clip.clip_tags, Some(crate::model::ClipTags::self_1()));

        // 異常なデシリアライズ
        let result: Result<AnonymousClip, _> =
            serde_json::from_str(ANONYMOUS_CLIP_JSON_INVALID);
        assert!(result.is_err());
    }

    #[test]
    fn test_anonymous_clip_initializer_init() {
        let valid_initializer = AnonymousClipInitializer {
            song_title: "Test Song 3".to_string(),
            artists: crate::model::InternalArtists::test_name_1(),
            external_artists: Some(crate::model::ExternalArtists::test_name_1()),
            is_clipped: true,
            start_time: crate::model::Duration::from_secs_u16(15),
            end_time: crate::model::Duration::from_secs_u16(20),
            clip_tags: Some(crate::model::ClipTags::self_1()),
        };
        let result = valid_initializer.init();
        assert!(result.is_ok());

        let invalid_initializer = AnonymousClipInitializer {
            song_title: "Test Song 4".to_string(),
            artists: crate::model::InternalArtists::test_name_1(),
            external_artists: Some(crate::model::ExternalArtists::test_name_1()),
            is_clipped: false,
            start_time: crate::model::Duration::from_secs_u16(25),
            // start >= end
            end_time: crate::model::Duration::from_secs_u16(20),
            clip_tags: Some(crate::model::ClipTags::self_1()),
        };
        let result = invalid_initializer.init();
        assert!(result.is_err());
    }
}
