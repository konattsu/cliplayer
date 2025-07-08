// NOTE VideoTags, ClipTagsは存在できる制約は同じだが混同しないように型を分離

/// 動画に適用するタグ
///
/// タグは空文字列を許容しない
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq)]
pub struct VideoTags(Tags);

impl VideoTags {
    /// 動画タグを新規作成する
    ///
    /// - Error: タグが空文字列のとき
    /// - Ok: 空文字列でないとき, ベクタが空の時も許容
    pub fn new(tags: Vec<String>) -> Result<Self, &'static str> {
        Tags::new(tags).map(VideoTags)
    }
}

/// クリップに適用するタグ
///
/// タグは空文字列を許容しない
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ClipTags(Tags);

impl ClipTags {
    /// クリップタグを新規作成する
    ///
    /// - Error: タグが空文字列のとき
    /// - Ok: 空文字列でないとき, ベクタが空の時も許容
    pub fn new(tags: Vec<String>) -> Result<Self, &'static str> {
        Tags::new(tags).map(ClipTags)
    }
}

/// タグたち
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq)]
struct Tags(Vec<Tag>);

impl Tags {
    /// - Error: タグが空文字列のとき
    /// - Ok: 空文字列でないとき, ベクタが空の時も許容
    pub fn new(tags: Vec<String>) -> Result<Self, &'static str> {
        let mut tags = tags
            .into_iter()
            .map(|tag| Tag::new(tag))
            .collect::<Result<Vec<Tag>, &'static str>>()?;
        tags.sort();
        Ok(Self(tags))
    }
}

/// タグ
#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Tag(String);

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
    fn new(tag: String) -> Result<Self, &'static str> {
        if tag.is_empty() {
            Err("Tag cannot be an empty string")
        } else {
            Ok(Tag(tag))
        }
    }
}

#[cfg(test)]
impl VideoTags {
    /// returns `Test Video Tag1`
    pub fn self_1() -> Self {
        VideoTags(Tags(vec![Tag::new("Test Video Tag1".to_string()).unwrap()]))
    }
    /// returns `Test Video Tag2`
    pub fn self_2() -> Self {
        VideoTags(Tags(vec![Tag::new("Test Video Tag2".to_string()).unwrap()]))
    }
    /// returns `Test Video Tag3`, `Test Video Tag4`
    pub fn self_3() -> Self {
        VideoTags(Tags(vec![
            Tag::new("Test Video Tag3".to_string()).unwrap(),
            Tag::new("Test Video Tag4".to_string()).unwrap(),
        ]))
    }
}

#[cfg(test)]
impl ClipTags {
    /// returns `Test Clip Tag1`
    pub fn self_1() -> Self {
        ClipTags(Tags(vec![Tag::new("Test Clip Tag1".to_string()).unwrap()]))
    }
    /// returns `Test Clip Tag2`
    pub fn self_2() -> Self {
        ClipTags(Tags(vec![Tag::new("Test Clip Tag2".to_string()).unwrap()]))
    }
    /// returns `Test Clip Tag3`, `Test Clip Tag4`
    pub fn self_3() -> Self {
        ClipTags(Tags(vec![
            Tag::new("Test Clip Tag3".to_string()).unwrap(),
            Tag::new("Test Clip Tag4".to_string()).unwrap(),
        ]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tags() -> Vec<String> {
        vec![
            "Tag4".to_string(),
            "Tag1".to_string(),
            "Apple".to_string(),
            "Tag3".to_string(),
            "Tag2".to_string(),
        ]
    }

    #[test]
    fn test_video_tags_new() {
        let video_tags = VideoTags::new(tags()).unwrap();
        assert_eq!(video_tags.0.0[0].0, "Apple");
        assert_eq!(video_tags.0.0[1].0, "Tag1");
        assert_eq!(video_tags.0.0[2].0, "Tag2");
        assert_eq!(video_tags.0.0[3].0, "Tag3");
        assert_eq!(video_tags.0.0[4].0, "Tag4");
    }

    #[test]
    fn test_clip_tags_new() {
        let video_tags = VideoTags::new(tags()).unwrap();
        assert_eq!(video_tags.0.0[0].0, "Apple");
        assert_eq!(video_tags.0.0[1].0, "Tag1");
        assert_eq!(video_tags.0.0[2].0, "Tag2");
        assert_eq!(video_tags.0.0[3].0, "Tag3");
        assert_eq!(video_tags.0.0[4].0, "Tag4");
    }

    #[test]
    fn test_tags_new() {
        let video_tags = VideoTags::new(tags()).unwrap();
        assert_eq!(video_tags.0.0[0].0, "Apple");
        assert_eq!(video_tags.0.0[1].0, "Tag1");
        assert_eq!(video_tags.0.0[2].0, "Tag2");
        assert_eq!(video_tags.0.0[3].0, "Tag3");
        assert_eq!(video_tags.0.0[4].0, "Tag4");
    }
}
