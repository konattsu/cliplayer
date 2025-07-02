#[derive(Clone)]
pub struct YouTubeApiKey(String);

impl YouTubeApiKey {
    pub fn new(key: String) -> Self {
        Self(key)
    }

    pub fn as_str(&self) -> &str {
        &self.0
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
