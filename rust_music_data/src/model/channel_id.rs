/// YouTubeのチャンネルid
///
/// - 大文字の`UC`で始まる
/// - 大文字の`UC`の直後に24文字の `a-z`, `A-Z`, `0-9`, `-`, `_` での構成 (計26文字)
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChannelId(String);

impl ChannelId {
    pub fn new(id: String) -> Result<Self, &'static str> {
        if Self::is_valid_channel_id(&id) {
            Ok(ChannelId(id))
        } else {
            Err("Invalid channel ID format")
        }
    }

    /// チャンネルidが有効かどうか
    ///
    /// - 大文字の`UC`で始まる
    /// - 大文字の`UC`の直後に24文字の `a-z`, `A-Z`, `0-9`, `-`, `_` での構成
    fn is_valid_channel_id(id: &str) -> bool {
        static ID_CHARS: &str =
            "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_-";
        if !id.starts_with("UC") || id.len() != 26 {
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

#[cfg(test)]
impl ChannelId {
    /// return `UC111111111111111111111111`
    pub fn test_id_1() -> Self {
        ChannelId::new("UC111111111111111111111111".to_string()).unwrap()
    }
    /// return `UC222222222222222222222222`
    pub fn test_id_2() -> Self {
        ChannelId::new("UC222222222222222222222222".to_string()).unwrap()
    }
    /// return `UC333333333333333333333333`
    pub fn test_id_3() -> Self {
        ChannelId::new("UC333333333333333333333333".to_string()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_id_test_id() {
        assert_eq!(ChannelId::test_id_1().0, "UC111111111111111111111111");
        assert_eq!(ChannelId::test_id_2().0, "UC222222222222222222222222");
        assert_eq!(ChannelId::test_id_3().0, "UC333333333333333333333333");
    }

    #[test]
    fn test_channel_id_valid() {
        assert!(ChannelId::new("UC111110000011111000001111".to_string()).is_ok());
        assert!(ChannelId::new("UC5j49p8hhgbouhba-9y3bnpoa".to_string()).is_ok());
    }

    #[test]
    fn test_channel_id_invalid_too_long() {
        assert!(ChannelId::new("UChgpoay395-8yuhlkjhj9a-8_g".to_string()).is_err());
    }

    #[test]
    fn test_channel_id_invalid_too_short() {
        assert!(ChannelId::new("UCabcdefghijklmno1234567".to_string()).is_err());
    }

    #[test]
    fn test_channel_id_invalid_format() {
        // missing prefix
        assert!(ChannelId::new("Ucbcdefghijklmno12345678".to_string()).is_err());
        // invalid characters
        assert!(ChannelId::new("UCabcdefghijklmno1234567!".to_string()).is_err());
        assert!(ChannelId::new("UCabcdefghijklmno123456 4".to_string()).is_err());
    }
}
