// TODO アーティストのjsonファイルの構造を確認する, テスト用も本番用もパース処理を統一
// TODO 単品(`InternalArtist`や`ExternalArtist`)非公開でもいい

// テストでは静的に値を定義
// rust-analyzerがこれ(本番用)をinactiveにするが無視していい
/// 有効なアーティストIDのセット
///
/// - `ARTIST_SET_PATH` 環境変数で指定されたファイルから読み込む
///   - 先ほどの環境変数が指定されていないと `artist_set.json` を読み込む
#[cfg(not(test))]
static ARTIST_SET: once_cell::sync::Lazy<std::collections::HashSet<String>> =
    once_cell::sync::Lazy::new(|| {
        let path_str = std::env::var("ARTIST_SET_PATH")
            .unwrap_or_else(|_| "artist_set.json".to_string());
        let data = std::fs::read_to_string(path_str).unwrap();
        serde_json::from_str(&data).unwrap()
    });

#[cfg(test)]
static ARTIST_SET: once_cell::sync::Lazy<std::collections::HashSet<String>> =
    once_cell::sync::Lazy::new(|| {
        [
            "Aimer Test".to_string(),
            "Eir Aoi Test".to_string(),
            "Lisa Test".to_string(),
        ]
        .into()
    });

// MARK: Internal

/// 内部アーティスト
///
/// 事前に定義したアーティストIDのうちのどれかであることを保証
#[derive(Debug, serde::Serialize, Clone, PartialEq, Eq)]
pub struct InternalArtist(String);

/// 内部アーティストのリスト
///
/// `artists` フィールドに `InternalArtist` のリストを保持
///
/// - `artists` は空でないことを保証
#[derive(Debug, serde::Serialize, Clone, PartialEq, Eq)]
pub struct InternalArtists(Vec<InternalArtist>);

impl InternalArtist {
    fn new<'a, T: Into<std::borrow::Cow<'a, str>>>(id: T) -> Result<Self, String> {
        let id = id.into();
        if !Self::is_valid_internal_artist(&id) {
            Err(format!("invalid artist : {}", id))
        } else {
            Ok(InternalArtist(id.into_owned()))
        }
    }

    /// 有効な内部アーティストIDかどうか
    fn is_valid_internal_artist(id: &str) -> bool {
        ARTIST_SET.contains(id)
    }
}

// デシリアライズ時にも`Self`の存在条件を確認するためのカスタムデシリアライザ
impl<'de> serde::Deserialize<'de> for InternalArtist {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id: String = serde::Deserialize::deserialize(deserializer)?;
        Self::new(id).map_err(serde::de::Error::custom)
    }
}

impl InternalArtists {
    fn new(artists: Vec<InternalArtist>) -> Result<Self, &'static str> {
        if artists.is_empty() {
            Err("artists list cannot be empty")
        } else {
            Ok(InternalArtists(artists))
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

// artistsが空でないことを保証するためのカスタムデシリアライザ
impl<'de> serde::Deserialize<'de> for InternalArtists {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct RawInternalArtists(Vec<InternalArtist>);

        let raw = RawInternalArtists::deserialize(deserializer)?;
        if raw.0.is_empty() {
            Err(serde::de::Error::custom("artists list cannot be empty"))
        } else {
            Ok(InternalArtists(raw.0))
        }
    }
}

#[cfg(test)]
impl InternalArtist {
    /// `Aimer Test`
    pub fn test_name1() -> Self {
        Self::new("Aimer Test").unwrap()
    }
    /// `Eir Aoi Test`
    pub fn test_name2() -> Self {
        Self::new("Eir Aoi Test").unwrap()
    }
    /// `Lisa Test`
    pub fn test_name3() -> Self {
        Self::new("Lisa Test").unwrap()
    }
}

#[cfg(test)]
impl InternalArtists {
    /// Vec `Aimer Test`
    pub fn test_name_1() -> Self {
        Self::new(vec![InternalArtist::test_name1()]).unwrap()
    }
    /// Vec `Eir Aoi Test`
    pub fn test_name_2() -> Self {
        Self::new(vec![InternalArtist::test_name2()]).unwrap()
    }
    /// Vec `Lisa Test`
    pub fn test_name_3() -> Self {
        Self::new(vec![InternalArtist::test_name3()]).unwrap()
    }
}

// MARK: External

/// 外部アーティスト
///
/// 以下を保証
/// - 内部アーティストIDと重複しないこと
/// - 空文字列でないこと
#[derive(Debug, serde::Serialize, Clone, PartialEq, Eq)]
pub struct ExternalArtist(String);

#[derive(Debug, serde::Serialize, Clone, PartialEq, Eq)]
pub struct ExternalArtists(Vec<ExternalArtist>);

impl ExternalArtist {
    /// 新しい外部アーティストを生成
    /// - 内部アーティストIDと重複しないこと
    /// - 空文字列でないこと
    fn new<'a, T: Into<std::borrow::Cow<'a, str>>>(id: T) -> Result<Self, String> {
        let id = id.into();
        // 内部アーティストIDと重複しない、かつ空文字列でないこと
        if Self::is_valid_external_artist(&id) {
            Ok(ExternalArtist(id.into_owned()))
        } else {
            Err(format!("invalid external artist ID: {}", id))
        }
    }

    /// 有効な外部アーティストIDかどうか
    /// - 内部アーティストIDと重複しないこと
    /// - 空文字列でないこと
    fn is_valid_external_artist(id: &str) -> bool {
        !InternalArtist::is_valid_internal_artist(id) && !id.is_empty()
    }
}

// デシリアライズ時にも`Self`の存在条件を確認するためのカスタムデシリアライザ
impl<'de> serde::Deserialize<'de> for ExternalArtist {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id: String = serde::Deserialize::deserialize(deserializer)?;
        Self::new(id).map_err(serde::de::Error::custom)
    }
}

impl ExternalArtists {
    fn new(artists: Vec<ExternalArtist>) -> Result<Self, &'static str> {
        if artists.is_empty() {
            Err("artists list cannot be empty")
        } else {
            Ok(ExternalArtists(artists))
        }
    }
}

// artistsが空でないことを保証するためのカスタムデシリアライザ
impl<'de> serde::Deserialize<'de> for ExternalArtists {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct RawExternalArtists(Vec<ExternalArtist>);

        let raw = RawExternalArtists::deserialize(deserializer)?;
        if raw.0.is_empty() {
            Err(serde::de::Error::custom("artists list cannot be empty"))
        } else {
            Ok(ExternalArtists(raw.0))
        }
    }
}

#[cfg(test)]
impl ExternalArtist {
    /// `Apple Mike`
    pub fn test_name1() -> Self {
        Self::new("Apple Mike").unwrap()
    }
    /// `Milk Mike`
    pub fn test_name2() -> Self {
        Self::new("Milk Mike").unwrap()
    }
    /// `Banana Mike`
    pub fn test_name3() -> Self {
        Self::new("Banana Mike").unwrap()
    }
}

#[cfg(test)]
impl ExternalArtists {
    /// Vec `Apple Mike`
    pub fn test_name_1() -> Self {
        Self::new(vec![ExternalArtist::test_name1()]).unwrap()
    }
    /// Vec `Milk Mike`
    pub fn test_name_2() -> Self {
        Self::new(vec![ExternalArtist::test_name2()]).unwrap()
    }
    /// Vec `Banana Mike`
    pub fn test_name_3() -> Self {
        Self::new(vec![ExternalArtist::test_name3()]).unwrap()
    }
}

// MARK: Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_artist_function_for_test_works() {
        assert_eq!(InternalArtist::test_name1().0, "Aimer Test");
        assert_eq!(InternalArtist::test_name2().0, "Eir Aoi Test");
        assert_eq!(InternalArtist::test_name3().0, "Lisa Test");

        assert_eq!(ExternalArtist::test_name1().0, "Apple Mike");
        assert_eq!(ExternalArtist::test_name2().0, "Milk Mike");
        assert_eq!(ExternalArtist::test_name3().0, "Banana Mike");
    }

    #[test]
    fn test_artist_new_valid() {
        assert!(InternalArtist::new("Aimer Test").is_ok());
        assert!(InternalArtist::new("Eir Aoi Test").is_ok());
        assert!(InternalArtist::new("Lisa Test").is_ok());
    }

    #[test]
    fn test_artist_new_invalid() {
        assert!(InternalArtist::new("Invalid Artist").is_err());
        assert!(InternalArtist::new("").is_err());
    }

    #[test]
    fn test_external_artist_new_valid() {
        assert!(ExternalArtist::new("External Artist").is_ok());
        assert!(ExternalArtist::new("Another Artist").is_ok());
    }

    #[test]
    fn test_external_artist_new_invalid() {
        assert!(ExternalArtist::new("Aimer Test").is_err());
        assert!(ExternalArtist::new("Eir Aoi Test").is_err());
        assert!(ExternalArtist::new("Lisa Test").is_err());
        assert!(ExternalArtist::new("").is_err());
    }

    #[test]
    fn test_internal_artist_deserialize_valid() {
        let json = r#""Aimer Test""#;
        let artist: InternalArtist =
            serde_json::from_str(json).expect("Failed to deserialize internal artist");
        assert_eq!(artist.0, "Aimer Test");
        let json = r#""Eir Aoi Test""#;
        let artist: InternalArtist =
            serde_json::from_str(json).expect("Failed to deserialize internal artist");
        assert_eq!(artist.0, "Eir Aoi Test");
    }

    #[test]
    fn test_internal_artist_deserialize_invalid() {
        // 無効なアーティストID
        let json = r#""Invalid Artist""#;
        let result: Result<InternalArtist, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Expected error for invalid internal artist"
        );
        // 空文字列
        let json = r#""""#;
        let result: Result<InternalArtist, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Expected error for empty internal artist");
    }

    #[test]
    fn test_external_artist_deserialize_valid() {
        let json = r#""External Artist""#;
        let artist: ExternalArtist =
            serde_json::from_str(json).expect("Failed to deserialize external artist");
        assert_eq!(artist.0, "External Artist");

        let json = r#""Another Artist""#;
        let artist: ExternalArtist =
            serde_json::from_str(json).expect("Failed to deserialize external artist");
        assert_eq!(artist.0, "Another Artist");
    }

    #[test]
    fn test_external_artist_deserialize_invalid() {
        // 内部アーティストIDと重複する
        let json = r#""Aimer Test""#;
        let result: Result<ExternalArtist, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Expected error for external artist with internal ID"
        );
        // 空文字列
        let json = r#""""#;
        let result: Result<ExternalArtist, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Expected error for empty external artist");
    }

    #[test]
    fn test_internal_artists_deserialize_valid() {
        let json = r#"["Aimer Test", "Eir Aoi Test", "Lisa Test"]"#;
        let _artists: InternalArtists =
            serde_json::from_str(json).expect("Failed to deserialize internal artists");
    }

    #[test]
    fn test_internal_artists_deserialize_invalid() {
        let json = r#"[]"#; // 空の配列は許容されない
        let result: Result<InternalArtists, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Expected error for empty artists list");
    }

    #[test]
    fn test_external_artists_deserialize_valid() {
        let json = r#"["External Artist 1", "External Artist 2"]"#;
        let _artists: ExternalArtists =
            serde_json::from_str(json).expect("Failed to deserialize external artists");
    }

    #[test]
    fn test_external_artists_deserialize_invalid() {
        let json = r#"[]"#; // 空の配列は許容されない
        let result: Result<ExternalArtists, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Expected error for empty artists list");
    }
}
