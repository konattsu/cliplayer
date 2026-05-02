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
    pub(crate) fn len(&self) -> usize {
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
