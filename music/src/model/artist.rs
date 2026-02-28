/// アーティストとその周辺情報のhashmap
///
/// (artist_id, ArtistDefinition)
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub(crate) struct ArtistDefinitions(
    std::collections::HashMap<String, ArtistDefinition>,
);

/// アーティストとその周辺情報
#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct ArtistDefinition {
    /// 日本語での名前
    ja: String,
    /// 平仮名での名前
    jah: String,
    /// 英語での名前
    en: String,
    /// alias
    aliases: Vec<String>,
    /// チャンネルid
    channel_id: crate::model::ChannelId,
    /// カラー
    color: crate::model::Color,
    #[serde(skip_serializing_if = "is_false")]
    #[serde(default = "default_for_is_graduate")]
    /// 卒業したか
    is_graduated: bool,
}

fn is_false(val: &bool) -> bool {
    !*val
}
fn default_for_is_graduate() -> bool {
    false
}

impl ArtistDefinitions {
    fn len(&self) -> usize {
        self.0.len()
    }

    /// アーティストIDが定義されているかどうか
    fn is_defined_artist_id(&self, artist_id: &str) -> bool {
        self.0.contains_key(artist_id)
    }
}

// テストでは静的に値を定義
// rust-analyzerがこれ(本番用)をinactiveにするが無視していい
/// アーティストidとその周辺情報
///
/// - `ARTIST_SET_PATH` 環境変数で指定されたファイルから読み込む
///   - 先ほどの環境変数が指定されていないと `./data/artists_data.json` を読み込む
#[cfg(not(test))]
static LOADED_ARTISTS_DATA: once_cell::sync::Lazy<ArtistDefinitions> =
    once_cell::sync::Lazy::new(|| {
        const ARTIST_SET_PATH: &str = "./data/artists_data.json";

        let path_str = std::env::var("ARTIST_SET_PATH")
            .unwrap_or_else(|_| ARTIST_SET_PATH.to_string());
        let data = std::fs::read_to_string(path_str.clone()).unwrap_or_else(|e| {
            panic!(
                "Failed to read artists data from {path_str}. \
                This value is read from the env value, or default to {ARTIST_SET_PATH}. \
                reason: {e}"
            )
        });
        let data: ArtistDefinitions = serde_json::from_str(&data).unwrap();
        tracing::debug!("Loaded {} artists from {}", data.len(), path_str);
        data
    });

/// アーティストidとその周辺情報
#[cfg(test)]
static LOADED_ARTISTS_DATA: once_cell::sync::Lazy<ArtistDefinitions> =
    once_cell::sync::Lazy::new(|| {
        const ARTIST_DATA: &str = r#"
        {
            "aimer-test": {
                "ja": "エイマーテスト",
                "jah": "えいまーてすと",
                "en": "Aimer Test",
                "aliases": ["aim"],
                "channelId": "UC1111111111111111111111",
                "color": "111111"
            },
            "eir-aoi-test": {
                "ja": "エイラアオイテスト",
                "jah": "えいらあおいてすと",
                "en": "Eir Aoi Test",
                "aliases": [],
                "channelId": "UC2222222222222222222222",
                "color": "222222"
            },
            "lisa-test": {
                "ja": "リサテスト",
                "jah": "りさてすと",
                "en": "Lisa Test",
                "aliases": ["ls"],
                "channelId": "UC3333333333333333333333",
                "color": "333333",
                "isGraduated": true
            }
        }"#;
        let hash_set: ArtistDefinitions =
            serde_json::from_str(ARTIST_DATA).expect("will not fail");
        tracing::debug!("Loaded {} artists. (for tests)", hash_set.len());
        hash_set
    });

// MARK: Internal

/// 内部アーティスト
///
/// 事前に定義したアーティストIDのうちのどれかであることを保証
#[derive(Debug, serde::Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct InternalArtist(String);

/// 内部アーティストのリスト
///
/// 内部に `InternalArtist` のリストを保持
///
/// 以下を保証
/// - `artists` は空でないこと
/// - `artists` の要素は `InternalArtist` の順序でソートされていること
#[derive(Debug, serde::Serialize, Clone, PartialEq, Eq)]
pub(crate) struct InternalArtists(Vec<InternalArtist>);

impl InternalArtist {
    fn new<'a, T: Into<std::borrow::Cow<'a, str>>>(id: T) -> Result<Self, String> {
        let id = id.into();
        if !Self::is_valid_internal_artist(&id) {
            Err(format!("invalid internal artist: {id}"))
        } else {
            Ok(InternalArtist(id.into_owned()))
        }
    }

    /// 有効な内部アーティストIDかどうか
    fn is_valid_internal_artist(id: &str) -> bool {
        LOADED_ARTISTS_DATA.is_defined_artist_id(id)
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
    pub(crate) fn new(artists: Vec<&str>) -> Result<Self, String> {
        if artists.is_empty() {
            Err("artists list cannot be empty".to_string())
        } else {
            let mut internal_artists: Vec<InternalArtist> = artists
                .into_iter()
                .map(InternalArtist::new)
                .collect::<Result<Vec<_>, _>>()?;
            Self::sort_artists(&mut internal_artists);
            Ok(InternalArtists(internal_artists))
        }
    }

    /// アーティストの日本語名を取得
    pub(crate) fn get_artists_ja_name(&self) -> Vec<String> {
        let mut artists_name = Vec::new();
        for artist in &self.0 {
            if let Some(matched_artist) = LOADED_ARTISTS_DATA.0.get(&artist.0) {
                artists_name.push(matched_artist.ja.clone());
            } else {
                tracing::debug!("artist(id: {}) is not matched", artist.0);
            }
        }
        artists_name
    }

    pub(crate) fn to_vec(&self) -> Vec<&str> {
        self.0.iter().map(|artist| artist.0.as_str()).collect()
    }

    fn sort_artists(artists: &mut [InternalArtist]) {
        artists.sort();
    }
}

// artistsが空でないことを保証するため
// artistsをソートするため
impl<'de> serde::Deserialize<'de> for InternalArtists {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct RawInternalArtists(Vec<InternalArtist>);

        let mut raw = RawInternalArtists::deserialize(deserializer)?;
        if raw.0.is_empty() {
            Err(serde::de::Error::custom("artists list cannot be empty"))
        } else {
            Self::sort_artists(&mut raw.0);
            Ok(InternalArtists(raw.0))
        }
    }
}

// MARK: External

/// 外部アーティスト
///
/// 以下を保証
/// - 内部アーティストIDと重複しないこと
/// - 空文字列でないこと
#[derive(Debug, serde::Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ExternalArtist(String);

/// 外部アーティストのリスト
///
/// 内部に `ExternalArtist` のリストを保持
///
/// 以下を保証
/// - `artists` は空でないこと
/// - `artists` の要素は `ExternalArtist` の順序でソートされていること
#[derive(Debug, serde::Serialize, Clone, PartialEq, Eq)]
pub(crate) struct ExternalArtists(Vec<ExternalArtist>);

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
            Err(format!("invalid external artist id: {id}"))
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
    pub(crate) fn to_vec(&self) -> Vec<&str> {
        self.0.iter().map(|artist| artist.0.as_str()).collect()
    }

    /// 外部アーティストのリストをソート
    fn sort_artists(artists: &mut [ExternalArtist]) {
        artists.sort();
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

        let mut raw = RawExternalArtists::deserialize(deserializer)?;
        if raw.0.is_empty() {
            Err(serde::de::Error::custom("artists list cannot be empty"))
        } else {
            Self::sort_artists(&mut raw.0);
            Ok(ExternalArtists(raw.0))
        }
    }
}

// MARK: For Tests

#[cfg(test)]
impl InternalArtist {
    /// `aimer-test`
    pub(crate) fn self_1() -> Self {
        Self::new("aimer-test").unwrap()
    }
    /// `eir-aoi-test`
    pub(crate) fn self_2() -> Self {
        Self::new("eir-aoi-test").unwrap()
    }
    /// `lisa-test`
    pub(crate) fn self_3() -> Self {
        Self::new("lisa-test").unwrap()
    }
}

#[cfg(test)]
impl InternalArtists {
    fn new_for_test(mut artists: Vec<InternalArtist>) -> Result<Self, &'static str> {
        if artists.is_empty() {
            Err("artists list cannot be empty")
        } else {
            Self::sort_artists(&mut artists);
            Ok(InternalArtists(artists))
        }
    }

    /// Vec `aimer-test`
    pub(crate) fn self_1() -> Self {
        Self::new_for_test(vec![InternalArtist::self_1()]).unwrap()
    }
    /// Vec `eir-aoi-test`
    pub(crate) fn self_2() -> Self {
        Self::new_for_test(vec![InternalArtist::self_2()]).unwrap()
    }
    /// Vec `lisa-test`
    pub(crate) fn self_3() -> Self {
        Self::new_for_test(vec![InternalArtist::self_3()]).unwrap()
    }
    /// Vec `aimer-test`, `eir-aoi-test`, `lisa-test`
    pub(crate) fn self_4() -> Self {
        Self::new_for_test(vec![
            InternalArtist::self_1(),
            InternalArtist::self_2(),
            InternalArtist::self_3(),
        ])
        .unwrap()
    }
}

#[cfg(test)]
impl ExternalArtist {
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

#[cfg(test)]
impl ExternalArtists {
    fn new_for_test(mut artists: Vec<ExternalArtist>) -> Result<Self, &'static str> {
        if artists.is_empty() {
            Err("artists list cannot be empty")
        } else {
            Self::sort_artists(&mut artists);
            Ok(ExternalArtists(artists))
        }
    }

    /// Vec `Apple Mike`
    pub(crate) fn self_1() -> Self {
        Self::new_for_test(vec![ExternalArtist::self_1()]).unwrap()
    }
    /// Vec `Milk Mike`
    pub(crate) fn self_2() -> Self {
        Self::new_for_test(vec![ExternalArtist::self_2()]).unwrap()
    }
    /// Vec `Banana Mike`
    pub(crate) fn self_3() -> Self {
        Self::new_for_test(vec![ExternalArtist::self_3()]).unwrap()
    }
}

// MARK: Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_artist_function_for_test_works() {
        assert_eq!(InternalArtist::self_1().0, "aimer-test");
        assert_eq!(InternalArtist::self_2().0, "eir-aoi-test");
        assert_eq!(InternalArtist::self_3().0, "lisa-test");

        assert_eq!(ExternalArtist::self_1().0, "Apple Mike");
        assert_eq!(ExternalArtist::self_2().0, "Milk Mike");
        assert_eq!(ExternalArtist::self_3().0, "Banana Mike");
    }

    #[test]
    fn test_artist_new_valid() {
        assert!(InternalArtist::new("aimer-test").is_ok());
        assert!(InternalArtist::new("eir-aoi-test").is_ok());
        assert!(InternalArtist::new("lisa-test").is_ok());
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
        assert!(ExternalArtist::new("aimer-test").is_err());
        assert!(ExternalArtist::new("eir-aoi-test").is_err());
        assert!(ExternalArtist::new("lisa-test").is_err());
        assert!(ExternalArtist::new("").is_err());
    }

    #[test]
    fn test_internal_artist_deserialize_valid() {
        let json = r#""aimer-test""#;
        let artist: InternalArtist =
            serde_json::from_str(json).expect("Failed to deserialize internal artist");
        assert_eq!(artist.0, "aimer-test");
        let json = r#""eir-aoi-test""#;
        let artist: InternalArtist =
            serde_json::from_str(json).expect("Failed to deserialize internal artist");
        assert_eq!(artist.0, "eir-aoi-test");
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
        let json = r#""aimer-test""#;
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
        let json = r#"["eir-aoi-test", "lisa-test", "aimer-test"]"#;
        let artists: InternalArtists =
            serde_json::from_str(json).expect("Failed to deserialize internal artists");
        // ソートできているか
        assert_eq!(artists.0[0].0, "aimer-test");
        assert_eq!(artists.0[1].0, "eir-aoi-test");
        assert_eq!(artists.0[2].0, "lisa-test");
    }

    #[test]
    fn test_internal_artists_deserialize_invalid() {
        let json = r#"[]"#; // 空の配列は許容されない
        let result: Result<InternalArtists, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Expected error for empty artists list");
    }

    #[test]
    fn test_external_artists_deserialize_valid() {
        let json = r#"["External Artist 1", "External Artist 2", "Apple"]"#;
        let artists: ExternalArtists =
            serde_json::from_str(json).expect("Failed to deserialize external artists");
        // ソートできているか
        assert_eq!(artists.0[0].0, "Apple");
        assert_eq!(artists.0[1].0, "External Artist 1");
        assert_eq!(artists.0[2].0, "External Artist 2");
    }

    #[test]
    fn test_external_artists_deserialize_invalid() {
        let json = r#"[]"#; // 空の配列は許容されない
        let result: Result<ExternalArtists, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Expected error for empty artists list");
    }

    #[test]
    fn test_internal_artists_new_valid() {
        let artists = InternalArtists::new(vec!["aimer-test", "eir-aoi-test"])
            .expect("should create valid InternalArtists");
        assert_eq!(artists.0.len(), 2);
        assert_eq!(artists.0[0].0, "aimer-test");
        assert_eq!(artists.0[1].0, "eir-aoi-test");
    }

    #[test]
    fn test_internal_artists_new_invalid() {
        let result = InternalArtists::new(vec![]);
        assert!(result.is_err(), "Expected error for empty artists list");
        let result = InternalArtists::new(vec!["invalid-artist"]);
        assert!(result.is_err(), "Expected error for invalid artist ID");
        let result = InternalArtists::new(vec!["aimer-test", "invalid-artist"]);
        assert!(result.is_err(), "Expected error for invalid artist ID");
    }
}
