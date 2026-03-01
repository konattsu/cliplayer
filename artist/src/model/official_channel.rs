#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct OfficialChannels(std::collections::HashMap<OfficialId, OfficialChannel>);

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct OfficialChannel {
    /// 日本語での名前
    ja: String,
    /// 平仮名での名前
    jah: String,
    /// 英語での名前
    en: String,
    /// エイリアス
    aliases: Vec<String>,
    /// チャンネルid
    channel_id: cmn_rs::yt::ChannelId,
    /// 整数id
    int_id: u16,
}

#[derive(
    serde::Deserialize,
    serde::Serialize,
    Debug,
    Clone,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
pub struct OfficialId(String);

pub struct OfficialChannelsInner {
    pub ja: String,
    pub jah: String,
    pub en: String,
    pub aliases: Vec<String>,
    pub channel_id: cmn_rs::yt::ChannelId,
}

impl OfficialChannels {
    fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn into_iter(
        self,
    ) -> impl Iterator<Item = (OfficialId, OfficialChannel)> {
        self.0.into_iter()
    }
}

impl OfficialChannel {
    pub fn into_inner(self) -> OfficialChannelsInner {
        OfficialChannelsInner {
            ja: self.ja,
            jah: self.jah,
            en: self.en,
            aliases: self.aliases,
            channel_id: self.channel_id,
        }
    }
}

impl OfficialId {
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(not(test))]
pub static LOADED_OFFICIAL_CHANNEL_DATA: once_cell::sync::Lazy<OfficialChannels> =
    once_cell::sync::Lazy::new(|| {
        const OFFICIAL_CHANNEL_PATH: &str = "./artist/data/official_channels.json";

        let path_str = std::env::var("OFFICIAL_CHANNEL_PATH")
            .unwrap_or_else(|_| OFFICIAL_CHANNEL_PATH.to_string());
        let data = std::fs::read_to_string(path_str.clone()).unwrap_or_else(|e| {
            panic!(
                "Failed to read official channels data from {path_str}. \
                This value is read from the env value, or default to {OFFICIAL_CHANNEL_PATH}. \
                reason: {e}"
            )
        });
        let data: OfficialChannels = serde_json::from_str(&data).unwrap();
        tracing::info!("Loaded {} official channels from {}", data.len(), path_str);
        tracing::debug!("Loaded official channels data: {:#?}", data);
        data
    });

#[cfg(test)]
pub static LOADED_OFFICIAL_CHANNEL_DATA: once_cell::sync::Lazy<OfficialChannels> =
    once_cell::sync::Lazy::new(|| {
        const OFFICIAL_CHANNEL_DATA: &str = r#"
        {
            "test-channel-1": {
                "ja": "テストチャンネル1",
                "jah": "てすとちゃんねるいち",
                "en": "Test Channel 1",
                "aliases": ["てすといち"],
                "channelId": "UC1111111111111111111111",
                "intId": 950
            },
            "test-channel-2": {
                "ja": "テストチャンネル2",
                "jah": "てすとちゃんねるに",
                "en": "Test Channel 2",
                "aliases": ["てすとに"],
                "channelId": "UC2222222222222222222222",
                "intId": 951
            },
            "test-channel-3": {
                "ja": "テストチャンネル3",
                "jah": "てすとちゃんねるさん",
                "en": "Test Channel 3",
                "aliases": ["てすとさん"],
                "channelId": "UC3333333333333333333333",
                "intId": 952
            }
        }"#;
        let channels: OfficialChannels =
            serde_json::from_str(OFFICIAL_CHANNEL_DATA).unwrap();
        tracing::info!("Loaded {} official channels from test data", channels.len());
        tracing::debug!("Loaded official channels data: {:#?}", channels);
        channels
    });
