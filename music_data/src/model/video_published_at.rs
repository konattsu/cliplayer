/// 動画のアップロード時間
///
/// - ミリ秒以下は切り捨て
/// - タイムスタンプに直したとき, 符号なし48bitで表現できる範囲内であることを保証
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct VideoPublishedAt(chrono::DateTime<chrono::Utc>);

impl VideoPublishedAt {
    /// 動画のアップデート時間を生成
    ///
    /// ミリ秒以下は切り捨てられる
    ///
    /// - Error: `upload_at`が符号なし48ビットのタイムスタンプの範囲外の場合
    ///   - i.e. `0..2^48-1` millisの範囲外
    pub(crate) fn new(
        upload_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<Self, &'static str> {
        Self::validate_unsigned_48bit_timestamp(upload_at)?;
        Ok(VideoPublishedAt(Self::truncate_millis(upload_at)))
    }

    pub(crate) fn as_chrono_datetime(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.0
    }

    pub(crate) fn as_secs(&self) -> u64 {
        u64::try_from(self.0.timestamp())
            .expect("VideoPublishedAt::as_secs() is overflow")
    }

    pub(crate) fn get_year(&self) -> usize {
        use chrono::Datelike;
        self.0.year() as usize
    }

    pub(crate) fn get_month(&self) -> usize {
        use chrono::Datelike;
        self.0.month() as usize
    }

    /// 動画のアップロード時間を加算
    ///
    /// - Error: 加算した結果が符号なし48ビットのタイムスタンプの範囲外の場合
    ///   - i.e. `0..2^48-1` millisの範囲外
    pub(crate) fn try_add(
        &self,
        other: &VideoPublishedAt,
    ) -> Result<VideoPublishedAt, &'static str> {
        // 最大で,48bit + 48bit = 49bitなので`chrono::Duration`を一時的に使用する
        // `chrono::Duration`側では符号なしだと63bitまで扱えるので問題ない
        let new_upload_at =
            self.0 + chrono::Duration::milliseconds(other.0.timestamp_millis());
        // 最大が49bitなので, 再度48bitの範囲内であることを確認
        Self::validate_unsigned_48bit_timestamp(new_upload_at)?;
        Ok(VideoPublishedAt(new_upload_at))
    }

    /// 符号なし48ビットのタイムスタンプの範囲内であることを検証
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

    /// ミリ秒以下を切り捨て
    fn truncate_millis(
        dt: chrono::DateTime<chrono::Utc>,
    ) -> chrono::DateTime<chrono::Utc> {
        use chrono::TimeZone;
        let secs = dt.timestamp();
        chrono::Utc.timestamp_opt(secs, 0).unwrap()
    }
}

impl std::fmt::Display for VideoPublishedAt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
        )
    }
}

// Display側に委譲
impl serde::Serialize for VideoPublishedAt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
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

// MARK: For Tests

#[cfg(test)]
impl VideoPublishedAt {
    /// returns `2024-01-01T01:01:01Z`
    pub(crate) fn self_1() -> Self {
        use chrono::TimeZone;
        let dt = chrono::Utc.with_ymd_and_hms(2024, 1, 1, 1, 1, 1).unwrap();
        VideoPublishedAt::new(dt).unwrap()
    }
    /// returns `2024-02-02T02:02:02Z`
    pub(crate) fn self_2() -> Self {
        use chrono::TimeZone;
        let dt = chrono::Utc.with_ymd_and_hms(2024, 2, 2, 2, 2, 2).unwrap();
        VideoPublishedAt::new(dt).unwrap()
    }
    /// returns `2024-03-03T03:03:03Z`
    pub(crate) fn self_3() -> Self {
        use chrono::TimeZone;
        let dt = chrono::Utc.with_ymd_and_hms(2024, 3, 3, 3, 3, 3).unwrap();
        VideoPublishedAt::new(dt).unwrap()
    }
}

// MARK: Tests

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_video_published_at_test_cases() {
        use chrono::TimeZone;
        let cases = vec![
            (
                VideoPublishedAt::self_1(),
                Utc.with_ymd_and_hms(2024, 1, 1, 1, 1, 1).unwrap(),
            ),
            (
                VideoPublishedAt::self_2(),
                Utc.with_ymd_and_hms(2024, 2, 2, 2, 2, 2).unwrap(),
            ),
            (
                VideoPublishedAt::self_3(),
                Utc.with_ymd_and_hms(2024, 3, 3, 3, 3, 3).unwrap(),
            ),
        ];
        for (actual, expected) in cases {
            assert_eq!(actual.as_chrono_datetime(), &expected);
        }
    }

    #[test]
    fn test_video_published_at_boundary_values() {
        use chrono::TimeZone;
        // 0ミリ秒
        let dt0 = Utc.timestamp_millis_opt(0).unwrap();
        assert!(VideoPublishedAt::new(dt0).is_ok());

        // 2^48-1ミリ秒
        let max = (1u64 << 48) - 1;
        let dt_max = Utc.timestamp_millis_opt(max as i64).unwrap();
        assert!(VideoPublishedAt::new(dt_max).is_ok());

        // 2^48ミリ秒（範囲外）
        let over = (1u64 << 48) as i64;
        let dt_over = Utc.timestamp_millis_opt(over).unwrap();
        assert!(VideoPublishedAt::new(dt_over).is_err());
    }

    #[test]
    fn test_video_published_at_truncate_millis() {
        use chrono::TimeZone;
        // ミリ秒以下が切り捨てられることを確認
        let dt = Utc.with_ymd_and_hms(2024, 5, 5, 5, 5, 5).unwrap()
            + chrono::Duration::milliseconds(123);
        let v = VideoPublishedAt::new(dt).unwrap();
        let expected = Utc.with_ymd_and_hms(2024, 5, 5, 5, 5, 5).unwrap();
        assert_eq!(v.as_chrono_datetime(), &expected);
    }

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
