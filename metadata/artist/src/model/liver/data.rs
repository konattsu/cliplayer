/// アーティストとその周辺情報のhashmap
///
/// (artist_id, ArtistDefinition)
#[derive(serde::Serialize, Debug, Clone)]
pub struct Livers(std::collections::HashMap<super::LiverId, Liver>);

/// デシリアライズ時は LiverId のバリデーションを一時的に迂回するため
/// `HashMap<String, Liver>` として読んでから変換する。
/// (LiverId::new() が LOADED_LIVER_DATA にアクセスするためデッドロックを防ぐ)
impl<'de> serde::Deserialize<'de> for Livers {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::collections::HashMap;

        let raw: HashMap<String, Liver> =
            serde::Deserialize::deserialize(deserializer)?;
        let map = raw
            .into_iter()
            .map(|(id, liver)| (super::LiverId::from_raw(id), liver))
            .collect::<HashMap<super::LiverId, Liver>>();
        Ok(Livers(map))
    }
}

/// アーティストとその周辺情報
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Liver {
    /// 日本語での名前
    ja: String,
    /// 平仮名での名前
    jah: String,
    /// 英語での名前
    en: String,
    /// alias
    aliases: Vec<String>,
    /// チャンネルid
    channel_id: cmn_rs::yt::ChannelId,
    /// カラー
    color: cmn_rs::color::Color,
    /// 整数id
    #[serde(default)]
    int_id: u16,
    /// 卒業したか
    #[serde(skip_serializing_if = "is_false")]
    #[serde(default)]
    is_graduated: bool,
}

fn is_false(val: &bool) -> bool {
    !*val
}

pub struct LiverInner {
    pub ja: String,
    pub jah: String,
    pub en: String,
    pub aliases: Vec<String>,
    pub channel_id: cmn_rs::yt::ChannelId,
    pub color: cmn_rs::color::Color,
    pub int_id: u16,
    pub is_graduated: bool,
}

impl Livers {
    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    /// 指定したアーティストIDが存在するか
    pub(crate) fn is_contains_liver_id(&self, id: &str) -> bool {
        self.0.keys().any(|liver_id| liver_id.as_str() == id)
    }

    fn iter_ids(&self) -> impl Iterator<Item = &str> {
        self.0.keys().map(|id| id.as_str())
    }

    /// ソート済みのIDリストを返す
    pub(crate) fn sorted_ids(&self) -> Vec<&str> {
        let mut ids: Vec<&str> = self.iter_ids().collect();
        ids.sort_unstable();
        ids
    }

    /// 指定したIDのアーティストの日本語名を返す. 存在しない場合はNone
    pub(crate) fn get_ja_name(&self, id: &super::LiverId) -> Option<String> {
        self.0.get(id).map(|liver| liver.ja.clone())
    }
}

impl IntoIterator for Livers {
    type Item = (super::LiverId, Liver);
    type IntoIter = std::collections::hash_map::IntoIter<super::LiverId, Liver>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Liver {
    pub fn into_inner(self) -> LiverInner {
        LiverInner {
            ja: self.ja,
            jah: self.jah,
            en: self.en,
            aliases: self.aliases,
            channel_id: self.channel_id,
            color: self.color,
            int_id: self.int_id,
            is_graduated: self.is_graduated,
        }
    }
}

#[cfg(any(test, feature = "test-helpers"))]
#[allow(dead_code)] // because these cau be used by other crates with `test-helpers` feature
impl Liver {
    pub(crate) fn self_1() -> Self {
        Self {
            ja: "田角陸".to_string(),
            jah: "たずみりく".to_string(),
            en: "Tazumi Riku".to_string(),
            aliases: vec!["りっくん".to_string()],
            channel_id: cmn_rs::yt::ChannelId::test_id_1(),
            color: cmn_rs::color::Color::from_rgb(0x11, 0x11, 0x11),
            int_id: 1,
            is_graduated: false,
        }
    }

    /// ゆがみん
    pub(crate) fn self_2() -> Self {
        Self {
            ja: "ゆがみん".to_string(),
            jah: "ゆがみん".to_string(),
            en: "Yugamin".to_string(),
            aliases: vec![],
            channel_id: cmn_rs::yt::ChannelId::test_id_2(),
            color: cmn_rs::color::Color::from_rgb(0x22, 0x22, 0x22),
            int_id: 2,
            is_graduated: false,
        }
    }

    /// ゆーどりっく
    pub(crate) fn self_3() -> Self {
        Self {
            ja: "ユードリック".to_string(),
            jah: "ゆーどりっく".to_string(),
            en: "Yudorikku".to_string(),
            aliases: vec![],
            channel_id: cmn_rs::yt::ChannelId::test_id_3(),
            color: cmn_rs::color::Color::from_rgb(0x33, 0x33, 0x33),
            int_id: 3,
            is_graduated: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter_and_sorted_ids_work() {
        let mut map = std::collections::HashMap::new();
        map.insert(crate::model::LiverId::self_1(), Liver::self_1());
        map.insert(crate::model::LiverId::self_2(), Liver::self_2());
        // intentionally insert in non-sorted order
        let livers = Livers(map);

        let ids: Vec<&str> = livers.iter_ids().collect();
        assert_eq!(ids.len(), 2);
        // order unspecified

        let sorted = livers.sorted_ids();
        assert_eq!(sorted, vec!["riku-tazumi", "yugamin"]);
    }
}
