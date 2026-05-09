use assert_cmd::prelude::*;
use std::io::Write;
use std::process::Command;

const LIVER_SNIPPET_JSON: &str = r#"{
  "LiverNamesSnippet": {
    "prefix": "liver",
    "body": ["placeholder"],
    "description": "test snippet"
  },
  "keep": {
    "x": 1
  }
}
"#;

const TAG_SNIPPET_JSON: &str = r#"{
  "VideoTagsSnippet": {
    "prefix": "vtag",
    "body": ["placeholder"],
    "description": "test snippet"
  },
  "keep": {
    "x": 1
  }
}
"#;

fn write_text_file(path: &std::path::Path, content: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

fn read_json(path: &std::path::Path) -> serde_json::Value {
    serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}

const TEST_DATASET_BUILD_ID: &str =
    "dataset-build-20260509abcdef0123456789abcdef0123456789abcdef01234567";

#[test]
fn test_metadata_artist_minify_command_generates_outputs() {
    let tmp = tempfile::tempdir().unwrap();
    let out_dir = tmp.path().join("out");
    std::fs::create_dir_all(&out_dir).unwrap();

    let mut cmd = Command::cargo_bin("metadata").unwrap();
    cmd.arg("artist")
        .arg("minify")
        .arg("--output-dir")
        .arg(out_dir.to_string_lossy().to_string())
        .arg("--min-livers-search-index-file-name")
        .arg("livers_search_index.min.json")
        .arg("--min-channels-file-name")
        .arg("channels.min.json")
        .arg("--min-livers-file-name")
        .arg("livers.min.json")
        .arg("--min-official-channels-file-name")
        .arg("official_channels.min.json")
        .arg("--dataset-build-id")
        .arg(TEST_DATASET_BUILD_ID);

    cmd.assert().success();

    assert!(out_dir.join("livers_search_index.min.json").exists());
    assert!(out_dir.join("channels.min.json").exists());
    assert!(out_dir.join("livers.min.json").exists());
    assert!(out_dir.join("official_channels.min.json").exists());

    let livers = read_json(&out_dir.join("livers.min.json"));
    assert_eq!(livers["schemaVersion"], 1);
    assert_eq!(livers["datasetBuildId"], TEST_DATASET_BUILD_ID);
    assert!(livers["data"]["riku-tazumi"].is_object());

    let channels = read_json(&out_dir.join("channels.min.json"));
    assert_eq!(channels["schemaVersion"], 1);
    assert!(channels["data"].is_object());

    let search_index = read_json(&out_dir.join("livers_search_index.min.json"));
    assert_eq!(search_index["schemaVersion"], 1);
    assert!(search_index["data"].is_array());
}

#[test]
fn test_metadata_artist_minify_requires_dataset_build_id() {
    let tmp = tempfile::tempdir().unwrap();
    let out_dir = tmp.path().join("out");
    std::fs::create_dir_all(&out_dir).unwrap();

    let mut cmd = Command::cargo_bin("metadata").unwrap();
    cmd.arg("artist")
        .arg("minify")
        .arg("--output-dir")
        .arg(out_dir.to_string_lossy().to_string())
        .arg("--min-livers-search-index-file-name")
        .arg("livers_search_index.min.json")
        .arg("--min-channels-file-name")
        .arg("channels.min.json")
        .arg("--min-livers-file-name")
        .arg("livers.min.json")
        .arg("--min-official-channels-file-name")
        .arg("official_channels.min.json");

    cmd.assert().failure();
}

#[test]
fn test_metadata_artist_snippet_updates_snippet_only() {
    let tmp = tempfile::tempdir().unwrap();
    let out_dir = tmp.path().join("out");
    std::fs::create_dir_all(&out_dir).unwrap();
    let snippet_path = tmp.path().join("music.code-snippets");
    write_text_file(&snippet_path, LIVER_SNIPPET_JSON);

    let mut cmd = Command::cargo_bin("metadata").unwrap();
    cmd.arg("artist")
        .arg("snippet")
        .arg("--music-code-snippets-path")
        .arg(snippet_path.to_string_lossy().to_string());

    cmd.assert().success();

    let snippet = std::fs::read_to_string(snippet_path).unwrap();
    assert!(snippet.contains("LiverNamesSnippet"));
    assert!(snippet.contains("\"${1|"));
    assert!(!out_dir.join("livers.min.json").exists());
}

#[test]
fn test_metadata_artist_hash_inputs_prints_sha256() {
    use assert_cmd::assert::OutputAssertExt;

    let mut cmd = Command::cargo_bin("metadata").unwrap();
    cmd.arg("--quiet").arg("artist").arg("hash-inputs");

    let output = cmd.assert().success().get_output().stdout.clone();
    let hash = String::from_utf8(output).unwrap();
    let hash = hash.trim();
    assert_eq!(hash.len(), 64);
    assert!(hash.bytes().all(|byte| byte.is_ascii_hexdigit()));
}

#[test]
fn test_metadata_tag_snippet_updates_snippet() {
    let tmp = tempfile::tempdir().unwrap();
    let out_dir = tmp.path().join("out");
    std::fs::create_dir_all(&out_dir).unwrap();
    let snippet_path = tmp.path().join("tags.code-snippets");
    write_text_file(&snippet_path, TAG_SNIPPET_JSON);

    let mut cmd = Command::cargo_bin("metadata").unwrap();
    cmd.arg("tag")
        .arg("snippet")
        .arg("--code-snippets-path")
        .arg(snippet_path.to_string_lossy().to_string());

    cmd.assert().success();

    let snippet = std::fs::read_to_string(snippet_path).unwrap();
    assert!(snippet.contains("VideoTagsSnippet"));
    assert!(snippet.contains("\"${1|"));
    assert!(!out_dir.join("tags.min.json").exists());
}

#[test]
fn test_metadata_tag_minify_generates_min() {
    let tmp = tempfile::tempdir().unwrap();
    let out_dir = tmp.path().join("out");
    std::fs::create_dir_all(&out_dir).unwrap();

    let mut cmd = Command::cargo_bin("metadata").unwrap();
    cmd.arg("tag")
        .arg("minify")
        .arg("--output-dir")
        .arg(out_dir.to_string_lossy().to_string())
        .arg("--min-tags-file-name")
        .arg("tags.min.json")
        .arg("--dataset-build-id")
        .arg(TEST_DATASET_BUILD_ID);

    cmd.assert().success();
    assert!(out_dir.join("tags.min.json").exists());

    let tags = read_json(&out_dir.join("tags.min.json"));
    assert_eq!(tags["schemaVersion"], 1);
    assert_eq!(tags["datasetBuildId"], TEST_DATASET_BUILD_ID);
}

#[test]
fn test_metadata_tag_minify_requires_dataset_build_id() {
    let tmp = tempfile::tempdir().unwrap();
    let out_dir = tmp.path().join("out");
    std::fs::create_dir_all(&out_dir).unwrap();

    let mut cmd = Command::cargo_bin("metadata").unwrap();
    cmd.arg("tag")
        .arg("minify")
        .arg("--output-dir")
        .arg(out_dir.to_string_lossy().to_string())
        .arg("--min-tags-file-name")
        .arg("tags.min.json");

    cmd.assert().failure();
}

#[test]
fn test_metadata_tag_hash_inputs_prints_sha256() {
    use assert_cmd::assert::OutputAssertExt;

    let mut cmd = Command::cargo_bin("metadata").unwrap();
    cmd.arg("--quiet").arg("tag").arg("hash-inputs");

    let output = cmd.assert().success().get_output().stdout.clone();
    let hash = String::from_utf8(output).unwrap();
    let hash = hash.trim();
    assert_eq!(hash.len(), 64);
    assert!(hash.bytes().all(|byte| byte.is_ascii_hexdigit()));
}
