#[derive(serde::Serialize, Debug, Clone)]
pub struct Channels(
    std::collections::HashMap<crate::model::ChannelId, crate::artist::model::ArtistId>,
);

// TODO serialize時に順番を保証するか検討

impl Channels {
    pub fn new(artists: &crate::artist::model::Artists) -> Self {
        let mut channels = std::collections::HashMap::with_capacity(artists.0.len());

        for (artist_id, artist) in artists.0.iter() {
            channels.insert(artist.channel_id.clone(), artist_id.clone());
        }

        Self(channels)
    }

    pub fn output_json(&self, path: &std::path::Path) -> anyhow::Result<()> {
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
