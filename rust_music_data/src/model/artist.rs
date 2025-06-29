/// 内部アーティスト
///
/// 事前に定義したアーティストIDのうちのどれかであることを保証
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq, Hash)]
pub struct InternalArtist(String);

// TODO アーティストのjsonファイルの構造を確認する

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

impl InternalArtist {
    pub fn new<'a, T: Into<std::borrow::Cow<'a, str>>>(id: T) -> Result<Self, String> {
        let id = id.into();
        if !Self::is_valid_internal_artist(&id) {
            Err(format!("invalid artist ID: {}", id))
        } else {
            Ok(InternalArtist(id.into_owned()))
        }
    }

    /// 有効な内部アーティストIDかどうか
    pub fn is_valid_internal_artist(id: &str) -> bool {
        ARTIST_SET.contains(id)
    }
}

impl std::str::FromStr for InternalArtist {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[cfg(test)]
impl InternalArtist {
    /// Aimer Test
    pub fn test_name1() -> Self {
        Self("Aimer Test".to_string())
    }
    /// Eir Aoi Test
    pub fn test_name2() -> Self {
        Self("Eir Aoi Test".to_string())
    }
    /// Lisa Test
    pub fn test_name3() -> Self {
        Self("Lisa Test".to_string())
    }
}

/// 外部アーティスト
///
/// 以下を保証
/// - 内部アーティストIDと重複しないこと
/// - 空文字列でないこと
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq, Hash)]
pub struct ExternalArtist(String);

impl ExternalArtist {
    /// 新しい外部アーティストを生成
    /// - 内部アーティストIDと重複しないこと
    /// - 空文字列でないこと
    pub fn new<'a, T: Into<std::borrow::Cow<'a, str>>>(id: T) -> Result<Self, String> {
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
    pub fn is_valid_external_artist(id: &str) -> bool {
        !InternalArtist::is_valid_internal_artist(id) && !id.is_empty()
    }
}

#[cfg(test)]
impl ExternalArtist {
    /// Apple Mike
    pub fn test_name1() -> Self {
        Self("Apple Mike".to_string())
    }
    /// Milk Mike
    pub fn test_name2() -> Self {
        Self("Milk Mike".to_string())
    }
    /// Banana Mike
    pub fn test_name3() -> Self {
        Self("Banana Mike".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
