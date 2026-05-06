#[cfg(not(any(test, feature = "test-helpers")))]
pub static LOADED_OFFICIAL_CHANNEL_DATA: once_cell::sync::Lazy<
    super::OfficialChannels,
> = once_cell::sync::Lazy::new(|| {
    let path = crate::cfg::official_channel_data_path();
    let path_str = path.to_string_lossy().into_owned();
    let data = std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "Failed to read official channels data from {}. reason: {e}",
            path.display()
        )
    });
    let data: super::OfficialChannels = serde_json::from_str(&data).unwrap();
    tracing::info!("Loaded {} official channels from {}", data.len(), path_str);
    tracing::debug!("Loaded official channels data: {:#?}", data);
    data
});

#[cfg(any(test, feature = "test-helpers"))]
pub static LOADED_OFFICIAL_CHANNEL_DATA: once_cell::sync::Lazy<
    super::OfficialChannels,
> = once_cell::sync::Lazy::new(|| {
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
    let channels: super::OfficialChannels =
        serde_json::from_str(OFFICIAL_CHANNEL_DATA).unwrap();
    tracing::info!("Loaded {} official channels from test data", channels.len());
    tracing::debug!("Loaded official channels data: {:#?}", channels);
    channels
});
