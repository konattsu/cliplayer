/// YouTubeのチャンネルid
///
/// - 大文字の`UC`で始まる
/// - 大文字の`UC`の直後に22文字の `a-z`, `A-Z`, `0-9`, `-`, `_` での構成 (計24文字)
#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ChannelId(String);

impl<'de> serde::Deserialize<'de> for ChannelId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = String::deserialize(deserializer)?;
        ChannelId::new(id).map_err(serde::de::Error::custom)
    }
}

impl ChannelId {
    pub(crate) fn new(id: String) -> Result<Self, &'static str> {
        if Self::is_valid_channel_id(&id) {
            Ok(ChannelId(id))
        } else {
            Err("Invalid channel ID format")
        }
    }

    /// チャンネルidが有効かどうか
    ///
    /// - 大文字の`UC`で始まる
    /// - 大文字の`UC`の直後に22文字の `a-z`, `A-Z`, `0-9`, `-`, `_` での構成
    fn is_valid_channel_id(id: &str) -> bool {
        const ID_CHARS: &str =
            "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_-";
        if !id.starts_with("UC") || id.len() != 24 {
            return false;
        }
        for c in id[2..].chars() {
            if !ID_CHARS.contains(c) {
                return false;
            }
        }
        true
    }
}

impl std::fmt::Display for ChannelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// MARK: For Tests

#[cfg(test)]
impl ChannelId {
    /// return `UC1111111111111111111111` (24chars)
    pub(crate) fn test_id_1() -> Self {
        ChannelId::new("UC1111111111111111111111".to_string()).unwrap()
    }
    /// return `UC2222222222222222222222` (24chars)
    pub(crate) fn test_id_2() -> Self {
        ChannelId::new("UC2222222222222222222222".to_string()).unwrap()
    }
    /// return `UC3333333333333333333333` (24chars)
    pub(crate) fn test_id_3() -> Self {
        ChannelId::new("UC3333333333333333333333".to_string()).unwrap()
    }
}

// MARK: Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_id_test_id() {
        assert_eq!(ChannelId::test_id_1().0, "UC1111111111111111111111");
        assert_eq!(ChannelId::test_id_2().0, "UC2222222222222222222222");
        assert_eq!(ChannelId::test_id_3().0, "UC3333333333333333333333");
    }

    #[test]
    fn test_channel_id_valid() {
        assert!(ChannelId::new("UC1111100000111110000011".to_string()).is_ok());
        assert!(ChannelId::new("UC7_MFM9b8hp5kuTSpa8WyOa".to_string()).is_ok());
    }

    #[test]
    fn test_channel_id_invalid_too_long() {
        assert!(ChannelId::new("UChgpoay395-8yuhlkjhj9a-8_g".to_string()).is_err());
    }

    #[test]
    fn test_channel_id_invalid_too_short() {
        assert!(ChannelId::new("UCabcdefghijklmno123457".to_string()).is_err());
        assert!(ChannelId::new("".to_string()).is_err());
        assert!(ChannelId::new("UC".to_string()).is_err());
    }

    #[test]
    fn test_channel_id_invalid_format() {
        // missing prefix
        assert!(ChannelId::new("Uabcdefghijklmno12345678".to_string()).is_err());
        // invalid characters
        assert!(ChannelId::new("UCabcdefghijklmno123456!".to_string()).is_err());
        assert!(ChannelId::new("UCabcdefghijklmno12345 4".to_string()).is_err());
    }
}
