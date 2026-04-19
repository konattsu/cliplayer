/// ライバーIDのリスト
///
/// 内部に `LiverId` のリストを保持
///
/// 以下を保証
/// - `artists` は空でないこと
/// - `artists` の要素は `LiverId` の順序でソートされていること
#[derive(Debug, serde::Serialize, Clone, PartialEq, Eq)]
pub struct LiverIds(Vec<super::LiverId>);

impl LiverIds {
    pub fn new(liver_ids: Vec<&str>) -> Result<Self, String> {
        if liver_ids.is_empty() {
            Err("liver ids list cannot be empty".to_string())
        } else {
            let mut liver_ids: Vec<super::LiverId> = liver_ids
                .into_iter()
                .map(super::LiverId::new)
                .collect::<Result<Vec<_>, _>>()?;
            Self::sort_artists(&mut liver_ids);
            Ok(LiverIds(liver_ids))
        }
    }

    pub fn to_vec(&self) -> Vec<&str> {
        self.0.iter().map(|artist| artist.as_str()).collect()
    }

    /// 各ライバーの日本語名のリストを返す
    ///
    /// - `LOADED_LIVER_DATA` に存在するアーティストのみ返す
    pub fn get_artists_ja_name(&self) -> Vec<String> {
        self.0
            .iter()
            .filter_map(|id| super::LOADED_LIVER_DATA.get_ja_name(id))
            .collect()
    }

    fn sort_artists(artists: &mut [super::LiverId]) {
        artists.sort();
    }
}

// artistsが空でないことを保証するため
// artistsをソートするため
impl<'de> serde::Deserialize<'de> for LiverIds {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct RawInternalArtists(Vec<super::LiverId>);

        let mut raw = RawInternalArtists::deserialize(deserializer)?;
        if raw.0.is_empty() {
            Err(serde::de::Error::custom("artists list cannot be empty"))
        } else {
            Self::sort_artists(&mut raw.0);
            Ok(LiverIds(raw.0))
        }
    }
}

#[cfg(any(test, feature = "test-helpers"))]
impl LiverIds {
    fn new_for_test(mut artists: Vec<super::LiverId>) -> Result<Self, &'static str> {
        if artists.is_empty() {
            Err("artists list cannot be empty")
        } else {
            Self::sort_artists(&mut artists);
            Ok(LiverIds(artists))
        }
    }

    pub fn self_1() -> Self {
        Self::new_for_test(vec![super::LiverId::self_1()]).unwrap()
    }
    pub fn self_2() -> Self {
        Self::new_for_test(vec![super::LiverId::self_2()]).unwrap()
    }
    pub fn self_3() -> Self {
        Self::new_for_test(vec![super::LiverId::self_3()]).unwrap()
    }
    pub fn self_4() -> Self {
        Self::new_for_test(vec![
            super::LiverId::self_1(),
            super::LiverId::self_2(),
            super::LiverId::self_3(),
        ])
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_valid() {
        let artists = LiverIds::new(vec!["riku-tazumi", "yugamin"])
            .expect("should create valid InternalArtists");
        assert_eq!(artists.0.len(), 2);
        assert_eq!(artists.0[0].as_str(), "riku-tazumi");
        assert_eq!(artists.0[1].as_str(), "yugamin");
    }

    #[test]
    fn new_invalid() {
        let result = LiverIds::new(vec![]);
        assert!(result.is_err(), "Expected error for empty artists list");
        let result = LiverIds::new(vec!["invalid-artist"]);
        assert!(result.is_err(), "Expected error for invalid artist ID");
        let result = LiverIds::new(vec!["yugamin", "invalid-artist"]);
        assert!(result.is_err(), "Expected error for invalid artist ID");
    }

    #[test]
    fn deserialize_valid() {
        let json = r#"["riku-tazumi", "yugamin", "yudorikku"]"#;
        let artists: LiverIds =
            serde_json::from_str(json).expect("Failed to deserialize internal artists");
        assert_eq!(artists.0[0].as_str(), "riku-tazumi");
        // because of sorting, "yudorikku" comes before "yugamin"
        assert_eq!(artists.0[1].as_str(), "yudorikku");
        assert_eq!(artists.0[2].as_str(), "yugamin");
    }

    #[test]
    fn deserialize_invalid() {
        let json = r#"[]"#;
        let result: Result<LiverIds, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Expected error for empty artists list");
    }
}
