/// ライバー
///
/// 事前に定義したライバーIDのうちのどれかであることを保証
#[derive(Debug, serde::Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct LiverId(String);

impl LiverId {
    pub(crate) fn new<'a, T: Into<std::borrow::Cow<'a, str>>>(
        id: T,
    ) -> Result<Self, String> {
        let id = id.into();
        if !Self::is_valid_liver_id(&id) {
            Err(format!("invalid liver: {id}"))
        } else {
            Ok(LiverId(id.into_owned()))
        }
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }

    /// 有効な内部アーティストIDかどうか
    fn is_valid_liver_id(id: &str) -> bool {
        crate::model::LOADED_LIVER_DATA.is_contains_liver_id(id)
    }
}

// デシリアライズ時にも`Self`の存在条件を確認するためのカスタムデシリアライザ
impl<'de> serde::Deserialize<'de> for LiverId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id: String = serde::Deserialize::deserialize(deserializer)?;
        Self::new(id).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
impl LiverId {
    /// `riku-tazumi`
    pub(crate) fn self_1() -> Self {
        Self::new("riku-tazumi").unwrap()
    }
    /// `yugamin`
    pub(crate) fn self_2() -> Self {
        Self::new("yugamin").unwrap()
    }
    /// `yudorikku`
    pub(crate) fn self_3() -> Self {
        Self::new("yudorikku").unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_artist_function_for_test_works() {
        assert_eq!(LiverId::self_1().as_str(), "riku-tazumi");
        assert_eq!(LiverId::self_2().as_str(), "yugamin");
        assert_eq!(LiverId::self_3().as_str(), "yudorikku");
    }

    #[test]
    fn new_valid_ids_succeed() {
        assert!(LiverId::new("riku-tazumi").is_ok());
        assert!(LiverId::new("yugamin").is_ok());
        assert!(LiverId::new("yudorikku").is_ok());
    }

    #[test]
    fn new_invalid_ids_fail() {
        // because the test data don't contain this id(`Invalid Artist`)
        assert!(LiverId::new("Invalid Artist").is_err());
        assert!(LiverId::new("").is_err());
    }

    #[test]
    fn deserialize_valid() {
        let json = r#""riku-tazumi""#;
        let artist: LiverId = serde_json::from_str(json).expect("deserialize failed");
        assert_eq!(artist.0, "riku-tazumi");
    }

    #[test]
    fn deserialize_invalid() {
        let json = r#""Invalid Artist""#;
        let result: Result<LiverId, _> = serde_json::from_str(json);
        assert!(result.is_err());

        let json = r#""""#;
        let result: Result<LiverId, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
