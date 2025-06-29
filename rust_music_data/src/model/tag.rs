/// 楽曲に適用するタグ
///
/// front側でタグを使用してフィルタリングできるようにする
///
/// 柔軟にするため任意の文字列をタグとして使用可
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tag(String);

impl Tag {
    pub fn new(tag: String) -> Self {
        Tag(tag)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TagList(Vec<Tag>);

impl TagList {
    pub fn new(tags: Vec<Tag>) -> Self {
        TagList(tags)
    }

    pub fn from_vec_str(tags: Vec<&str>) -> Self {
        TagList(
            tags.into_iter()
                .map(|tag| Tag::new(tag.to_string()))
                .collect(),
        )
    }

    pub fn as_slice(&self) -> &[Tag] {
        &self.0
    }

    pub fn into_vec(self) -> Vec<Tag> {
        self.0
    }
}

impl Default for TagList {
    fn default() -> Self {
        TagList(Vec::new())
    }
}
