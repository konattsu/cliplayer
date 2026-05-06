/// 動画やクリップに適用する動画タグIDのリスト
///
/// 内部に `VideoTagId` のリストを保持し、以下を保証する。
/// - 要素は `tagctl::model::VideoTagId` として妥当であること
/// - 要素は `VideoTagId` の順序でソートされていること
/// - 重複がないこと
/// - deserialize 時に `null` は空配列とみなすこと
#[derive(Debug, serde::Serialize, Clone, PartialEq, Eq, Default)]
pub(crate) struct VideoTagIds(Vec<tagctl::model::VideoTagId>);

impl VideoTagIds {
    pub(crate) fn to_vec(&self) -> Vec<&str> {
        self.0
            .iter()
            .map(tagctl::model::VideoTagId::as_str)
            .collect()
    }

    fn sort_dedup_tag_ids(tag_ids: &mut Vec<tagctl::model::VideoTagId>) {
        tag_ids.sort();
        tag_ids.dedup();
    }
}

impl<'de> serde::Deserialize<'de> for VideoTagIds {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut tag_ids =
            Option::<Vec<tagctl::model::VideoTagId>>::deserialize(deserializer)?
                .unwrap_or_default();
        Self::sort_dedup_tag_ids(&mut tag_ids);
        Ok(VideoTagIds(tag_ids))
    }
}

#[cfg(any(test, feature = "test-helpers"))]
#[allow(dead_code)]
impl VideoTagIds {
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn new(tag_ids: Vec<&str>) -> Result<Self, String> {
        let mut tag_ids = tag_ids
            .into_iter()
            .map(tagctl::model::VideoTagId::new)
            .collect::<Result<Vec<_>, _>>()?;
        Self::sort_dedup_tag_ids(&mut tag_ids);
        Ok(VideoTagIds(tag_ids))
    }

    /// returns `karaoke`
    pub(crate) fn self_1() -> Self {
        Self::new(vec!["karaoke"]).unwrap()
    }

    /// returns `3d`
    pub(crate) fn self_2() -> Self {
        Self::new(vec!["3d"]).unwrap()
    }

    /// returns `acoustic`, `karaoke`
    pub(crate) fn self_3() -> Self {
        Self::new(vec!["karaoke", "acoustic"]).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tags_for_test_methods() {
        let video_tags_1 = VideoTagIds::self_1();
        assert_eq!(video_tags_1.to_vec(), vec!["karaoke"]);

        let video_tags_2 = VideoTagIds::self_2();
        assert_eq!(video_tags_2.to_vec(), vec!["3d"]);

        let video_tags_3 = VideoTagIds::self_3();
        assert_eq!(video_tags_3.to_vec(), vec!["acoustic", "karaoke"]);
    }

    #[test]
    fn test_video_tag_ids_new() {
        let video_tags = VideoTagIds::new(vec!["karaoke", "acoustic", "3d"]).unwrap();
        assert_eq!(video_tags.to_vec(), vec!["3d", "acoustic", "karaoke"]);
    }

    #[test]
    fn test_video_tag_ids_new_dedups_duplicates() {
        let video_tags =
            VideoTagIds::new(vec!["karaoke", "acoustic", "karaoke"]).unwrap();
        assert_eq!(video_tags.to_vec(), vec!["acoustic", "karaoke"]);
    }

    #[test]
    fn test_tags_new_invalid() {
        let video_tags = VideoTagIds::new(vec!["invalid-tag"]);
        assert!(video_tags.is_err());

        let video_tags = VideoTagIds::new(vec![""]);
        assert!(video_tags.is_err());
    }

    #[test]
    fn test_tags_deserialize() {
        #[derive(serde::Deserialize)]
        struct ForDe {
            #[serde(default)]
            video: VideoTagIds,
            #[serde(default)]
            clip: VideoTagIds,
        }

        let json = r#"{"video": ["karaoke", "acoustic", "3d"], "clip": ["karaoke", "acoustic", "3d"]}"#;
        let de: ForDe = serde_json::from_str(json).unwrap();
        assert_eq!(de.video.to_vec(), vec!["3d", "acoustic", "karaoke"]);
        assert_eq!(de.clip.to_vec(), vec!["3d", "acoustic", "karaoke"]);

        let json_dup =
            r#"{"video": ["karaoke", "acoustic", "karaoke"], "clip": ["3d", "3d"]}"#;
        let de_dup: ForDe = serde_json::from_str(json_dup).unwrap();
        assert_eq!(de_dup.video.to_vec(), vec!["acoustic", "karaoke"]);
        assert_eq!(de_dup.clip.to_vec(), vec!["3d"]);

        let json_empty = r#"{"video": null, "clip": null}"#;
        let de_empty: ForDe = serde_json::from_str(json_empty).unwrap();
        assert!(de_empty.video.is_empty());
        assert!(de_empty.clip.is_empty());
    }
}
