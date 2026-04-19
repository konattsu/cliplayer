use clap::Parser;

fn unique_path(prefix: &str) -> std::path::PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();

    std::env::temp_dir()
        .join(format!("metadata-{prefix}-{}-{nanos}", std::process::id()))
}

fn sorted_keys_from_json_map(path: &std::path::Path) -> Vec<String> {
    let content = std::fs::read_to_string(path).expect("json file should be readable");
    let map: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(&content).expect("json file should be valid");

    let mut keys = map.keys().cloned().collect::<Vec<_>>();
    keys.sort_unstable();
    keys
}

#[test]
fn test_artist_subcommand_generates_outputs() {
    let temp_dir = unique_path("artist-out");
    std::fs::create_dir_all(&temp_dir).expect("temp output dir should be created");

    let snippet_path = unique_path("artist-snippet");
    std::fs::write(
        &snippet_path,
        r#"
        {
          "LiverNamesSnippet": {
            "body": ["\"${1|placeholder|}\",", ""],
            "description": "LiversData object based on music.schema.json",
            "prefix": "liver",
            "scope": "json",
          },
        }
        "#,
    )
    .expect("snippet file should be written");

    let cli = metadata::cli::Cli::parse_from([
        "metadata",
        "artist",
        "--output-dir",
        temp_dir.to_str().expect("temp output dir should be utf-8"),
        "--music-code-snippets-path",
        snippet_path.to_str().expect("snippet path should be utf-8"),
    ]);

    metadata::cli_exec_handler::cli_exec_handler(cli)
        .expect("artist subcommand should succeed");

    for file_name in [
        "livers_search_index.min.json",
        "channels.min.json",
        "livers.min.json",
        "official_channels.min.json",
    ] {
        assert!(
            temp_dir.join(file_name).exists(),
            "{file_name} should exist"
        );
    }

    let expected_ids = sorted_keys_from_json_map(std::path::Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/artist/data/livers.json"
    )));
    let expected_body = format!("\"${{1|{}|}}\",", expected_ids.join(","));

    let snippet_json: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(&snippet_path)
            .expect("snippet file should be readable"),
    )
    .expect("written snippet should be valid json");
    let body = snippet_json["LiverNamesSnippet"]["body"][0]
        .as_str()
        .expect("snippet body should be a string");
    assert_eq!(body, expected_body);
}

#[test]
fn test_tag_subcommand_updates_snippet() {
    let snippet_path = unique_path("tag-snippet");
    std::fs::write(
        &snippet_path,
        r#"
        {
          "VideoTagsSnippet": {
            "body": ["\"${1|placeholder|}\",", ""],
            "description": "Array of tags for video data",
            "prefix": "vtag",
            "scope": "json",
          },
        }
        "#,
    )
    .expect("snippet file should be written");

    let cli = metadata::cli::Cli::parse_from([
        "metadata",
        "tag",
        "--code-snippets-path",
        snippet_path.to_str().expect("snippet path should be utf-8"),
    ]);

    metadata::cli_exec_handler::cli_exec_handler(cli)
        .expect("tag subcommand should succeed");

    let expected_ids = sorted_keys_from_json_map(std::path::Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tag/data/tags.json"
    )));
    let expected_body = format!("\"${{1|{}|}}\",", expected_ids.join(","));

    let snippet_json: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(&snippet_path)
            .expect("snippet file should be readable"),
    )
    .expect("written snippet should be valid json");
    let body = snippet_json["VideoTagsSnippet"]["body"][0]
        .as_str()
        .expect("snippet body should be a string");
    assert_eq!(body, expected_body);
}
