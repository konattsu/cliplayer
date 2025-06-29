#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct VideoId(String);

impl VideoId {
    pub fn new(id: String) -> Result<Self, &'static str> {
        if Self::is_valid_video_id(&id) {
            Ok(VideoId(id))
        } else {
            Err("Invalid video ID format")
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// 動画idの検証を行う
    ///
    /// `a-z`, `A-Z`, `0-9`, `-`, `_` の11文字で構成されていること
    fn is_valid_video_id(id: &str) -> bool {
        static ID_CHARS: &str =
            "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_-";
        if id.len() != 11 {
            return false;
        }
        for c in id.chars() {
            if !ID_CHARS.contains(c) {
                return false;
            }
        }
        true
    }
}

impl std::fmt::Display for VideoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
impl VideoId {
    /// return `11111111111`
    pub fn test_id_1() -> Self {
        VideoId::new("11111111111".to_string()).unwrap()
    }
    /// return `22222222222`
    pub fn test_id_2() -> Self {
        VideoId::new("22222222222".to_string()).unwrap()
    }
    /// return `33333333333`
    pub fn test_id_3() -> Self {
        VideoId::new("33333333333".to_string()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_id_test_id() {
        assert_eq!(VideoId::test_id_1().0, "11111111111");
        assert_eq!(VideoId::test_id_2().0, "22222222222");
        assert_eq!(VideoId::test_id_3().0, "33333333333");
    }

    #[test]
    fn test_video_id_valid_lowercase_id() {
        assert!(VideoId::new("abcdefghijk".to_string()).is_ok());
    }

    #[test]
    fn test_video_id_valid_uppercase_id() {
        assert!(VideoId::new("ABCDEFGHIJK".to_string()).is_ok());
    }

    #[test]
    fn test_video_id_valid_symbols_id() {
        assert!(VideoId::new("__________-".to_string()).is_ok());
    }

    #[test]
    fn test_video_id_invalid_too_long() {
        assert!(VideoId::new("012345678901".to_string()).is_err());
    }

    #[test]
    fn test_video_id_invalid_too_short() {
        assert!(VideoId::new("0123456789".to_string()).is_err());
    }

    #[test]
    fn test_video_id_invalid_characters() {
        assert!(VideoId::new("0123456789*".to_string()).is_err());
        assert!(VideoId::new("012345678 *".to_string()).is_err());
    }
}
