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
    /// 切り抜いた動画が存在した場合の動画id
    clipped_video_id: Option<crate::model::VideoId>,
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
    /// 切り抜いた動画が存在した場合の動画id
    clipped_video_id: Option<crate::model::VideoId>,
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
            clipped_video_id: self.clipped_video_id,
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
            clipped_video_id: Option<crate::model::VideoId>,
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
            clipped_video_id: raw.clipped_video_id,
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
        super::verified::VerifiedClipInitializer {
            song_title: self.song_title,
            artists: self.artists,
            external_artists: self.external_artists,
            clipped_video_id: self.clipped_video_id,
            start_time: self.start_time,
            end_time: self.end_time,
            clip_tags: self.clip_tags,
            uuid,
            // データを保持していないのでNone
            volume_percent: None,
        }
        .init(video_duration)
    }

    pub(crate) fn vec_to_markdown(clips: &[Self]) -> String {
        let mut md_str = "| # | Song Title | Clip Range | Artists | Other |\n|:---|:---|:---|:---|:---|\n".to_string();
        for (i, clip) in clips.iter().enumerate() {
            let clip_range = format!(
                "{} - {}",
                clip.start_time.to_short_str(),
                clip.end_time.to_short_str()
            );
            let other_info = clip.to_markdown_other_info();
            let row = format!(
                "| {} | {} | {} | {} | {} |\n",
                i + 1,
                clip.song_title,
                clip_range,
                clip.artists.get_artists_ja_name().join("<br>"),
                other_info
            );
            md_str.push_str(&row);
        }
        md_str
    }

    fn to_markdown_other_info(&self) -> String {
        let mut base_str = Vec::new();

        // 60*7 = 420sec = 7min
        if self.start_time.as_secs() + 420 < self.end_time.as_secs() {
            base_str.push("clip range too long?".to_string());
        }
        if let Some(external_artists) = &self.external_artists {
            base_str.push(format!(
                "external artists: {}",
                external_artists.to_vec().join(", ")
            ));
        }
        if let Some(clipped_video_id) = &self.clipped_video_id {
            base_str.push(format!(
                "clipped video exists [here](https://youtu.be/{clipped_video_id})",
            ));
        }
        if let Some(clip_tags) = &self.clip_tags {
            base_str.push(format!("cTags: {}", clip_tags.to_vec().join(", ")));
        }
        base_str.join("<br>")
    }
}

// MARK: For Tests
#[cfg(test)]
impl AnonymousClip {
    pub(crate) fn self_a_1() -> Self {
        AnonymousClipInitializer {
            song_title: "Test Song A1".to_string(),
            artists: crate::model::InternalArtists::self_1(),
            external_artists: Some(crate::model::ExternalArtists::self_1()),
            clipped_video_id: None,
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
            artists: crate::model::InternalArtists::self_2(),
            external_artists: None,
            clipped_video_id: Some(crate::model::VideoId::test_id_3()),
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
            artists: crate::model::InternalArtists::self_3(),
            external_artists: Some(crate::model::ExternalArtists::self_2()),
            clipped_video_id: None,
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
            artists: crate::model::InternalArtists::self_1(),
            external_artists: Some(crate::model::ExternalArtists::self_3()),
            clipped_video_id: Some(crate::model::VideoId::test_id_4()),
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
            artists: crate::model::InternalArtists::self_2(),
            external_artists: None,
            clipped_video_id: None,
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
            artists: crate::model::InternalArtists::self_4(),
            external_artists: None,
            clipped_video_id: Some(crate::model::VideoId::test_id_5()),
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
        assert_eq!(clip_a_1.artists, crate::model::InternalArtists::self_1());
        assert_eq!(clip_a_1.external_artists, Some(crate::model::ExternalArtists::self_1()));
        assert!(clip_a_1.clipped_video_id.is_none());
        assert_eq!(clip_a_1.start_time, crate::model::Duration::from_secs_u16(5));
        assert_eq!(clip_a_1.end_time, crate::model::Duration::from_secs_u16(10));
        assert_eq!(clip_a_1.clip_tags, None);

        let clip_a_2 = AnonymousClip::self_a_2();
        assert_eq!(clip_a_2.song_title, "Test Song A2");
        assert_eq!(clip_a_2.artists, crate::model::InternalArtists::self_2());
        assert_eq!(clip_a_2.external_artists, None);
        assert_eq!(clip_a_2.clipped_video_id, Some(crate::model::VideoId::test_id_3()));
        assert_eq!(clip_a_2.start_time, crate::model::Duration::from_secs_u16(15));
        assert_eq!(clip_a_2.end_time, crate::model::Duration::from_secs_u16(20));
        assert_eq!(clip_a_2.clip_tags, None);

        let clip_a_3 = AnonymousClip::self_a_3();
        assert_eq!(clip_a_3.song_title, "Test Song A3");
        assert_eq!(clip_a_3.artists, crate::model::InternalArtists::self_3());
        assert_eq!(clip_a_3.external_artists, Some(crate::model::ExternalArtists::self_2()));
        assert!(clip_a_3.clipped_video_id.is_none());
        assert_eq!(clip_a_3.start_time, crate::model::Duration::from_secs_u16(25));
        assert_eq!(clip_a_3.end_time, crate::model::Duration::from_secs_u16(30));
        assert_eq!(clip_a_3.clip_tags, Some(crate::model::ClipTags::self_2()));

        let clip_b_1 = AnonymousClip::self_b_1();
        assert_eq!(clip_b_1.song_title, "Test Song B1");
        assert_eq!(clip_b_1.artists, crate::model::InternalArtists::self_1());
        assert_eq!(clip_b_1.external_artists, Some(crate::model::ExternalArtists::self_3()));
        assert_eq!(clip_b_1.clipped_video_id, Some(crate::model::VideoId::test_id_4()));
        assert_eq!(clip_b_1.start_time, crate::model::Duration::from_secs_u16(7));
        assert_eq!(clip_b_1.end_time, crate::model::Duration::from_secs_u16(17));
        assert_eq!(clip_b_1.clip_tags, Some(crate::model::ClipTags::self_3()));

        let clip_b_2 = AnonymousClip::self_b_2();
        assert_eq!(clip_b_2.song_title, "Test Song B2");
        assert_eq!(clip_b_2.artists, crate::model::InternalArtists::self_2());
        assert_eq!(clip_b_2.external_artists, None);
        assert!(clip_b_2.clipped_video_id.is_none());
        assert_eq!(clip_b_2.start_time, crate::model::Duration::from_secs_u16(27));
        assert_eq!(clip_b_2.end_time, crate::model::Duration::from_secs_u16(37));
        assert_eq!(clip_b_2.clip_tags, Some(crate::model::ClipTags::self_1()));

        let clip_b_3 = AnonymousClip::self_b_3();
        assert_eq!(clip_b_3.song_title, "Test Song B3");
        assert_eq!(clip_b_3.artists, crate::model::InternalArtists::self_4());
        assert_eq!(clip_b_3.external_artists, None);
        assert_eq!(clip_b_3.clipped_video_id, Some(crate::model::VideoId::test_id_5()));
        assert_eq!(clip_b_3.start_time, crate::model::Duration::from_secs_u16(47));
        assert_eq!(clip_b_3.end_time, crate::model::Duration::from_secs_u16(57));
        assert_eq!(clip_b_3.clip_tags, None);
    }

    const ANONYMOUS_CLIP_JSON_VALID: &str = r#"
    {
        "songTitle": "Test Song 1",
        "artists": ["aimer-test"],
        "externalArtists": ["Apple Mike"],
        "clippedVideoId": null,
        "startTime": "PT5S",
        "endTime": "PT10S",
        "clipTags": ["Test Clip Tag1"]
    }"#;

    // `startTime` >= `endTime`
    const ANONYMOUS_CLIP_JSON_INVALID: &str = r#"
    {
        "songTitle": "Test Song 2",
        "artists": ["aimer-test"],
        "externalArtists": ["Apple Mike"],
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
        assert_eq!(clip.artists, crate::model::InternalArtists::self_1());
        assert_eq!(
            clip.external_artists,
            Some(crate::model::ExternalArtists::self_1())
        );
        assert!(clip.clipped_video_id.is_none());
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
            artists: crate::model::InternalArtists::self_1(),
            external_artists: Some(crate::model::ExternalArtists::self_1()),
            clipped_video_id: None,
            start_time: crate::model::Duration::from_secs_u16(15),
            end_time: crate::model::Duration::from_secs_u16(20),
            clip_tags: Some(crate::model::ClipTags::self_1()),
        };
        let result = valid_initializer.init();
        assert!(result.is_ok());

        let invalid_initializer = AnonymousClipInitializer {
            song_title: "Test Song 4".to_string(),
            artists: crate::model::InternalArtists::self_1(),
            external_artists: Some(crate::model::ExternalArtists::self_1()),
            clipped_video_id: None,
            start_time: crate::model::Duration::from_secs_u16(25),
            // start >= end
            end_time: crate::model::Duration::from_secs_u16(20),
            clip_tags: Some(crate::model::ClipTags::self_1()),
        };
        let result = invalid_initializer.init();
        assert!(result.is_err());
    }

    #[test]
    fn test_anonymous_clip_to_markdown_other_info() {
        let clip = AnonymousClip::self_a_1();
        let other_info = clip.to_markdown_other_info();
        assert!(other_info.contains("external artists: Apple Mike"));

        let clip = AnonymousClip::self_a_2();
        let other_info = clip.to_markdown_other_info();
        assert!(
            other_info
                .contains("clipped video exists [here](https://youtu.be/33333333333)")
        );

        let clips = AnonymousClip::self_b_1();
        let other_info = clips.to_markdown_other_info();
        assert!(other_info.contains("external artists: Banana Mike"));
        assert!(
            other_info
                .contains("clipped video exists [here](https://youtu.be/44444444444)")
        );
        assert!(other_info.contains("cTags: Test Clip Tag3, Test Clip Tag4"));
    }

    #[test]
    fn test_anonymous_clip_vec_to_markdown() {
        let clips = vec![
            AnonymousClip::self_a_1(),
            AnonymousClip::self_a_2(),
            AnonymousClip::self_a_3(),
            AnonymousClip::self_b_1(),
            AnonymousClip::self_b_2(),
            AnonymousClip::self_b_3(),
        ];
        let markdown = AnonymousClip::vec_to_markdown(&clips);

        println!("md: {}", markdown);

        let expect = r#"| # | Song Title | Clip Range | Artists | Other |
|:---|:---|:---|:---|:---|
| 1 | Test Song A1 | 5 - 10 | エイマーテスト | external artists: Apple Mike |
| 2 | Test Song A2 | 15 - 20 | エイラアオイテスト | clipped video exists [here](https://youtu.be/33333333333) |
| 3 | Test Song A3 | 25 - 30 | リサテスト | external artists: Milk Mike<br>cTags: Test Clip Tag2 |
| 4 | Test Song B1 | 7 - 17 | エイマーテスト | external artists: Banana Mike<br>clipped video exists [here](https://youtu.be/44444444444)<br>cTags: Test Clip Tag3, Test Clip Tag4 |
| 5 | Test Song B2 | 27 - 37 | エイラアオイテスト | cTags: Test Clip Tag1 |
| 6 | Test Song B3 | 47 - 57 | エイマーテスト<br>エイラアオイテスト<br>リサテスト | clipped video exists [here](https://youtu.be/55555555555) |
"#;

        assert_eq!(expect, markdown);
    }
}
