use assert_cmd::prelude::*;
use std::io::Write;
use std::process::Command;

const LIVER_SNIPPET_JSON5: &str = r#"{
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

const TAG_SNIPPET_JSON5: &str = r#"{
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

#[test]
fn test_metadata_artist_command_generates_outputs() {
    let tmp = tempfile::tempdir().unwrap();
    let out_dir = tmp.path().join("out");
    std::fs::create_dir_all(&out_dir).unwrap();
    let snippet_path = tmp.path().join("music.code-snippets");
    write_text_file(&snippet_path, LIVER_SNIPPET_JSON5);

    let mut cmd = Command::cargo_bin("metadata").unwrap();
    cmd.arg("artist")
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
        .arg("--music-code-snippets-path")
        .arg(snippet_path.to_string_lossy().to_string());

    cmd.assert().success();

    assert!(out_dir.join("livers_search_index.min.json").exists());
    assert!(out_dir.join("channels.min.json").exists());
    assert!(out_dir.join("livers.min.json").exists());
    assert!(out_dir.join("official_channels.min.json").exists());

    let snippet = std::fs::read_to_string(snippet_path).unwrap();
    assert!(snippet.contains("LiverNamesSnippet"));
    assert!(snippet.contains("\"${1|"));
}

#[test]
fn test_metadata_tag_command_updates_snippet() {
    let tmp = tempfile::tempdir().unwrap();
    let snippet_path = tmp.path().join("tags.code-snippets");
    write_text_file(&snippet_path, TAG_SNIPPET_JSON5);

    let mut cmd = Command::cargo_bin("metadata").unwrap();
    cmd.arg("tag")
        .arg("--code-snippets-path")
        .arg(snippet_path.to_string_lossy().to_string());

    cmd.assert().success();

    let snippet = std::fs::read_to_string(snippet_path).unwrap();
    assert!(snippet.contains("VideoTagsSnippet"));
    assert!(snippet.contains("\"${1|"));
}
