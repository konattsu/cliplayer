#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub(crate) struct Snippet {
    #[serde(rename = "LiverNamesSnippet")]
    liver_data: ArtistDataSnippet,
    #[serde(flatten)]
    other: serde_json::Value,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct ArtistDataSnippet {
    body: Vec<String>,
    #[serde(flatten)]
    other: serde_json::Value,
}

impl Snippet {
    pub(crate) fn load(
        music_data_code_snippets_path: &std::path::Path,
    ) -> anyhow::Result<Self> {
        let path = music_data_code_snippets_path;
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
        music_data_code_snippets_path: &std::path::Path,
        livers: &crate::model::Livers,
    ) -> anyhow::Result<()> {
        use anyhow::Context;

        self.rewrite_body(livers)?;

        let path = music_data_code_snippets_path;

        let file = std::fs::File::create(path).with_context(|| {
            format!("Failed to create/truncate file: {}", path.display())
        })?;
        serde_json::to_writer_pretty(file, &self).with_context(|| {
            format!("Failed to write JSON to file: {}", path.display())
        })?;

        Ok(())
    }

    fn rewrite_body(&mut self, livers: &crate::model::Livers) -> anyhow::Result<()> {
        use anyhow::Context;

        let liver_ids = livers.sorted_ids();
        let artist_ids_str = liver_ids.join(",");

        let body = self
            .liver_data
            .body
            .get_mut(0)
            .context("Failed to access `body` of artist_data")?;

        *body = format!("\"${{1|{artist_ids_str}|}}\",");
        Ok(())
    }
}
