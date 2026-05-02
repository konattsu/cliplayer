/// 動画タグとその周辺情報
///
/// - `VIDEO_TAG_SET_PATH` 環境変数で指定されたファイルから読み込む
/// - 未指定時は `data/tags.json` を読み込む
#[cfg(not(any(test, feature = "test-helpers")))]
pub static LOADED_VIDEO_TAG_DATA: once_cell::sync::Lazy<super::VideoTags> =
    once_cell::sync::Lazy::new(|| {
        let path = crate::cfg::video_tag_data_path();
        let path_str = path.to_string_lossy().into_owned();
        let data = std::fs::read_to_string(&path).unwrap_or_else(|e| {
            panic!(
                "Failed to read video tags data from {}. reason: {e}",
                path.display()
            )
        });
        let data: super::VideoTags = serde_json::from_str(&data).unwrap();
        tracing::info!("Loaded {} video tags from {}", data.len(), path_str);
        tracing::trace!("Loaded video tags data: {:#?}", data);
        data
    });

#[cfg(any(test, feature = "test-helpers"))]
pub static LOADED_VIDEO_TAG_DATA: once_cell::sync::Lazy<super::VideoTags> =
    once_cell::sync::Lazy::new(|| {
        const VIDEO_TAG_DATA: &str = r#"
        {
            "karaoke": {
                "ja": "歌枠",
                "en": "karaoke",
                "intId": 0
            },
            "3d": {
                "ja": "3D",
                "en": "3D",
                "intId": 1
            },
            "acoustic": {
                "ja": "弾き語り",
                "en": "acoustic",
                "blocked": true,
                "intId": 2
            }
        }"#;
        let video_tags: super::VideoTags =
            serde_json::from_str(VIDEO_TAG_DATA).unwrap();
        tracing::info!("Loaded {} video tags from test data", video_tags.len());
        tracing::trace!("Loaded video tags data: {:#?}", video_tags);
        video_tags
    });
