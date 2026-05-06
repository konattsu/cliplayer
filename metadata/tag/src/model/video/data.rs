/// 動画タグのモデル
#[derive(Debug, serde::Serialize, Clone)]
pub struct VideoTags(std::collections::HashMap<VideoTagId, VideoTag>);

impl VideoTags {
    pub(crate) fn iter(&self) -> impl Iterator<Item = (&VideoTagId, &VideoTag)> {
        self.0.iter()
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn is_contains_video_tag_id(&self, id: &str) -> bool {
        self.0.keys().any(|tag_id| tag_id.as_str() == id)
    }

    /// Iterator over tag ids as &str
    fn iter_ids(&self) -> impl Iterator<Item = &str> {
        self.0.keys().map(|id| id.0.as_str())
    }

    /// Return a sorted list of tag IDs
    pub fn sorted_ids(&self) -> Vec<&str> {
        let mut ids: Vec<&str> = self.iter_ids().collect();
        ids.sort_unstable();
        ids
    }
}

/// デシリアライズ時は VideoTagId のバリデーションを一時的に迂回するため
/// `HashMap<String, VideoTag>` として読んでから変換する。
/// (VideoTagId::new() が LOADED_VIDEO_TAG_DATA にアクセスするためデッドロックを防ぐ)
impl<'de> serde::Deserialize<'de> for VideoTags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::collections::HashMap;

        let raw: HashMap<String, VideoTag> =
            serde::Deserialize::deserialize(deserializer)?;
        let map = raw
            .into_iter()
            .map(|(id, tag)| (VideoTagId::from_raw(id), tag))
            .collect::<HashMap<VideoTagId, VideoTag>>();
        Ok(VideoTags(map))
    }
}

/// 動画タグ
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct VideoTag {
    /// 日本語
    pub(crate) ja: String,
    /// 英語
    pub(crate) en: String,
    /// フロントでの再生をブロックするか
    pub(crate) blocked: Option<bool>,
    /// 整数ID
    pub(crate) int_id: u16,
}

/// 動画タグID
#[derive(Debug, serde::Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoTagId(String);

impl VideoTagId {
    pub fn new<'a, T: Into<std::borrow::Cow<'a, str>>>(id: T) -> Result<Self, String> {
        let id = id.into();
        if !Self::is_valid_video_tag_id(&id) {
            Err(format!("invalid video tag: {id}"))
        } else {
            Ok(VideoTagId(id.into_owned()))
        }
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// バリデーションなしで VideoTagId を生成。LOADED_VIDEO_TAG_DATA の初期化時のみ使用。
    pub(super) fn from_raw(id: String) -> Self {
        VideoTagId(id)
    }

    fn is_valid_video_tag_id(id: &str) -> bool {
        crate::model::LOADED_VIDEO_TAG_DATA.is_contains_video_tag_id(id)
    }
}

impl<'de> serde::Deserialize<'de> for VideoTagId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id: String = serde::Deserialize::deserialize(deserializer)?;
        Self::new(id).map_err(serde::de::Error::custom)
    }
}

#[cfg(any(test, feature = "test-helpers"))]
impl VideoTagId {
    /// `karaoke`
    pub fn self_1() -> Self {
        Self::new("karaoke").unwrap()
    }

    /// `3d`
    pub fn self_2() -> Self {
        Self::new("3d").unwrap()
    }

    /// `acoustic`
    pub fn self_3() -> Self {
        Self::new("acoustic").unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_ids() {
        let sorted = crate::model::LOADED_VIDEO_TAG_DATA.sorted_ids();

        assert_eq!(sorted, vec!["3d", "acoustic", "karaoke"]);
    }

    #[test]
    fn new_valid_ids_succeed() {
        assert!(VideoTagId::new("karaoke").is_ok());
        assert!(VideoTagId::new("3d").is_ok());
        assert!(VideoTagId::new("acoustic").is_ok());
    }

    #[test]
    fn new_invalid_ids_fail() {
        assert!(VideoTagId::new("invalid-tag").is_err());
        assert!(VideoTagId::new("").is_err());
    }

    #[test]
    fn deserialize_valid() {
        let json = r#""karaoke""#;
        let tag_id: VideoTagId =
            serde_json::from_str(json).expect("deserialize failed");
        assert_eq!(tag_id.as_str(), "karaoke");
    }

    #[test]
    fn deserialize_invalid() {
        let json = r#""invalid-tag""#;
        let result: Result<VideoTagId, _> = serde_json::from_str(json);
        assert!(result.is_err());

        let json = r#""""#;
        let result: Result<VideoTagId, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
