// NOTE VideoTags, ClipTagsは存在できる制約は同じだが混同しないように型を分離

/// 動画に適用するタグ
///
/// - タグは空文字列を許容しない
/// - 内部ではタグをソートして保持する
/// - deserialize時にnullはベクタが空とみなす
#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct VideoTags(Tags);

impl<'de> serde::Deserialize<'de> for VideoTags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let tags = Tags::deserialize(deserializer)?;
        Ok(VideoTags::from_tags(tags))
    }
}

impl VideoTags {
    pub(crate) fn to_vec(&self) -> Vec<&str> {
        self.0.0.iter().map(|tag| tag.0.as_str()).collect()
    }

    fn from_tags(tags: Tags) -> Self {
        VideoTags(tags)
    }
}

/// クリップに適用するタグ
///
/// - タグは空文字列を許容しない
/// - 内部ではタグをソートして保持する
/// - deserialize時にnullはベクタが空とみなす
#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct ClipTags(Tags);

impl<'de> serde::Deserialize<'de> for ClipTags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let tags = Tags::deserialize(deserializer)?;
        Ok(ClipTags::from_tags(tags))
    }
}

impl ClipTags {
    pub(crate) fn to_vec(&self) -> Vec<&str> {
        self.0.0.iter().map(|tag| tag.0.as_str()).collect()
    }

    fn from_tags(tags: Tags) -> Self {
        ClipTags(tags)
    }
}

/// タグたち
///
/// - タグは空文字列を許容しない
/// - 内部ではタグをソートして保持する
/// - deserialize時にnullはベクタが空とみなす
#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq, Default)]
struct Tags(Vec<Tag>);

impl<'de> serde::Deserialize<'de> for Tags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // nullだとdefault(空のベクタ)とみなす
        let tags: Vec<Tag> =
            Option::<Vec<Tag>>::deserialize(deserializer)?.unwrap_or_default();
        Ok(Tags::from_vec(tags))
    }
}

impl Tags {
    fn from_vec(tags: Vec<Tag>) -> Self {
        let mut tags = tags;
        tags.sort();
        Tags(tags)
    }
}

/// タグ
///
/// - タグは空文字列を許容しない
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

// MARK: For Tests

#[cfg(test)]
impl VideoTags {
    /// returns `Test Video Tag1`
    pub(crate) fn self_1() -> Self {
        VideoTags(Tags(vec![Tag::new("Test Video Tag1".to_string()).unwrap()]))
    }
    /// returns `Test Video Tag2`
    pub(crate) fn self_2() -> Self {
        VideoTags(Tags(vec![Tag::new("Test Video Tag2".to_string()).unwrap()]))
    }
    /// returns `Test Video Tag3`, `Test Video Tag4`
    pub(crate) fn self_3() -> Self {
        VideoTags(Tags(vec![
            Tag::new("Test Video Tag3".to_string()).unwrap(),
            Tag::new("Test Video Tag4".to_string()).unwrap(),
        ]))
    }

    /// 動画タグを新規作成する
    ///
    /// - Error: タグが空文字列のとき
    /// - Ok: 空文字列でないとき, ベクタが空の時も許容
    pub(crate) fn new(tags: Vec<String>) -> Result<Self, &'static str> {
        Tags::new(tags).map(VideoTags)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.0.0.is_empty()
    }
}

#[cfg(test)]
impl ClipTags {
    /// returns `Test Clip Tag1`
    pub(crate) fn self_1() -> Self {
        ClipTags(Tags(vec![Tag::new("Test Clip Tag1".to_string()).unwrap()]))
    }
    /// returns `Test Clip Tag2`
    pub(crate) fn self_2() -> Self {
        ClipTags(Tags(vec![Tag::new("Test Clip Tag2".to_string()).unwrap()]))
    }
    /// returns `Test Clip Tag3`, `Test Clip Tag4`
    pub(crate) fn self_3() -> Self {
        ClipTags(Tags(vec![
            Tag::new("Test Clip Tag3".to_string()).unwrap(),
            Tag::new("Test Clip Tag4".to_string()).unwrap(),
        ]))
    }

    /// クリップタグを新規作成する
    ///
    /// - Error: タグが空文字列のとき
    /// - Ok: 空文字列でないとき, ベクタが空の時も許容
    pub(crate) fn new(tags: Vec<String>) -> Result<Self, &'static str> {
        Tags::new(tags).map(ClipTags)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.0.0.is_empty()
    }
}

#[cfg(test)]
impl Tags {
    /// タグのベクタを新規作成
    ///
    /// ソートして保持
    ///
    /// - Error: タグが空文字列のとき
    /// - Ok: 空文字列でないとき, ベクタが空の時も許容
    fn new(tags: Vec<String>) -> Result<Self, &'static str> {
        let mut tags = tags
            .into_iter()
            .map(|tag| Tag::new(tag))
            .collect::<Result<Vec<Tag>, &'static str>>()?;
        tags.sort();
        Ok(Self(tags))
    }
}

// MARK: Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tags_for_test_methods() {
        let video_tags_1 = VideoTags::self_1();
        assert_eq!(video_tags_1.0.0[0].0, "Test Video Tag1");
        let video_tags_2 = VideoTags::self_2();
        assert_eq!(video_tags_2.0.0[0].0, "Test Video Tag2");
        let video_tags_3 = VideoTags::self_3();
        assert_eq!(video_tags_3.0.0[0].0, "Test Video Tag3");
        assert_eq!(video_tags_3.0.0[1].0, "Test Video Tag4");

        let clip_tags_1 = ClipTags::self_1();
        assert_eq!(clip_tags_1.0.0[0].0, "Test Clip Tag1");
        let clip_tags_2 = ClipTags::self_2();
        assert_eq!(clip_tags_2.0.0[0].0, "Test Clip Tag2");
        let clip_tags_3 = ClipTags::self_3();
        assert_eq!(clip_tags_3.0.0[0].0, "Test Clip Tag3");
        assert_eq!(clip_tags_3.0.0[1].0, "Test Clip Tag4");
    }

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

    #[test]
    fn test_tags_new_invalid() {
        let empty_tag = Tag::new("".to_string());
        assert!(empty_tag.is_err());

        let video_tags = VideoTags::new(vec!["".to_string()]);
        assert!(video_tags.is_err());

        let clip_tags = ClipTags::new(vec!["".to_string()]);
        assert!(clip_tags.is_err());
    }

    #[test]
    fn test_tags_deserialize() {
        #[derive(serde::Deserialize)]
        struct ForDe {
            #[serde(default)]
            video: VideoTags,
            #[serde(default)]
            clip: ClipTags,
        }

        let json =
            r#"{"video": ["Tag2", "Tag3", "Tag1"], "clip": ["Tag2", "Tag3", "Tag1"]}"#;
        let de: ForDe = serde_json::from_str(json).unwrap();
        // 整列してるか
        assert_eq!(de.video.0.0.len(), 3);
        assert_eq!(de.video.0.0[0].0, "Tag1");
        assert_eq!(de.video.0.0[1].0, "Tag2");
        assert_eq!(de.video.0.0[2].0, "Tag3");
        // 整列してるか
        assert_eq!(de.clip.0.0.len(), 3);
        assert_eq!(de.clip.0.0[0].0, "Tag1");
        assert_eq!(de.clip.0.0[1].0, "Tag2");
        assert_eq!(de.clip.0.0[2].0, "Tag3");

        let json_empty = r#"{"video": null, "clip": null}"#;
        let de_empty: ForDe = serde_json::from_str(json_empty).unwrap();
        assert!(de_empty.video.is_empty());
        assert!(de_empty.clip.is_empty());
    }
}
