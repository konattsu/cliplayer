/// ライバーIDのリスト
///
/// 内部に `LiverId` のリストを保持
///
/// 以下を保証
/// - `liver_ids` は空でないこと
/// - `liver_ids` の要素は `LiverId` の順序でソートされていること
/// - 重複がないこと
#[derive(Debug, serde::Serialize, Clone, PartialEq, Eq)]
pub struct LiverIds(Vec<super::LiverId>);

impl LiverIds {
    pub fn new(liver_ids: Vec<&str>) -> Result<Self, String> {
        let liver_ids = liver_ids
            .into_iter()
            .map(super::LiverId::new)
            .collect::<Result<Vec<_>, _>>()?;
        Self::from_liver_ids(liver_ids).map_err(str::to_string)
    }

    pub fn to_vec(&self) -> Vec<&str> {
        self.0.iter().map(|liver_id| liver_id.as_str()).collect()
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

    /// sortして重複を削除する
    fn sort_dedup_liver_ids(liver_ids: &mut Vec<super::LiverId>) {
        liver_ids.sort();
        liver_ids.dedup();
    }

    fn from_liver_ids(
        mut liver_ids: Vec<super::LiverId>,
    ) -> Result<Self, &'static str> {
        if liver_ids.is_empty() {
            Err("liver ids list cannot be empty")
        } else {
            Self::sort_dedup_liver_ids(&mut liver_ids);
            Ok(LiverIds(liver_ids))
        }
    }
}

// liver_ids が空でないことを保証するため
// liver_ids をソートして重複を削除するため
impl<'de> serde::Deserialize<'de> for LiverIds {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct RawLiverIds(Vec<super::LiverId>);

        let raw = RawLiverIds::deserialize(deserializer)?;
        Self::from_liver_ids(raw.0).map_err(serde::de::Error::custom)
    }
}

#[cfg(any(test, feature = "test-helpers"))]
impl LiverIds {
    fn new_for_test(liver_ids: Vec<super::LiverId>) -> Result<Self, &'static str> {
        Self::from_liver_ids(liver_ids)
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
    fn new_dedupes_duplicates() {
        let artists = LiverIds::new(vec!["yugamin", "riku-tazumi", "yugamin"])
            .expect("should create valid LiverIds");
        assert_eq!(artists.0.len(), 2);
        assert_eq!(artists.0[0].as_str(), "riku-tazumi");
        assert_eq!(artists.0[1].as_str(), "yugamin");
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
