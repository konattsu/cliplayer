/// チャンネル名を表す構造体
///
/// 箱外で行われた配信/動画の時に付与
///
/// - チャンネル名は空文字列を許容しない
#[derive(Debug, serde::Serialize, Clone, PartialEq)]
pub(crate) struct UploaderName(String);

impl<'de> serde::Deserialize<'de> for UploaderName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let name = String::deserialize(deserializer)?;
        UploaderName::new(name).map_err(serde::de::Error::custom)
    }
}

impl UploaderName {
    pub(crate) fn new(name: String) -> Result<Self, &'static str> {
        if name.is_empty() {
            Err("Uploader name cannot be empty")
        } else {
            Ok(Self(name))
        }
    }
}

// MARK: For Tests

#[cfg(test)]
impl UploaderName {
    /// returns `Test Channel 1`
    pub(crate) fn test_uploader_name_1() -> Self {
        Self("Test Channel 1".to_string())
    }
    /// returns `Test Channel 2`
    pub(crate) fn test_uploader_name_2() -> Self {
        Self("Test Channel 2".to_string())
    }
    /// returns `Test Channel 3`
    pub(crate) fn test_uploader_name_3() -> Self {
        Self("Test Channel 3".to_string())
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

// MARK: Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uploader_name_test() {
        let uploader_name = UploaderName::test_uploader_name_1();
        assert_eq!(uploader_name.as_str(), "Test Channel 1");

        let uploader_name = UploaderName::test_uploader_name_2();
        assert_eq!(uploader_name.as_str(), "Test Channel 2");

        let uploader_name = UploaderName::test_uploader_name_3();
        assert_eq!(uploader_name.as_str(), "Test Channel 3");
    }

    #[test]
    fn test_uploader_name() {
        let _name = UploaderName::new("Test Channel".to_string()).unwrap();
        let _name = UploaderName::new("Foo  ".to_string()).unwrap();

        let name = UploaderName::new("".to_string());
        assert!(name.is_err());
    }
}
