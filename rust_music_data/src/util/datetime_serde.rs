// chrono::DateTime<chrono::Utc>に対してios8601形式のデシリアライズ, シリアライズを可能にする

pub fn serialize<S>(
    datetime: &chrono::DateTime<chrono::Utc>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    // ISO 8601形式 (例: "2023-12-10T12:00:00Z")
    serializer.serialize_str(&datetime.to_rfc3339())
}

pub fn deserialize<'de, D>(
    deserializer: D,
) -> Result<chrono::DateTime<chrono::Utc>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct Visitor;

    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = chrono::DateTime<chrono::Utc>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an ISO8601 datetime string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            chrono::DateTime::parse_from_rfc3339(v)
                .map_err(E::custom)
                .map(|dt| dt.with_timezone(&chrono::Utc))
        }
    }

    deserializer.deserialize_str(Visitor)
}

#[cfg(test)]
mod tests {
    #[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
    struct TestStruct {
        #[serde(with = "crate::util::datetime_serde")]
        datetime: chrono::DateTime<chrono::Utc>,
    }

    #[test]
    fn test_datetime_serde_serialize_deserialize() {
        let dt = chrono::DateTime::parse_from_rfc3339("2023-12-10T12:00:00Z")
            .unwrap()
            .with_timezone(&chrono::Utc);
        let s = TestStruct { datetime: dt };
        let json = serde_json::to_string(&s).unwrap();
        assert_eq!(json, "{\"datetime\":\"2023-12-10T12:00:00+00:00\"}");
        let de: TestStruct = serde_json::from_str(&json).unwrap();
        assert_eq!(de, s);
    }
}
