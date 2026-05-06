// テストでは静的に値を定義
// rust-analyzerがこれ(本番用)をinactiveにするが無視していい
/// アーティストidとその周辺情報
///
/// - `LIVER_SET_PATH` 環境変数で指定されたファイルから読み込む
/// - 未指定時は `data/livers.json` を読み込む
#[cfg(not(any(test, feature = "test-helpers")))]
pub static LOADED_LIVER_DATA: once_cell::sync::Lazy<super::Livers> =
    once_cell::sync::Lazy::new(|| {
        let path = crate::cfg::liver_data_path();
        let path_str = path.to_string_lossy().into_owned();
        let data = std::fs::read_to_string(&path).unwrap_or_else(|e| {
            panic!(
                "Failed to read livers data from {}. reason: {e}",
                path.display()
            )
        });
        let data: super::Livers = serde_json::from_str(&data).unwrap();
        tracing::info!("Loaded {} livers from {}", data.len(), path_str);
        tracing::trace!("Loaded livers data: {:#?}", data);
        data
    });

/// アーティストidとその周辺情報
#[cfg(any(test, feature = "test-helpers"))]
pub static LOADED_LIVER_DATA: once_cell::sync::Lazy<super::Livers> =
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
        let livers: super::Livers =
            serde_json::from_str(LIVER_DATA).expect("will not fail");
        tracing::info!("Loaded {} livers from test data", livers.len());
        tracing::trace!("Loaded livers data: {:#?}", livers);
        livers
    });
