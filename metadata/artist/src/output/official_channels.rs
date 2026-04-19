#[derive(serde::Serialize, Debug, Clone)]
pub(crate) struct OfficialChannels(
    std::collections::BTreeMap<crate::model::OfficialId, OfficialChannel>,
);

#[derive(serde::Serialize, Debug, Clone)]
struct OfficialChannel {
    ja: String,
    jah: String,
    en: String,
    channel_id: cmn_rs::yt::ChannelId,
}

impl OfficialChannels {
    pub(crate) fn new(official_channels: crate::model::OfficialChannels) -> Self {
        let mut channels = std::collections::BTreeMap::new();

        for (official_id, official_channel) in official_channels.into_iter() {
            channels.insert(official_id, OfficialChannel::new(official_channel));
        }

        Self(channels)
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

impl OfficialChannel {
    fn new(official_channel: crate::model::OfficialChannel) -> Self {
        let official_channel = official_channel.into_inner();
        Self {
            ja: official_channel.ja,
            jah: official_channel.jah,
            en: official_channel.en,
            channel_id: official_channel.channel_id,
        }
    }
}
