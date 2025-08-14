#[derive(Debug, Clone)]
pub(in crate::artist) struct Artists(
    pub(in crate::artist) std::collections::HashMap<crate::artist::model::ArtistId, Artist>,
);

#[derive(Debug, Clone)]
pub(in crate::artist) struct Artist {
    pub(in crate::artist) artist_id: crate::artist::model::ArtistId,
    pub(in crate::artist) ja: crate::artist::model::StringNonEmpty,
    pub(in crate::artist) jah: crate::artist::model::StringNonEmpty,
    pub(in crate::artist) en: crate::artist::model::StringNonEmpty,
    pub(in crate::artist) aliases: Vec<crate::artist::model::StringNonEmpty>,
    pub(in crate::artist) channel_id: crate::model::ChannelId,
    pub(in crate::artist) color: crate::model::Color,
    pub(in crate::artist) is_graduated: bool,
}

impl<'de> serde::Deserialize<'de> for Artists {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct RawArtists(
            std::collections::HashMap<crate::artist::model::ArtistId, RawArtist>,
        );

        #[derive(serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct RawArtist {
            ja: crate::artist::model::StringNonEmpty,
            jah: crate::artist::model::StringNonEmpty,
            en: crate::artist::model::StringNonEmpty,
            aliases: Vec<crate::artist::model::StringNonEmpty>,
            channel_id: crate::model::ChannelId,
            color: crate::model::Color,
            is_graduated: Option<bool>,
        }

        let raw_artists = RawArtists::deserialize(deserializer)?;
        raw_artists
            .0
            .into_iter()
            .map(|(artist_id, raw_artist)| {
                let artist = Artist {
                    artist_id,
                    ja: raw_artist.ja,
                    jah: raw_artist.jah,
                    en: raw_artist.en,
                    aliases: raw_artist.aliases,
                    channel_id: raw_artist.channel_id,
                    color: raw_artist.color,
                    is_graduated: raw_artist.is_graduated.unwrap_or(false),
                };
                Ok((artist.artist_id.clone(), artist))
            })
            .collect::<Result<std::collections::HashMap<_, _>, D::Error>>()
            .map(Artists)
    }
}

impl Artists {
    pub(in crate::artist) fn load(path: &str) -> anyhow::Result<Self> {
        let file = std::fs::File::open(path)
            .map_err(|e| anyhow::anyhow!("Failed to open file {}: {}", path, e))?;
        serde_json::from_reader(file).map_err(|e| {
            anyhow::anyhow!("Failed to deserialize artists data from {}: {}", path, e)
        })
    }
}

mod tests {
    #[allow(unused)]
    const TEST_ARTISTS_DATA: &str = r#"
{
    "mito-tsukino": {
        "ja": "月ノ美兎",
        "jah": "つきのみと",
        "en": "Tsukino Mito",
        "aliases": ["いいんちょう"],
        "channelId": "UCD-miitqNY3nyukJ4Fnf4_A",
        "color": "E43F3B"
    },
    "chihiro-yuki": {
        "ja": "勇気ちひろ",
        "jah": "ゆうきちひろ",
        "en": "Yuki Chihiro",
        "aliases": [],
        "channelId": "UCLO9QDxVL4bnvRRsz6K4bsQ",
        "color": "7BB3EE",
        "isGraduated": true
    },
    "elu": {
        "ja": "える",
        "jah": "える",
        "en": "Elu",
        "aliases": [],
        "channelId": "UCYKP16oMX9KKPbrNgo_Kgag",
        "color": "E2364F"
    }
}"#;

    #[test]
    fn test_artists_data_deserialization() {
        let _a: super::Artists = serde_json::from_str(TEST_ARTISTS_DATA)
            .expect("Failed to deserialize artists data");
    }
}
