/// 動画タグのモデル
#[derive(Debug, serde::Deserialize, Clone)]
pub struct VideoTags(std::collections::HashMap<VideoTagId, VideoTag>);

impl VideoTags {
    fn len(&self) -> usize {
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
#[allow(dead_code)]
#[derive(Debug, serde::Deserialize, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct VideoTag {
    /// 日本語
    ja: String,
    /// 英語
    en: String,
    /// フロントでの再生をブロックするか
    blocked: Option<bool>,
    /// 整数ID
    int_id: u16,
}

/// 動画タグID
#[derive(Debug, serde::Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct VideoTagId(String);

#[cfg(not(any(test, feature = "test-helpers")))]
pub(crate) static LOADED_VIDEO_TAG_DATA: once_cell::sync::Lazy<VideoTags> =
    once_cell::sync::Lazy::new(|| {
        let default_path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("data/tags.json");
        let path_str = std::env::var("VIDEO_TAG_SET_PATH")
            .unwrap_or_else(|_| default_path.to_string_lossy().into_owned());
        let data = std::fs::read_to_string(path_str.clone()).unwrap_or_else(|e| {
            panic!(
                "Failed to read video tags data from {path_str}. \
                This value is read from the env value, or default to {}. \
                reason: {e}",
                default_path.display()
            )
        });
        let data: VideoTags = serde_json::from_str(&data).unwrap();
        tracing::info!("Loaded {} video tags from {}", data.len(), path_str);
        tracing::trace!("Loaded video tags data: {:#?}", data);
        data
    });

#[cfg(any(test, feature = "test-helpers"))]
pub(crate) static LOADED_VIDEO_TAG_DATA: once_cell::sync::Lazy<VideoTags> =
    once_cell::sync::Lazy::new(|| {
        const VIDEO_TAG_DATA: &str = r#"
        {
            "karaoke": {
                "ja": "歌枠",
                "en": "karaoke",
                "intId": 0
            },
            "3d": {
                "ja": "3D",
                "en": "3D",
                "intId": 1
            },
            "acoustic": {
                "ja": "弾き語り",
                "en": "acoustic",
                "blocked": true,
                "intId": 2
            }
        }"#;
        let video_tags: VideoTags = serde_json::from_str(VIDEO_TAG_DATA).unwrap();
        tracing::info!("Loaded {} video tags from test data", video_tags.len());
        tracing::trace!("Loaded video tags data: {:#?}", video_tags);
        video_tags
    });

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_ids() {
        let sorted = LOADED_VIDEO_TAG_DATA.sorted_ids();

        assert_eq!(sorted, vec!["3d", "acoustic", "karaoke"]);
    }
}
