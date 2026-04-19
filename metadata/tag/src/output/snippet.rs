#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub(crate) struct Snippet {
    #[serde(rename = "VideoTagsSnippet")]
    video_tags_data: VideoTagDataSnippet,
    #[serde(flatten)]
    other: serde_json::Value,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct VideoTagDataSnippet {
    body: Vec<String>,
    #[serde(flatten)]
    other: serde_json::Value,
}

impl Snippet {
    pub(crate) fn load(
        video_tags_data_code_snippets_path: &std::path::Path,
    ) -> anyhow::Result<Self> {
        let path = video_tags_data_code_snippets_path;
        let content = std::fs::read_to_string(path).map_err(|e| {
            anyhow::anyhow!(
                "Failed to read snippet file: {} | source: {}",
                path.display(),
                e
            )
        })?;

        json5::from_str(&content).map_err(|e| {
            anyhow::anyhow!(
                "Failed to deserialize from {} | source: {}",
                path.display(),
                e
            )
        })
    }

    pub(crate) fn output_json(
        &mut self,
        video_tags_data_code_snippets_path: &std::path::Path,
        video_tags: &crate::model::VideoTags,
    ) -> anyhow::Result<()> {
        use anyhow::Context;

        self.rewrite_body(video_tags)?;

        let path = video_tags_data_code_snippets_path;

        let file = std::fs::File::create(path).with_context(|| {
            format!("Failed to create/truncate file: {}", path.display())
        })?;
        serde_json::to_writer_pretty(file, &self).with_context(|| {
            format!("Failed to write JSON to file: {}", path.display())
        })?;

        Ok(())
    }

    fn rewrite_body(
        &mut self,
        video_tags: &crate::model::VideoTags,
    ) -> anyhow::Result<()> {
        use anyhow::Context;

        let tag_ids = video_tags.sorted_ids();
        let tags_str = tag_ids.join(",");

        let body = self
            .video_tags_data
            .body
            .get_mut(0)
            .context("Failed to access `body` of video_tags_data")?;

        *body = format!("\"${{1|{tags_str}|}}\",");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rewrite_body_updates_first_entry() {
        let mut snippet = Snippet {
            video_tags_data: VideoTagDataSnippet {
                body: vec!["placeholder".to_string(), "".to_string()],
                other: serde_json::json!({
                    "prefix": "vtag",
                }),
            },
            other: serde_json::json!({}),
        };

        snippet
            .rewrite_body(&crate::model::LOADED_VIDEO_TAG_DATA)
            .expect("rewrite should succeed");

        assert_eq!(
            snippet.video_tags_data.body[0],
            "\"${1|3d,acoustic,karaoke|}\","
        );
    }
}
