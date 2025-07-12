#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VerifiedClip {
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
pub enum VerifiedClipError {
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
    /// UUIDv7の日付(Y:M:D)と, 与えられた動画情報にある動画の公開日が一致しないとき
    #[error("uuid date({uuid_date}) does not match video date({video_date})")]
    UuidDateMismatch {
        uuid_date: chrono::NaiveDate,
        video_date: crate::model::VideoPublishedAt,
    },
    /// `start_time`or `end_time`の時間が, 与えられた動画情報にある動画の長さより長いとき
    #[error(
        "time exceeds video duration: start({start_time}), end({end_time}), video duration({video_duration})"
    )]
    TimeExceedsVideoDuration {
        start_time: crate::model::Duration,
        end_time: crate::model::Duration,
        video_duration: crate::model::Duration,
    },
}

pub struct VerifiedClipInner {
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
    pub clip_tags: Option<crate::model::ClipTags>,
    /// uuid
    pub uuid: crate::model::UuidVer7,
    /// 音量の正規化時に設定すべき音量
    pub volume_percent: Option<crate::model::VolumePercent>,
}

pub struct VerifiedClipInitializer {
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
    pub clip_tags: Option<crate::model::ClipTags>,
    /// uuid
    pub uuid: crate::model::UuidVer7,
    /// 音量の正規化時に設定すべき音量
    pub volume_percent: Option<crate::model::VolumePercent>,
}

impl VerifiedClipInitializer {
    /// `VerifiedClip`を作成
    ///
    /// - Error:
    ///   - `start_time` >= `end_time`のとき
    ///   - `uuid`のタイムスタンプの時間(h:m:s)と`start_time`の時間が一致しないとき
    ///   - `uuid`の日付(Y:M:D)と, 与えられた動画情報にある動画の公開日が一致しないとき
    ///   - `start_time`or `end_time`の時間が, 与えられた動画情報にある動画の長さより長いとき
    pub fn init(
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
    pub fn get_uuid(&self) -> &crate::model::UuidVer7 {
        &self.uuid
    }
    pub fn get_start_time(&self) -> &crate::model::Duration {
        &self.start_time
    }

    pub fn into_inner(self) -> VerifiedClipInner {
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

// MARK: Tests

#[cfg(test)]
mod tests {
    use super::*;

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
