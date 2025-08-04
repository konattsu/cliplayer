/// アーティストのid
///
/// `a-z`, `A-Z`, `_` のみ使用可
#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(in crate::artist) struct ArtistId(String);

const ALLOWED_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ-";

impl ArtistId {
    pub(in crate::artist) fn new(id: String) -> Result<Self, &'static str> {
        if id.is_empty() {
            return Err("ArtistId cannot be empty");
        } else if id.chars().any(|c| !ALLOWED_CHARS.contains(c)) {
            return Err("ArtistId can only contain letters and hyphens");
        }
        Ok(Self(id))
    }

    pub(in crate::artist) fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> serde::Deserialize<'de> for ArtistId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = String::deserialize(deserializer)?;
        ArtistId::new(id).map_err(serde::de::Error::custom)
    }
}
