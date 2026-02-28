#[derive(serde::Serialize, Debug, Clone)]
pub(crate) struct Channels(
    std::collections::BTreeMap<cmn_rs::yt::ChannelId, ChannelsValue>,
);

#[derive(serde::Serialize, Debug, Clone)]
struct ChannelsValue {
    id: String,
    /// "liver" or "official"
    kind: String,
}

impl Channels {
    pub(crate) fn new(
        livers: &crate::model::Livers,
        official_channels: crate::model::OfficialChannels,
    ) -> Self {
        let mut channels = std::collections::BTreeMap::new();

        Self::add_livers(&mut channels, livers.clone());
        Self::add_official_channels(&mut channels, official_channels);

        Self(channels)
    }

    fn add_livers(
        channels: &mut std::collections::BTreeMap<cmn_rs::yt::ChannelId, ChannelsValue>,
        livers: crate::model::Livers,
    ) {
        const KIND_LIVER: &str = "liver";

        for (liver_id, liver) in livers.into_iter() {
            channels.insert(
                liver.into_inner().channel_id,
                ChannelsValue::new(liver_id.as_str(), KIND_LIVER),
            );
        }
    }

    fn add_official_channels(
        channels: &mut std::collections::BTreeMap<cmn_rs::yt::ChannelId, ChannelsValue>,
        official_channels: crate::model::OfficialChannels,
    ) {
        const KIND_OFFICIAL: &str = "official";

        for (official_id, official_channel) in official_channels.into_iter() {
            channels.insert(
                official_channel.into_inner().channel_id,
                ChannelsValue::new(official_id.as_str(), KIND_OFFICIAL),
            );
        }
    }

    pub(crate) fn output_json(&self, path: &std::path::Path) -> anyhow::Result<()> {
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

impl ChannelsValue {
    fn new(id: &str, kind: &str) -> Self {
        Self {
            id: id.to_string(),
            kind: kind.to_string(),
        }
    }
}
