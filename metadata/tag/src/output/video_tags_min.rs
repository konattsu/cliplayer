/// minifyして出力する用

#[derive(serde::Serialize, Debug, Clone)]
pub(crate) struct MinVideoTags(std::collections::BTreeMap<String, MinVideoTag>);

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct MinVideoTag {
    ja: String,
    en: String,
    int_id: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    blocked: Option<bool>,
}

impl MinVideoTags {
    pub(crate) fn new(video_tags: &crate::model::VideoTags) -> Self {
        let mut map = std::collections::BTreeMap::new();

        for (tag_id, tag) in video_tags.iter() {
            map.insert(
                tag_id.as_str().to_string(),
                MinVideoTag {
                    ja: tag.ja.clone(),
                    en: tag.en.clone(),
                    int_id: tag.int_id,
                    blocked: tag.blocked,
                },
            );
        }

        Self(map)
    }

    pub(crate) fn output_json(&self, path: &std::path::Path) -> anyhow::Result<()> {
        use anyhow::Context;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create output directory: {}", parent.display())
            })?;
        }

        let file = std::fs::File::create(path).with_context(|| {
            format!("Failed to create/truncate file at {}", path.display())
        })?;
        serde_json::to_writer(file, &self.0).with_context(|| {
            format!("Failed to write JSON to file: {}", path.display())
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_video_tags_serializes_as_object_map() {
        let out = MinVideoTags::new(&crate::model::LOADED_VIDEO_TAG_DATA);
        let json = serde_json::to_string(&out.0).expect("serialize should succeed");

        // keys come from tag ids
        assert!(json.contains("\"3d\""));
        assert!(json.contains("\"karaoke\""));

        // values include camelCase intId
        assert!(json.contains("\"intId\""));
        assert!(json.contains("\"ja\""));
        assert!(json.contains("\"en\""));
    }
}
