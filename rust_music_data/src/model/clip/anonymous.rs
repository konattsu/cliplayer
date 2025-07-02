/// 構造と型だけ適しているクリップ情報
///
/// - `start_time` < `end_time`のみの保証
/// - 外部の値との整合性の確認をしていない
#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct AnonymousClip {
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
}

pub struct AnonymousClipInitializer {
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
}

impl AnonymousClipInitializer {
    /// `AnonymousClip`を作成
    ///
    /// - Error: `start_time` >= `end_time`のとき
    ///   - e.g. `start_time`: 5秒, `end_time`: 3秒
    pub fn init(self) -> Result<AnonymousClip, String> {
        super::validate_start_end_times(&self.start_time, &self.end_time)?;

        Ok(AnonymousClip {
            song_title: self.song_title,
            song_title_jah: self.song_title_jah,
            artists: self.artists,
            external_artists: self.external_artists,
            is_clipped: self.is_clipped,
            start_time: self.start_time,
            end_time: self.end_time,
            tags: self.tags,
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
        struct RawAnonymousClip {
            song_title: String,
            song_title_jah: String,
            artists: crate::model::InternalArtists,
            external_artists: Option<crate::model::ExternalArtists>,
            is_clipped: bool,
            start_time: crate::model::Duration,
            end_time: crate::model::Duration,
            tags: Option<crate::model::TagList>,
        }

        let raw: RawAnonymousClip = serde::Deserialize::deserialize(deserializer)
            .map_err(serde::de::Error::custom)?;

        super::validate_start_end_times(&raw.start_time, &raw.end_time)
            .map_err(serde::de::Error::custom)?;

        Ok(AnonymousClip {
            song_title: raw.song_title,
            song_title_jah: raw.song_title_jah,
            artists: raw.artists,
            external_artists: raw.external_artists,
            is_clipped: raw.is_clipped,
            start_time: raw.start_time,
            end_time: raw.end_time,
            tags: raw.tags,
        })
    }
}

impl AnonymousClip {
    /// 与えられた`datetime`と`start_time`を基にUUIDを生成
    ///
    /// - 引数の`video_upload_date`は日付のみを使用し, 時刻の情報は無視される
    /// - 時刻の情報は`start_time`に基づいて生成される
    ///   - e.g. `2024-01-01T12:12:12Z`と`start_time`: 5秒: `2024-01-01T00:00:05Z`となる
    fn generate_uuid(
        &self,
        video_upload_date: &crate::model::VideoPublishedAt,
    ) -> crate::model::UuidVer7 {
        use chrono::{Datelike, TimeZone};

        let date = video_upload_date.as_chrono_datetime();
        // 時刻の情報を落とし, 日付のみにする
        let date = chrono::Utc
            .with_ymd_and_hms(date.year(), date.month(), date.day(), 0, 0, 0)
            .unwrap();
        // 日付に`start_time`の時間を加える
        let chrono_datetime = date + *self.start_time.as_chrono_duration();

        let dt = crate::model::VideoPublishedAt::new(chrono_datetime).unwrap();
        crate::model::UuidVer7::generate(&dt)
    }

    pub fn try_into_verified_clip(
        self,
        video_published_at: &crate::model::VideoPublishedAt,
        video_duration: &crate::model::Duration,
    ) -> Result<super::VerifiedClip, super::VerifiedClipError> {
        let uuid = self.generate_uuid(video_published_at);
        super::VerifiedClipInitializer {
            song_title: self.song_title,
            song_title_jah: self.song_title_jah,
            artists: self.artists,
            external_artists: self.external_artists,
            is_clipped: self.is_clipped,
            start_time: self.start_time,
            end_time: self.end_time,
            tags: self.tags,
            uuid,
            // データを保持していないのでNone
            volume_percent: None,
        }
        .init(video_published_at, video_duration)
    }
}

// MARK: Tests

#[cfg(test)]
mod tests {
    use super::*;

    const ANONYMOUS_CLIP_JSON_VALID: &str = r#"
    {
        "songTitle": "Test Song 1",
        "songTitleJah": "てすとそんぐいち",
        "artists": ["Aimer Test"],
        "externalArtists": ["Apple Mike"],
        "isClipped": false,
        "startTime": "PT5S",
        "endTime": "PT10S",
        "tags": ["tag1", "tag2"]
    }"#;

    // `startTime` >= `endTime`
    const ANONYMOUS_CLIP_JSON_INVALID: &str = r#"
    {
        "songTitle": "Test Song 2",
        "songTitleJah": "てすとそんぐに",
        "artists": ["Aimer Test"],
        "externalArtists": ["Apple Mike"],
        "isClipped": false,
        "startTime": "PT10S",
        "endTime": "PT5S",
        "tags": ["tag1", "tag2"]
    }"#;

    #[test]
    fn test_anonymous_clip_deserialize() {
        // 正常なデシリアライズ
        let clip: AnonymousClip =
            serde_json::from_str(ANONYMOUS_CLIP_JSON_VALID).unwrap();
        assert_eq!(clip.song_title, "Test Song 1");
        assert_eq!(clip.song_title_jah, "てすとそんぐいち");
        assert_eq!(clip.artists, crate::model::InternalArtists::test_name_1());
        assert_eq!(
            clip.external_artists,
            Some(crate::model::ExternalArtists::test_name_1())
        );
        assert!(!clip.is_clipped);
        assert_eq!(clip.start_time, crate::model::Duration::from_secs(5));
        assert_eq!(clip.end_time, crate::model::Duration::from_secs(10));
        assert_eq!(clip.tags, Some(crate::model::TagList::test_tag_list_1()));

        // 異常なデシリアライズ
        let result: Result<AnonymousClip, _> =
            serde_json::from_str(ANONYMOUS_CLIP_JSON_INVALID);
        assert!(result.is_err());
    }

    #[test]
    fn test_anonymous_clip_initializer_init() {
        let valid_initializer = AnonymousClipInitializer {
            song_title: "Test Song 3".to_string(),
            song_title_jah: "てすとそんぐさん".to_string(),
            artists: crate::model::InternalArtists::test_name_1(),
            external_artists: Some(crate::model::ExternalArtists::test_name_1()),
            is_clipped: true,
            start_time: crate::model::Duration::from_secs(15),
            end_time: crate::model::Duration::from_secs(20),
            tags: Some(crate::model::TagList::test_tag_list_1()),
        };
        let result = valid_initializer.init();
        assert!(result.is_ok());

        let invalid_initializer = AnonymousClipInitializer {
            song_title: "Test Song 4".to_string(),
            song_title_jah: "てすとそんぐよん".to_string(),
            artists: crate::model::InternalArtists::test_name_1(),
            external_artists: Some(crate::model::ExternalArtists::test_name_1()),
            is_clipped: false,
            start_time: crate::model::Duration::from_secs(25),
            // start >= end
            end_time: crate::model::Duration::from_secs(20),
            tags: Some(crate::model::TagList::test_tag_list_1()),
        };
        let result = invalid_initializer.init();
        assert!(result.is_err());
    }

    #[test]
    fn test_anonymous_clip_generate_uuid() {
        use chrono::TimeZone;

        let clip = AnonymousClipInitializer {
            song_title: "Test Song 5".to_string(),
            song_title_jah: "てすとそんぐご".to_string(),
            artists: crate::model::InternalArtists::test_name_1(),
            external_artists: Some(crate::model::ExternalArtists::test_name_1()),
            is_clipped: false,
            // この値が重要, UUIDv7の生成に使用される
            start_time: crate::model::Duration::from_secs(30),
            end_time: crate::model::Duration::from_secs(35),
            tags: Some(crate::model::TagList::test_tag_list_1()),
        }
        .init()
        .unwrap();

        // 動画公開時刻の日付とclipの`start_time`が正常に反映されているか
        let video_published_at = crate::model::VideoPublishedAt::new(
            chrono::Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        )
        .unwrap();
        let uuid = clip.generate_uuid(&video_published_at);
        assert_eq!(
            uuid.get_datetime(),
            chrono::Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 30).unwrap()
        );

        // 動画公開時間の時間情報が落とされているか
        let video_published_at = crate::model::VideoPublishedAt::new(
            chrono::Utc
                .with_ymd_and_hms(2024, 1, 1, 12, 12, 12)
                .unwrap(),
        )
        .unwrap();
        let uuid = clip.generate_uuid(&video_published_at);
        assert_eq!(
            uuid.get_datetime(),
            chrono::Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 30).unwrap()
        );
    }
}
