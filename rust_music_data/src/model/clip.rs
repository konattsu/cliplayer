// NOTE: ちょっと汚いけどstart_time, end_timeの検証をdeserializeエラーに埋め込んでいる
// 外部トレイト実装でなく独自メソッドの実装を検討したが, 上位層でこれらの構造体使うときを考えたときに
// Deserializeトレイト欲しかった

/// `start_time` < `end_time` の検証
///
/// - Ok: `start_time` < `end_time`のとき
/// - Error: `start_time` >= `end_time`のとき
fn validate_start_end_times(
    start_time: &crate::model::Duration,
    end_time: &crate::model::Duration,
) -> Result<(), String> {
    if start_time >= end_time {
        return Err(format!(
            "invalid clip time range: start({}) must be less than to end({})",
            start_time, end_time
        ));
    }
    Ok(())
}

/// クリップ
#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq)]
pub enum Clip {
    /// 識別済みのクリップ
    Identified(IdentifiedClip),
    /// 識別されていないクリップ
    Unidentified(UnidentifiedClip),
}

impl<'de> serde::Deserialize<'de> for Clip {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // フィールドの上側から評価されていくためフィールド順は非常に重要
        #[derive(serde::Deserialize)]
        #[serde(untagged)]
        enum RawClip {
            Identified(IdentifiedClip),
            Unidentified(UnidentifiedClip),
        }

        let raw: RawClip = serde::Deserialize::deserialize(deserializer)
            .map_err(serde::de::Error::custom)?;

        match raw {
            RawClip::Identified(clip) => Ok(Clip::Identified(clip)),
            RawClip::Unidentified(clip) => Ok(Clip::Unidentified(clip)),
        }
    }
}

// MARK: IdentifiedClip

/// 識別済みのクリップ情報
#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct IdentifiedClip {
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
    tags: Option<crate::model::TagList>,
    /// このクリップを識別するためのuuid
    uuid: crate::model::UuidVer7,
    /// 音量の正規化時に設定すべき音量
    volume_percent: Option<crate::model::VolumePercent>,
}

/// `IdentifiedClip`を初期化するための構造体
pub struct IdentifiedClipInitializer {
    /// 曲名
    pub song_title: String,
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
    /// このクリップを識別するためのuuid
    pub uuid: crate::model::UuidVer7,
    /// 音量の正規化時に設定すべき音量
    pub volume_percent: Option<crate::model::VolumePercent>,
}

impl IdentifiedClipInitializer {
    /// `IdentifiedClip`を作成
    ///
    /// - Error: `start_time` >= `end_time`のとき
    ///   - e.g. `start_time`: 5秒, `end_time`: 3秒
    pub fn init(self) -> Result<IdentifiedClip, String> {
        validate_start_end_times(&self.start_time, &self.end_time)?;

        Ok(IdentifiedClip {
            song_title: self.song_title,
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

// TODO 他にもつけたほうがいい制約を熟考する
//   Self::is_exists的なものに全て統合してもいい

// デシリアライズ時に `start_time` < `end_time` のバリデーションを行うため
// 独自にDeserializeトレイトを実装している
impl<'de> serde::Deserialize<'de> for IdentifiedClip {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct RawIdentifiedClip {
            song_title: String,
            artists: crate::model::InternalArtists,
            external_artists: Option<crate::model::ExternalArtists>,
            is_clipped: bool,
            start_time: crate::model::Duration,
            end_time: crate::model::Duration,
            tags: Option<crate::model::TagList>,
            uuid: crate::model::UuidVer7,
            volume_percent: Option<crate::model::VolumePercent>,
        }

        let raw: RawIdentifiedClip = serde::Deserialize::deserialize(deserializer)
            .map_err(serde::de::Error::custom)?;

        // `start_time` < `end_time` の検証
        validate_start_end_times(&raw.start_time, &raw.end_time)
            .map_err(serde::de::Error::custom)?;

        Ok(IdentifiedClip {
            song_title: raw.song_title,
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

impl IdentifiedClip {
    pub fn get_song_title(&self) -> &str {
        &self.song_title
    }
    pub fn get_artists(&self) -> &crate::model::InternalArtists {
        &self.artists
    }
    pub fn get_external_artists(&self) -> Option<&crate::model::ExternalArtists> {
        self.external_artists.as_ref()
    }
    pub fn is_clipped(&self) -> bool {
        self.is_clipped
    }
    pub fn get_start_time(&self) -> &crate::model::Duration {
        &self.start_time
    }
    pub fn get_end_time(&self) -> &crate::model::Duration {
        &self.end_time
    }
    pub fn get_tags(&self) -> Option<&crate::model::TagList> {
        self.tags.as_ref()
    }
    pub fn get_uuid(&self) -> &crate::model::UuidVer7 {
        &self.uuid
    }
    pub fn get_volume_percent(&self) -> Option<crate::model::VolumePercent> {
        self.volume_percent
    }

    pub fn into_clip(self) -> Clip {
        Clip::Identified(self)
    }
}

// MARK: UnidentifiedClip

/// 識別されていないクリップ情報
#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct UnidentifiedClip {
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
    tags: Option<crate::model::TagList>,
}

/// `UnidentifiedClip`を初期化するための構造体
pub struct UnidentifiedClipInitializer {
    /// 曲名
    pub song_title: String,
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

impl UnidentifiedClipInitializer {
    /// `UnidentifiedClip`を作成
    ///
    /// - Error: `start_time` >= `end_time`のとき
    ///  - e.g. `start_time`: 5秒, `end_time`: 3秒
    pub fn init(self) -> Result<UnidentifiedClip, String> {
        validate_start_end_times(&self.start_time, &self.end_time)?;

        Ok(UnidentifiedClip {
            song_title: self.song_title,
            artists: self.artists,
            external_artists: self.external_artists,
            is_clipped: self.is_clipped,
            start_time: self.start_time,
            end_time: self.end_time,
            tags: self.tags,
        })
    }
}

// デシリアライズ時に `start_time` < `end_time` のバリデーションを行うため
// 独自にDeserializeトレイトを実装している
impl<'de> serde::Deserialize<'de> for UnidentifiedClip {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct RawUnidentifiedClip {
            song_title: String,
            artists: crate::model::InternalArtists,
            external_artists: Option<crate::model::ExternalArtists>,
            is_clipped: bool,
            start_time: crate::model::Duration,
            end_time: crate::model::Duration,
            tags: Option<crate::model::TagList>,
        }

        let raw: RawUnidentifiedClip = serde::Deserialize::deserialize(deserializer)
            .map_err(serde::de::Error::custom)?;

        // `start_time` < `end_time` の検証
        validate_start_end_times(&raw.start_time, &raw.end_time)
            .map_err(serde::de::Error::custom)?;

        Ok(UnidentifiedClip {
            song_title: raw.song_title,
            artists: raw.artists,
            external_artists: raw.external_artists,
            is_clipped: raw.is_clipped,
            start_time: raw.start_time,
            end_time: raw.end_time,
            tags: raw.tags,
        })
    }
}

impl UnidentifiedClip {
    pub fn get_song_title(&self) -> &str {
        &self.song_title
    }
    pub fn get_artists(&self) -> &crate::model::InternalArtists {
        &self.artists
    }
    pub fn get_external_artists(&self) -> Option<&crate::model::ExternalArtists> {
        self.external_artists.as_ref()
    }
    pub fn is_clipped(&self) -> bool {
        self.is_clipped
    }
    pub fn get_start_time(&self) -> &crate::model::Duration {
        &self.start_time
    }
    pub fn get_end_time(&self) -> &crate::model::Duration {
        &self.end_time
    }
    pub fn get_tags(&self) -> Option<&crate::model::TagList> {
        self.tags.as_ref()
    }

    pub fn into_clip(self) -> Clip {
        Clip::Unidentified(self)
    }

    /// 与えられた`datetime`と`start_time`を基にUUIDを生成
    ///
    /// - 引数の`video_upload_date`は日付のみを使用し, 時刻の情報は無視される
    /// - 時刻の情報は`start_time`に基づいて生成される
    ///   - e.g. `2024-01-01T12:12:12Z`と`start_time`: 5秒: `2024-01-01T00:00:05Z`となる
    ///
    ///
    /// - panic: `datetime`をタイムスタンプに変換すると, 48bit符号なし整数で表現できないとき
    ///   - i.e. 1970年1月1日 - 約10895年でないとき
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

    pub fn into_identified(
        self,
        video_upload_date: &crate::model::VideoPublishedAt,
    ) -> IdentifiedClip {
        let uuid = self.generate_uuid(video_upload_date);
        // `IdentifiedClip`に変換
        IdentifiedClipInitializer {
            song_title: self.song_title,
            artists: self.artists,
            external_artists: self.external_artists,
            is_clipped: self.is_clipped,
            start_time: self.start_time,
            end_time: self.end_time,
            tags: self.tags,
            uuid,
            volume_percent: None,
        }
        .init()
        // UnidentifiedClipでも `start_time` < `end_time` は保証されているため
        // unwrap()でpanicにならない
        .unwrap()
    }
}

// MARK: Tests

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    // MARK: -- validate_start_end_times

    #[test]
    fn test_validate_start_end_times_valid() {
        let start = crate::model::Duration::from_secs(5);
        let end = crate::model::Duration::from_secs(10);
        assert!(validate_start_end_times(&start, &end).is_ok());
    }

    #[test]
    fn test_validate_start_end_times_invalid_equal() {
        let start = crate::model::Duration::from_secs(10);
        let end = crate::model::Duration::from_secs(10);
        let result = validate_start_end_times(&start, &end);
        assert!(result.is_err());
        assert!(!result.unwrap_err().is_empty());
    }

    #[test]
    fn test_validate_start_end_times_invalid_more_than() {
        let start = crate::model::Duration::from_secs(10);
        let end = crate::model::Duration::from_secs(5);
        let result = validate_start_end_times(&start, &end);
        assert!(result.is_err());
        assert!(!result.unwrap_err().is_empty());
    }

    // MARK: -- IdentifiedClip

    #[test]
    fn test_identified_clip_deserialization() {
        use std::str::FromStr;

        let json = r#"{
            "song_title": "Test Clip",
            "artists": ["Aimer Test"],
            "external_artists": null,
            "is_clipped": true,
            "start_time": "PT0S",
            "end_time": "PT10S",
            "tags": ["tag1", "tag2"],
            "uuid": "01974f99-33b3-7d67-8f85-6949efe44d6e",
            "volume_percent": 50
        }"#;

        let clip: IdentifiedClip = serde_json::from_str(json).unwrap();
        assert_eq!(clip.get_song_title(), "Test Clip");
        assert_eq!(clip.get_artists().len(), 1);
        assert!(clip.get_external_artists().is_none());
        assert!(clip.is_clipped());
        assert_eq!(
            clip.get_start_time().clone().into_chrono_duration(),
            chrono::Duration::seconds(0)
        );
        assert_eq!(
            clip.get_end_time().clone().into_chrono_duration(),
            chrono::Duration::seconds(10)
        );
        assert_eq!(
            clip.get_tags().unwrap(),
            &crate::model::TagList::test_tag_list_1()
        );
        assert_eq!(
            clip.get_uuid(),
            &crate::model::UuidVer7::from_str("01974f99-33b3-7d67-8f85-6949efe44d6e")
                .unwrap()
        );
        assert_eq!(
            clip.get_volume_percent(),
            Some(crate::model::VolumePercent::new(50).unwrap())
        );
    }

    /// `start_time` >= `end_time` のときのデシリアライズエラーを検証
    #[test]
    fn test_identified_clip_deserialization_invalid() {
        let json = r#"{
            "song_title": "Test Clip",
            "artists": ["Aimer Test"],
            "external_artists": null,
            "is_clipped": true,
            "start_time": "PT10S",
            "end_time": "PT5S",
            "uuid": "01974f99-33b3-7d67-8f85-6949efe44d6e",
        }"#;

        let result: Result<IdentifiedClip, _> = serde_json::from_str(json);
        assert!(result.is_err());
        assert!(!result.unwrap_err().to_string().is_empty());
    }

    // MARK: -- UnidentifiedClip

    #[test]
    fn test_unidentified_clip_deserialization() {
        let json = r#"{
            "song_title": "Test Clip",
            "artists": ["Aimer Test"],
            "external_artists": null,
            "is_clipped": true,
            "start_time": "PT0S",
            "end_time": "PT10S"
        }"#;

        let clip: UnidentifiedClip = serde_json::from_str(json).unwrap();
        assert_eq!(clip.get_song_title(), "Test Clip");
        assert_eq!(clip.get_artists().len(), 1);
        assert!(clip.get_external_artists().is_none());
        assert!(clip.is_clipped());
        assert_eq!(
            clip.get_start_time().clone().into_chrono_duration(),
            chrono::Duration::seconds(0)
        );
        assert_eq!(
            clip.get_end_time().clone().into_chrono_duration(),
            chrono::Duration::seconds(10)
        );
    }

    /// `start_time` >= `end_time` のときのデシリアライズエラーを検証
    #[test]
    fn test_unidentified_clip_deserialization_invalid() {
        let json = r#"{
            "song_title": "Test Clip",
            "artists": ["Aimer Test"],
            "external_artists": null,
            "is_clipped": true,
            "start_time": "PT10S",
            "end_time": "PT5S",
        }"#;

        let result: Result<UnidentifiedClip, _> = serde_json::from_str(json);
        assert!(result.is_err());
        assert!(!result.unwrap_err().to_string().is_empty());
    }

    fn generate_uuid_test_helper(
        start_time: &crate::model::Duration,
    ) -> UnidentifiedClip {
        UnidentifiedClip {
            song_title: "Test Clip".to_string(),
            artists: crate::model::InternalArtists::test_name_1(),
            external_artists: None,
            is_clipped: true,
            start_time: start_time.clone(),
            end_time: crate::model::Duration::from_secs(10)
                .try_add(start_time)
                .unwrap(),
            tags: None,
        }
    }

    #[test]
    fn test_unidentified_clip_generate_uuid() {
        // uuidはstart_timeの時刻, date(引数)の日付を基に生成される. これを検証
        let start_time = crate::model::Duration::from_secs(5);
        let clip = generate_uuid_test_helper(&start_time);
        let date = crate::model::VideoPublishedAt::new(
            chrono::Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
        )
        .unwrap();
        let uuid = clip.generate_uuid(&date);
        assert_eq!(
            uuid.get_datetime(),
            chrono::Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 5).unwrap()
        );

        // uuid生成時にdate(引数)の時刻情報が落とされているかの検証
        let start_time = crate::model::Duration::from_secs(30);
        let clip = generate_uuid_test_helper(&start_time);
        let date = crate::model::VideoPublishedAt::new(
            chrono::Utc.with_ymd_and_hms(2025, 1, 1, 2, 2, 2).unwrap(),
        )
        .unwrap();
        let uuid = clip.generate_uuid(&date);
        assert_eq!(
            uuid.get_datetime(),
            // 時刻の情報は落ちるので, 2025-01-01T02:02:32Z ではない
            chrono::Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 30).unwrap()
        );
    }
}
