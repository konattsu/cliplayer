#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub(in crate::artist) struct Snippet {
    #[serde(rename = "ArtistDataSnippet")]
    artist_data: ArtistDataSnippet,
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
    pub(in crate::artist) fn load(
        music_data_code_snippets_path: &std::path::Path,
    ) -> anyhow::Result<Self> {
        let path = music_data_code_snippets_path;

        let reader =
            std::fs::File::open(std::path::PathBuf::from(path)).map_err(|e| {
                anyhow::anyhow!(
                    "Failed to open snippet file: {} | source: {}",
                    path.display(),
                    e.to_string()
                )
            })?;

        serde_json::from_reader(reader).map_err(|e| {
            anyhow::anyhow!(
                "Failed to deserialize from {} | source: {}",
                path.display(),
                e.to_string()
            )
        })
    }

    pub(in crate::artist) fn output_json(
        &mut self,
        music_data_code_snippets_path: &std::path::Path,
        artists: &crate::artist::model::Artists,
    ) -> anyhow::Result<()> {
        use anyhow::Context;

        self.rewrite_body(artists)?;

        let path = music_data_code_snippets_path;

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
        artists: &crate::artist::model::Artists,
    ) -> anyhow::Result<()> {
        use anyhow::Context;
        let artist_ids_str = artists
            .0
            .keys()
            .map(|id| id.as_str())
            .collect::<Vec<&str>>()
            .join(",");
        let body = self
            .artist_data
            .body
            .get_mut(0)
            .context("Failed to access `body` of artist_data")?;

        *body = format!("\"${{1|{artist_ids_str}|}}\"");
        Ok(())
    }
}
