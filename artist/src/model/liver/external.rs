/// 外部アーティストの名前
///
/// 以下を保証
/// - 内部アーティストIDと重複しないこと
/// - 空文字列でないこと
#[derive(Debug, serde::Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct ExternalArtistName(String);

/// 外部アーティストのリスト
///
/// 内部に `ExternalArtist` のリストを保持
///
/// 以下を保証
/// - `artists` は空でないこと
/// - `artists` の要素は `ExternalArtist` の順序でソートされていること
#[derive(Debug, serde::Serialize, Clone, PartialEq, Eq)]
pub struct ExternalArtistsName(Vec<ExternalArtistName>);

impl ExternalArtistName {
    /// 新しい外部アーティストを生成
    /// - 内部アーティストIDと重複しないこと
    /// - 空文字列でないこと
    fn new<'a, T: Into<std::borrow::Cow<'a, str>>>(id: T) -> Result<Self, String> {
        let id = id.into();
        // 内部アーティストIDと重複しない、かつ空文字列でないこと
        match Self::valid_external_artist(&id) {
            Err(e) => Err(e),
            Ok(_ok) => Ok(ExternalArtistName(id.into_owned())),
        }
    }

    /// 有効な外部アーティスト名かどうか
    /// - 内部アーティストIDと重複しないこと
    /// - 空文字列でないこと
    fn valid_external_artist(id: &str) -> Result<(), String> {
        if crate::model::LOADED_LIVER_DATA.is_contains_liver_id(id) {
            Err("external artist id cannot overlap with internal artist id".to_string())
        } else if id.is_empty() {
            Err("external artist id cannot be empty".to_string())
        } else {
            Ok(())
        }
    }
}

// デシリアライズ時にも`Self`の存在条件を確認するためのカスタムデシリアライザ
impl<'de> serde::Deserialize<'de> for ExternalArtistName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id: String = serde::Deserialize::deserialize(deserializer)?;
        Self::new(id).map_err(serde::de::Error::custom)
    }
}

impl ExternalArtistsName {
    pub fn to_vec(&self) -> Vec<&str> {
        self.0.iter().map(|artist| artist.0.as_str()).collect()
    }

    /// 外部アーティストのリストをソート
    fn sort_artists(artists: &mut [ExternalArtistName]) {
        artists.sort();
    }
}

// artistsが空でないことを保証するためのカスタムデシリアライザ
impl<'de> serde::Deserialize<'de> for ExternalArtistsName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct RawExternalArtists(Vec<ExternalArtistName>);

        let mut raw = RawExternalArtists::deserialize(deserializer)?;
        if raw.0.is_empty() {
            Err(serde::de::Error::custom("artists list cannot be empty"))
        } else {
            Self::sort_artists(&mut raw.0);
            Ok(ExternalArtistsName(raw.0))
        }
    }
}

// MARK: For Tests

#[cfg(any(test, feature = "test-helpers"))]
impl ExternalArtistName {
    /// `Apple Mike`
    pub(crate) fn self_1() -> Self {
        Self::new("Apple Mike").unwrap()
    }
    /// `Milk Mike`
    pub(crate) fn self_2() -> Self {
        Self::new("Milk Mike").unwrap()
    }
    /// `Banana Mike`
    pub(crate) fn self_3() -> Self {
        Self::new("Banana Mike").unwrap()
    }
}

#[cfg(any(test, feature = "test-helpers"))]
impl ExternalArtistsName {
    fn new_for_test(
        mut artists: Vec<ExternalArtistName>,
    ) -> Result<Self, &'static str> {
        if artists.is_empty() {
            Err("artists list cannot be empty")
        } else {
            Self::sort_artists(&mut artists);
            Ok(ExternalArtistsName(artists))
        }
    }

    pub fn self_1() -> Self {
        Self::new_for_test(vec![ExternalArtistName::self_1()]).unwrap()
    }
    pub fn self_2() -> Self {
        Self::new_for_test(vec![ExternalArtistName::self_2()]).unwrap()
    }
    pub fn self_3() -> Self {
        Self::new_for_test(vec![ExternalArtistName::self_3()]).unwrap()
    }
}

// MARK: Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid() {
        assert!(ExternalArtistName::new("External Artist").is_ok());
        assert!(ExternalArtistName::new("Another Artist").is_ok());
    }

    #[test]
    fn test_new_invalid() {
        // duplicate with `liver_id`
        assert!(ExternalArtistName::new("riku-tazumi").is_err());
        assert!(ExternalArtistName::new("yugamin").is_err());
        assert!(ExternalArtistName::new("yudorikku").is_err());
        assert!(ExternalArtistName::new("").is_err());
    }

    #[test]
    fn test_deserialize_valid() {
        let json = r#""External Artist""#;
        let artist: ExternalArtistName =
            serde_json::from_str(json).expect("deserialize");
        assert_eq!(artist.0, "External Artist");
    }

    #[test]
    fn test_deserialize_invalid() {
        // duplicate with `liver_id`
        let json = r#""riku-tazumi""#;
        let result: Result<ExternalArtistName, _> = serde_json::from_str(json);
        assert!(result.is_err());
        let json = r#""""#;
        let result: Result<ExternalArtistName, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_artists_deserialize_valid() {
        let json = r#"["External Artist 1", "External Artist 2", "Apple"]"#;
        let artists: ExternalArtistsName =
            serde_json::from_str(json).expect("deserialize");
        assert_eq!(artists.0[0].0, "Apple");
        assert_eq!(artists.0[1].0, "External Artist 1");
        assert_eq!(artists.0[2].0, "External Artist 2");
    }

    #[test]
    fn test_artists_deserialize_invalid() {
        let json = r#"[]"#;
        let result: Result<ExternalArtistsName, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
