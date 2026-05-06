#![cfg_attr(any(test, feature = "test-helpers"), allow(dead_code))]
fn manifest_dir() -> &'static str {
    env!("CARGO_MANIFEST_DIR")
}

fn data_path(default_file_name: &str, env_key: &str) -> std::path::PathBuf {
    std::env::var(env_key).map_or_else(
        |_| {
            std::path::Path::new(manifest_dir())
                .join("data")
                .join(default_file_name)
        },
        std::path::PathBuf::from,
    )
}

pub(crate) fn liver_data_path() -> std::path::PathBuf {
    data_path("livers.json", "LIVER_SET_PATH")
}

pub(crate) fn official_channel_data_path() -> std::path::PathBuf {
    data_path("official_channels.json", "OFFICIAL_CHANNEL_PATH")
}
