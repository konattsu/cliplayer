/// 楽曲, もしくは動画に適用するタグ
///
/// タグは空文字列を許容しない
#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Tag(String);

// tagが空文字列でないことを検証するためのカスタムシリアライザ
impl<'de> serde::Deserialize<'de> for Tag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let tag = String::deserialize(deserializer)?;
        Tag::new(tag).map_err(serde::de::Error::custom)
    }
}

impl Tag {
    /// タグを新規作成する
    ///
    /// - Error: タグが空文字列のとき
    pub fn new(tag: String) -> Result<Self, &'static str> {
        if tag.is_empty() {
            Err("Tag cannot be an empty string")
        } else {
            Ok(Tag(tag))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
impl Tag {
    /// return `tag1`
    pub fn self_1() -> Self {
        Tag::new("tag1".to_string()).unwrap()
    }
    /// return `tag2`
    pub fn self_2() -> Self {
        Tag::new("tag2".to_string()).unwrap()
    }
    /// return `tag3`
    pub fn self_3() -> Self {
        Tag::new("tag3".to_string()).unwrap()
    }
}

/// 楽曲, もしくは動画に適用するタグのリスト
#[derive(
    serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq, Default,
)]
pub struct TagList(Vec<Tag>);

#[cfg(test)]
impl TagList {
    /// return `["tag1", "tag2"]`
    pub fn test_tag_list_1() -> Self {
        TagList(vec![Tag::self_1(), Tag::self_2()])
    }
    /// return `["tag2", "tag3"]`
    pub fn test_tag_list_2() -> Self {
        TagList(vec![Tag::self_2(), Tag::self_3()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_new() {
        assert!(Tag::new("tag1".to_string()).is_ok());
        assert!(Tag::new("".to_string()).is_err());
    }

    #[test]
    fn test_tag_test_tag() {
        assert_eq!(Tag::self_1().as_str(), "tag1");
        assert_eq!(Tag::self_2().as_str(), "tag2");
        assert_eq!(Tag::self_3().as_str(), "tag3");
    }
}
