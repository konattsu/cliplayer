/// 動画タグのモデル
#[derive(Debug, serde::Deserialize, Clone)]
pub struct VideoTags(std::collections::HashMap<VideoTagId, VideoTag>);

impl VideoTags {
    pub(crate) fn iter(&self) -> impl Iterator<Item = (&VideoTagId, &VideoTag)> {
        self.0.iter()
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    /// Iterator over tag ids as &str
    fn iter_ids(&self) -> impl Iterator<Item = &str> {
        self.0.keys().map(|id| id.0.as_str())
    }

    /// Return a sorted list of tag IDs
    pub(crate) fn sorted_ids(&self) -> Vec<&str> {
        let mut ids: Vec<&str> = self.iter_ids().collect();
        ids.sort_unstable();
        ids
    }
}

/// 動画タグ
#[derive(Debug, serde::Deserialize, Clone)]
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
#[derive(Debug, serde::Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct VideoTagId(String);

impl VideoTagId {
    pub(crate) fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sorted_ids() {
        let sorted = crate::model::LOADED_VIDEO_TAG_DATA.sorted_ids();

        assert_eq!(sorted, vec!["3d", "acoustic", "karaoke"]);
    }
}
