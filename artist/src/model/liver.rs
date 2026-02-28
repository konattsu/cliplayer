mod external;
mod id;
mod ids;

pub(crate) use external::ExternalArtistsName;
pub(crate) use id::LiverId;
pub(crate) use ids::LiverIds;

//

//

//

//

/// アーティストとその周辺情報のhashmap
///
/// (artist_id, ArtistDefinition)
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub(crate) struct Livers(std::collections::HashMap<LiverId, Liver>);

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
    int_id: u16,
    #[serde(skip_serializing_if = "is_false")]
    #[serde(default = "default_for_is_graduate")]
    /// 卒業したか
    is_graduated: bool,
}

fn is_false(val: &bool) -> bool {
    !*val
}
fn default_for_is_graduate() -> bool {
    false
}

#[allow(dead_code)]
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
    fn len(&self) -> usize {
        self.0.len()
    }

    /// 指定したアーティストIDが存在するか
    fn is_contains_liver_id(&self, id: &str) -> bool {
        for liver_id in self.0.keys() {
            if liver_id.as_str() == id {
                return true;
            }
        }
        false
    }

    fn iter_ids(&self) -> impl Iterator<Item = &str> {
        self.0.keys().map(|id| id.as_str())
    }

    /// ソート済みのIDリストを返す。
    pub(crate) fn sorted_ids(&self) -> Vec<&str> {
        let mut ids: Vec<&str> = self.iter_ids().collect();
        ids.sort_unstable();
        ids
    }

    pub(crate) fn into_iter(self) -> impl Iterator<Item = (LiverId, Liver)> {
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

// テストでは静的に値を定義
// rust-analyzerがこれ(本番用)をinactiveにするが無視していい
/// アーティストidとその周辺情報
///
/// - `LIVER_SET_PATH` 環境変数で指定されたファイルから読み込む
///   - 先ほどの環境変数が指定されていないと `ARTIST_SET_PATH` を読み込む
#[cfg(not(test))]
pub static LOADED_LIVER_DATA: once_cell::sync::Lazy<Livers> =
    once_cell::sync::Lazy::new(|| {
        const LIVER_SET_PATH: &str = "./artist/data/artists_data.json";

        let path_str = std::env::var("LIVER_SET_PATH")
            .unwrap_or_else(|_| LIVER_SET_PATH.to_string());
        let data = std::fs::read_to_string(path_str.clone()).unwrap_or_else(|e| {
            panic!(
                "Failed to read livers data from {path_str}. \
                This value is read from the env value, or default to {LIVER_SET_PATH}. \
                reason: {e}"
            )
        });
        let data: Livers = serde_json::from_str(&data).unwrap();
        tracing::info!("Loaded {} livers from {}", data.len(), path_str);
        tracing::debug!("Loaded livers data: {:#?}", data);
        data
    });

/// アーティストidとその周辺情報
#[cfg(test)]
pub static LOADED_LIVER_DATA: once_cell::sync::Lazy<Livers> =
    once_cell::sync::Lazy::new(|| {
        const LIVER_DATA: &str = r#"
        {
            "riku-tazumi": {
                "ja": "田角陸",
                "jah": "たずみりく",
                "en": "Tazumi Riku",
                "aliases": ["りっくん"],
                "channelId": "UC1111111111111111111111",
                "color": "111111"
            },
            "yugamin": {
                "ja": "ゆがみん",
                "jah": "ゆがみん",
                "en": "Yugamin",
                "aliases": [],
                "channelId": "UC2222222222222222222222",
                "color": "222222"
            },
            "yudorikku": {
                "ja": "ユードリック",
                "jah": "ゆーどりっく",
                "en": "Yudorikku",
                "aliases": [],
                "channelId": "UC3333333333333333333333",
                "color": "333333",
                "isGraduated": true
            }
        }"#;
        let livers: Livers = serde_json::from_str(LIVER_DATA).expect("will not fail");
        tracing::info!("Loaded {} livers from test data", livers.len());
        tracing::debug!("Loaded livers data: {:#?}", livers);
        livers
    });

#[cfg(test)]
impl Liver {
    /// りっくん
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
    fn iter_and_sorted_ids_work() {
        let mut map = std::collections::HashMap::new();
        map.insert(LiverId::self_1(), Liver::self_1());
        map.insert(LiverId::self_2(), Liver::self_2());
        // intentionally insert in non-sorted order
        let livers = Livers(map);

        let ids: Vec<&str> = livers.iter_ids().collect();
        assert_eq!(ids.len(), 2);
        // order unspecified

        let sorted = livers.sorted_ids();
        assert_eq!(sorted, vec!["riku-tazumi", "yugamin"]);
    }
}
