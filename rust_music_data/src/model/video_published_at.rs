/// 動画のアップロード時間
///
/// タイムスタンプに直したとき, 符号なし48bitで表現できる範囲内であることを保証
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VideoPublishedAt(chrono::DateTime<chrono::Utc>);

impl VideoPublishedAt {
    /// 動画のアップデート時間を生成
    ///
    /// - Error: `upload_at`が符号なし48ビットのタイムスタンプの範囲外の場合
    ///   - i.e. `0..2^48-1` millisの範囲外
    pub fn new(upload_at: chrono::DateTime<chrono::Utc>) -> Result<Self, &'static str> {
        Self::validate_unsigned_48bit_timestamp(upload_at)?;
        Ok(VideoPublishedAt(upload_at))
    }

    pub fn as_chrono_datetime(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.0
    }

    pub fn into_chrono_duration(self) -> chrono::DateTime<chrono::Utc> {
        self.0
    }

    pub fn try_add(
        &self,
        other: &VideoPublishedAt,
    ) -> Result<VideoPublishedAt, &'static str> {
        let new_upload_at =
            self.0 + chrono::Duration::milliseconds(other.0.timestamp_millis());
        Self::validate_unsigned_48bit_timestamp(new_upload_at)?;
        Ok(VideoPublishedAt(new_upload_at))
    }

    fn validate_unsigned_48bit_timestamp(
        dt: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), &'static str> {
        // 符号なし48bitの範囲は0..2^48 - 1
        let timestamp = dt.timestamp_millis();
        if !(0..(1 << 48)).contains(&timestamp) {
            return Err("Timestamp must be between 0 and 2^48-1 milliseconds");
        }
        Ok(())
    }
}

impl serde::Serialize for VideoPublishedAt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_rfc3339())
    }
}

impl<'de> serde::Deserialize<'de> for VideoPublishedAt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = VideoPublishedAt;

            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                formatter.write_str("an ISO8601 datetime string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let dt = chrono::DateTime::parse_from_rfc3339(v)
                    .map_err(E::custom)?
                    .with_timezone(&chrono::Utc);
                VideoPublishedAt::new(dt).map_err(E::custom)
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_video_published_at_new_valid_timestamp() {
        let dt = Utc.timestamp_millis_opt(1_000_000).unwrap();
        let v = VideoPublishedAt::new(dt);
        assert!(v.is_ok());
        assert_eq!(v.unwrap().as_chrono_datetime(), &dt);
    }

    #[test]
    fn test_video_published_at_new_invalid_negative_timestamp() {
        let dt = Utc.timestamp_millis_opt(-1).unwrap();
        let v = VideoPublishedAt::new(dt);
        assert!(v.is_err());
    }

    #[test]
    fn test_video_published_at_new_invalid_too_large_timestamp() {
        let too_large = (1u64 << 48) as i64;
        let dt = Utc.timestamp_millis_opt(too_large).unwrap();
        let v = VideoPublishedAt::new(dt);
        assert!(v.is_err());
    }

    #[test]
    fn test_video_published_at_serialize_deserialize() {
        let dt = Utc.timestamp_millis_opt(1_234_567_890).unwrap();
        let v = VideoPublishedAt::new(dt).unwrap();
        let s = serde_json::to_string(&v).unwrap();
        let v2: VideoPublishedAt = serde_json::from_str(&s).unwrap();
        assert_eq!(v, v2);
    }

    #[test]
    fn test_video_published_at_try_add_valid() {
        let dt1 = Utc.timestamp_millis_opt(1_000_000).unwrap();
        let dt2 = Utc.timestamp_millis_opt(2_000_000).unwrap();
        let v1 = VideoPublishedAt::new(dt1).unwrap();
        let v2 = VideoPublishedAt::new(dt2).unwrap();
        let v3 = v1.try_add(&v2).unwrap();
        let expected = Utc.timestamp_millis_opt(1_000_000 + 2_000_000).unwrap();
        assert_eq!(v3.as_chrono_datetime(), &expected);
    }

    #[test]
    fn test_video_published_at_try_add_overflow() {
        let base = (1u64 << 48) as i64 - 500;
        let dt1 = Utc.timestamp_millis_opt(base).unwrap();
        let dt2 = Utc.timestamp_millis_opt(1000).unwrap();
        let v1 = VideoPublishedAt::new(dt1).unwrap();
        let v2 = VideoPublishedAt::new(dt2).unwrap();
        let result = v1.try_add(&v2);
        assert!(result.is_err());
    }
}
