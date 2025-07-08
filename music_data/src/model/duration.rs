/// ISO 8601 Duration型
///
/// - 0..24時間まで
/// - 秒未満を保持しない
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Duration {
    inner: chrono::Duration,
}

impl Duration {
    pub fn from_chrono_duration(
        duration: chrono::Duration,
    ) -> Result<Duration, &'static str> {
        Self::validate_within_24_hours(&duration)?;
        let chrono_duration = Self::truncate_millis(duration);
        Ok(Duration {
            inner: chrono_duration,
        })
    }

    pub fn from_secs(secs: u16) -> Self {
        // - 24 hours = 86400 seconds
        // - u16 max: 65535
        // 絶対に0..24時間の範囲内の値になる
        Self::from_chrono_duration(chrono::Duration::seconds(secs as i64))
            .expect("Duration.from_secs() should not fail")
    }

    pub fn as_chrono_duration(&self) -> &chrono::Duration {
        &self.inner
    }

    pub fn as_secs(&self) -> u32 {
        u32::try_from(self.inner.num_seconds()).expect("Duration.as_secs() overflowed")
    }

    pub fn as_chrono_time(&self) -> chrono::NaiveTime {
        chrono::NaiveTime::from_num_seconds_from_midnight_opt(
            self.inner.num_seconds() as u32,
            0,
        )
        // 内部の値は24時間を超えないのでunwrap
        .expect("Duration exceeds 24 hours")
    }

    pub fn try_add(&self, other: &Duration) -> Result<Duration, &'static str> {
        let new_duration = self.inner + other.inner;
        Self::validate_within_24_hours(&new_duration)?;
        Ok(Self {
            inner: Self::truncate_millis(new_duration),
        })
    }

    fn validate_within_24_hours(
        duration: &chrono::Duration,
    ) -> Result<(), &'static str> {
        // 24 hours = 86,400 seconds
        if !(0..86400).contains(&duration.num_seconds()) {
            return Err("Duration must be between 0 and 24 hours");
        }
        Ok(())
    }

    /// ミリ秒以下を切り捨てる
    fn truncate_millis(duration: chrono::Duration) -> chrono::Duration {
        chrono::Duration::seconds(duration.num_seconds())
    }
}

impl std::str::FromStr for Duration {
    type Err = &'static str;

    /// ISO 8601形式の文字列からDurationを生成する
    ///
    /// - `^PT(?:(\\d+H)?(\\d+M)?(\\d+S)?)$` のみ
    ///   - `PT`などのアルファベットはcase-sensitive, uppercaseのみ
    /// - `各値(\d+)`の有効範囲`0..2^16`
    ///   - `Duration`の最大値は24時間も適用されることに注意
    ///
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 敢えてregex使わない
        // 文字列の長さは最大で32文字 `PT65535H65535M65535S`
        if !s.starts_with("PT") {
            return Err("Duration must start with 'PT'");
        } else if s.len() > 32 {
            return Err("Duration string is too long");
        }

        let mut rest = &s[2..];
        let mut hours: u16 = 0;
        let mut mins: u16 = 0;
        let mut secs: u16 = 0;

        while !rest.is_empty() {
            if let Some(pos) = rest.find('H') {
                hours = rest[..pos].parse().map_err(|_| "Invalid hours")?;
                rest = &rest[pos + 1..];
            } else if let Some(pos) = rest.find('M') {
                mins = rest[..pos].parse().map_err(|_| "Invalid minutes")?;
                rest = &rest[pos + 1..];
            } else if let Some(pos) = rest.find('S') {
                secs = rest[..pos].parse().map_err(|_| "Invalid seconds")?;
                rest = &rest[pos + 1..];
            } else {
                return Err("Invalid duration format");
            }
        }

        // 出力のi64ではoverflowは起きない (∵ 2^16 * 3600 < 2^(64-1))
        // 計算時にu16だとオーバフローが起こるので事前にキャスト変換しておく
        #[allow(clippy::unnecessary_cast)]
        let total_secs = ((hours as i64 * 3600) as i64)
            + ((mins as i64 * 60) as i64)
            + (secs as i64);

        let chrono_duration = chrono::Duration::seconds(total_secs);
        Self::from_chrono_duration(chrono_duration)
    }
}

/// DurationをISO 8601形式の文字列に変換する
///
/// 0h, 0m, 0sのフィールドがある場合は省略される
impl std::fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let total_secs = self.inner.num_seconds();
        let hours = total_secs / 3600;
        let mins = (total_secs % 3600) / 60;
        let secs = total_secs % 60;

        let duration_without_prefix = if hours == 0 && mins == 0 {
            format!("{}S", secs)
        } else if hours == 0 && secs == 0 {
            format!("{}M", mins)
        } else if mins == 0 && secs == 0 {
            format!("{}H", hours)
        } else if hours == 0 {
            format!("{}M{}S", mins, secs)
        } else if mins == 0 {
            format!("{}H{}S", hours, secs)
        } else if secs == 0 {
            format!("{}H{}M", hours, mins)
        } else {
            format!("{}H{}M{}S", hours, mins, secs)
        };

        write!(f, "PT{}", duration_without_prefix)
    }
}

impl serde::Serialize for Duration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = self.to_string();
        serializer.serialize_str(&s)
    }
}

// FromStrと一緒なのでそっちに委譲
impl<'de> serde::Deserialize<'de> for Duration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct DurationVisitor;

        impl<'de> serde::de::Visitor<'de> for DurationVisitor {
            type Value = Duration;

            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                formatter
                    .write_str("a duration string in ISO 8601 format (e.g., PT1H2M3S)")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::str::FromStr;
                Duration::from_str(v).map_err(|e| E::custom(e))
            }
        }

        deserializer.deserialize_any(DurationVisitor)
    }
}

#[cfg(test)]
impl Duration {
    /// returns `PT10S`
    pub fn self_1() -> Self {
        use std::str::FromStr;
        Self::from_str("PT10S").unwrap()
    }
    /// returns `PT20M`
    pub fn self_2() -> Self {
        use std::str::FromStr;
        Self::from_str("PT20M").unwrap()
    }
    /// returns `PT1H`
    pub fn self_3() -> Self {
        use std::str::FromStr;
        Self::from_str("PT1H").unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_duration_test() {
        let duration_1 = Duration::self_1();
        let duration_2 = Duration::self_2();
        let duration_3 = Duration::self_3();

        assert_eq!(duration_1.to_string(), "PT10S");
        assert_eq!(duration_2.to_string(), "PT20M");
        assert_eq!(duration_3.to_string(), "PT1H");
    }

    #[test]
    fn test_duration_from_str_valid() {
        let cases: Vec<(&str, chrono::Duration)> = vec![
            ("PT1H2M3S", chrono::Duration::seconds(3723)),
            ("PT1H2M", chrono::Duration::seconds(3720)),
            ("PT1M2S", chrono::Duration::seconds(62)),
            ("PT10H20S", chrono::Duration::seconds(36020)),
            ("PT1H", chrono::Duration::seconds(3600)),
            ("PT120M", chrono::Duration::seconds(7200)),
            ("PT2S", chrono::Duration::seconds(2)),
            ("PT0S", chrono::Duration::seconds(0)),
            ("PT0H0M0S", chrono::Duration::seconds(0)),
            ("PT23H59M59S", chrono::Duration::seconds(86399)),
        ];

        for (input, expected) in cases {
            let duration = Duration::from_str(input).unwrap();
            assert_eq!(duration.inner, expected);
        }
    }

    #[test]
    fn test_duration_from_str_invalid_char() {
        let result = Duration::from_str("PT1H2M3s");
        assert!(result.is_err());
    }
    #[test]
    fn test_duration_from_str_invalid_prefix() {
        let result = Duration::from_str("1H2M3S");
        assert!(result.is_err());
    }
    #[test]
    fn test_duration_from_str_invalid_range_too_large_s() {
        // 2^16 = 65536 < 65537
        let result = Duration::from_str("PT65537S");
        assert!(result.is_err());
    }
    #[test]
    fn test_duration_from_str_invalid_suffix() {
        let result = Duration::from_str("PT1H2M3");
        assert!(result.is_err());
    }

    /// 入力値が24時間を超えないように注意, 超えるとpanic
    fn duration_from_hms(hours: u16, mins: u16, secs: u16) -> Duration {
        Duration::from_chrono_duration(chrono::Duration::seconds(
            (hours as i64 * 3600) + (mins as i64 * 60) + (secs as i64),
        ))
        .unwrap()
    }

    #[test]
    fn test_duration_display_valid() {
        let cases: Vec<(Duration, &str)> = vec![
            (duration_from_hms(23, 2, 3), "PT23H2M3S"),
            (duration_from_hms(1, 2, 0), "PT1H2M"),
            (duration_from_hms(1, 0, 3), "PT1H3S"),
            (duration_from_hms(0, 2, 3), "PT2M3S"),
            (duration_from_hms(1, 0, 0), "PT1H"),
            (duration_from_hms(0, 2, 0), "PT2M"),
            (duration_from_hms(0, 0, 3), "PT3S"),
        ];

        for (duration, expected) in cases {
            assert_eq!(duration.to_string(), expected);
        }
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    struct StructForDeserializeSerialize {
        duration: Duration,
    }

    #[test]
    fn test_duration_serialize() {
        let duration = Duration::from_str("PT1H2M3S").unwrap();
        let serialized =
            serde_json::to_string(&StructForDeserializeSerialize { duration }).unwrap();
        assert_eq!(serialized, r#"{"duration":"PT1H2M3S"}"#);
    }
    #[test]
    fn test_duration_deserialize() {
        let json = r#"{"duration":"PT1H2M3S"}"#;
        let deserialized: StructForDeserializeSerialize =
            serde_json::from_str(json).unwrap();
        assert_eq!(
            deserialized.duration,
            Duration::from_str("PT1H2M3S").unwrap()
        );
    }
}
