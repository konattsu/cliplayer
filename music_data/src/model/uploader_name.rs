/// チャンネル名を表す構造体
///
/// 箱外で行われた配信/動画の時に付与
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub struct UploaderName(String);

impl UploaderName {
    pub fn new(name: String) -> Result<Self, &'static str> {
        if name.is_empty() {
            Err("Uploader name cannot be empty")
        } else {
            Ok(Self(name))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

#[cfg(test)]
impl UploaderName {
    /// returns `Test Channel 1`
    pub fn test_uploader_name_1() -> Self {
        Self("Test Channel 1".to_string())
    }
    /// returns `Test Channel 2`
    pub fn test_uploader_name_2() -> Self {
        Self("Test Channel 2".to_string())
    }
    /// returns `Test Channel 3`
    pub fn test_uploader_name_3() -> Self {
        Self("Test Channel 3".to_string())
    }
}

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
