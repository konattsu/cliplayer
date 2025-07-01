/// チャンネル名を表す構造体
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct ChannelName(String);

impl ChannelName {
    pub fn new(name: String) -> Self {
        Self(name)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

#[cfg(test)]
impl ChannelName {
    /// returns `Test Channel 1`
    pub fn test_channel_name_1() -> Self {
        Self("Test Channel 1".to_string())
    }
    /// returns `Test Channel 2`
    pub fn test_channel_name_2() -> Self {
        Self("Test Channel 2".to_string())
    }
    /// returns `Test Channel 3`
    pub fn test_channel_name_3() -> Self {
        Self("Test Channel 3".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_name_test() {
        let channel_name = ChannelName::test_channel_name_1();
        assert_eq!(channel_name.as_str(), "Test Channel 1");

        let channel_name = ChannelName::test_channel_name_2();
        assert_eq!(channel_name.as_str(), "Test Channel 2");

        let channel_name = ChannelName::test_channel_name_3();
        assert_eq!(channel_name.as_str(), "Test Channel 3");
    }
}
