/// YouTubeApiKey
///
/// - 空文字列でないことを保証
#[derive(Clone)]
pub struct YouTubeApiKey(String);

impl YouTubeApiKey {
    /// YouTubeApiKeyを作成
    ///
    /// - Error: 空文字列のとき
    pub fn new(key: &str) -> Result<Self, &'static str> {
        if key.is_empty() {
            Err("YouTube API key cannot be empty")
        } else {
            Ok(Self(key.to_string()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::str::FromStr for YouTubeApiKey {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl std::fmt::Display for YouTubeApiKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::fmt::Debug for YouTubeApiKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "YouTubeApiKey(****)")
    }
}

#[cfg(test)]
impl YouTubeApiKey {
    pub fn dummy_api_key() -> Self {
        Self("dummy_api_key".to_string())
    }
}
