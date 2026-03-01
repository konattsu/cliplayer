/// minifyして出力する用

#[derive(serde::Serialize, Debug, Clone)]
pub(crate) struct OutputLivers(
    std::collections::BTreeMap<crate::model::LiverId, OutputLiver>,
);

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct OutputLiver {
    ja: String,
    jah: String,
    en: String,
    color: cmn_rs::color::Color,
    channel_id: cmn_rs::yt::ChannelId,
    #[serde(skip_serializing_if = "is_false")]
    is_graduated: bool,
}

fn is_false(value: &bool) -> bool {
    !*value
}

impl OutputLivers {
    pub(crate) fn new(artists: crate::model::Livers) -> Self {
        let mut map = std::collections::BTreeMap::new();

        for (liver_id, liver) in artists.into_iter() {
            let liver = liver.into_inner();
            let output_artist = OutputLiver {
                ja: liver.ja,
                jah: liver.jah,
                en: liver.en,
                color: liver.color,
                channel_id: liver.channel_id,
                is_graduated: liver.is_graduated,
            };
            map.insert(liver_id, output_artist);
        }
        Self(map)
    }

    pub(crate) fn output_json(&self, path: &std::path::Path) -> anyhow::Result<()> {
        use anyhow::Context;
        let file = std::fs::File::create(path).with_context(|| {
            format!("Failed to create/truncate file at {}", path.display())
        })?;
        serde_json::to_writer(file, self).with_context(|| {
            format!("Failed to write JSON to file: {}", path.display())
        })?;
        Ok(())
    }
}
