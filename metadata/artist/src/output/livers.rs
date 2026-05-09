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

    pub(crate) fn output_json(
        &self,
        path: &std::path::Path,
        metadata: &crate::output::BuildMetadata,
    ) -> anyhow::Result<()> {
        crate::output::minified_json::write_json(path, self, metadata)
    }
}
