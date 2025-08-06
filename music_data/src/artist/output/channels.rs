#[derive(serde::Serialize, Debug, Clone)]
pub(in crate::artist) struct Channels(
    std::collections::BTreeMap<crate::model::ChannelId, crate::artist::model::ArtistId>,
);

impl Channels {
    pub(in crate::artist) fn new(artists: &crate::artist::model::Artists) -> Self {
        let mut channels = std::collections::BTreeMap::new();

        for (artist_id, artist) in artists.0.iter() {
            channels.insert(artist.channel_id.clone(), artist_id.clone());
        }

        Self(channels)
    }

    pub(in crate::artist) fn output_json(
        &self,
        path: &std::path::Path,
    ) -> anyhow::Result<()> {
        use anyhow::Context;
        let file = std::fs::File::create(path).with_context(|| {
            format!("Failed to create/truncate file: {}", path.display())
        })?;
        serde_json::to_writer(file, &self.0).with_context(|| {
            format!("Failed to write JSON to file: {}", path.display())
        })?;
        Ok(())
    }
}
