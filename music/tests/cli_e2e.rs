use assert_cmd::prelude::*;
use predicates::str::contains;
use std::io::Write;
use std::process::Command;

const MONTHLY_FILE_JSON: &str = r#"[
  {
    "videoId": "cFc9Ywpk0QU",
    "title": "Test Karaoke Stream",
    "channelId": "UCivwPlOp0ojnMPZj5pNOPPA",
    "publishedAt": "2026-01-19T13:23:27Z",
    "syncedAt": "2026-04-22T01:57:28Z",
    "duration": "PT1H0M0S",
    "privacyStatus": "public",
    "embeddable": true,
    "videoTags": ["karaoke"],
    "clips": [
      {
        "songTitle": "fuwafuwa time",
        "liverIds": ["riku-tazumi"],
        "startTime": "PT3M2S",
        "endTime": "PT6M56S",
        "uuid": "11786ebd-4b42-428b-81f8-ecf791887326"
      }
    ]
  }
]
"#;

const ANONYMOUS_INPUT_JSON: &str = r#"[
  {
    "videoId": "cFc9Ywpk0QU",
    "videoTags": ["karaoke"],
    "clips": [
      {
        "songTitle": "fuwafuwa time",
        "liverIds": ["riku-tazumi"],
        "startTime": "PT3M2S",
        "endTime": "PT6M56S"
      }
    ]
  }
]
"#;

fn write_text_file(path: &std::path::Path, content: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

#[test]
fn test_add_validate_markdown_e2e() {
    let tmp = tempfile::tempdir().unwrap();
    let input_path = tmp.path().join("input/add.json");
    write_text_file(&input_path, ANONYMOUS_INPUT_JSON);

    let mut cmd = Command::cargo_bin("musictl").unwrap();
    cmd.arg("add")
        .arg("validate")
        .arg("--input")
        .arg(input_path.to_string_lossy().to_string())
        .arg("--markdown");

    cmd.assert()
        .success()
        .stdout(contains("# Music Data Summary"));
}

#[test]
fn test_update_validate_e2e() {
    let tmp = tempfile::tempdir().unwrap();
    let music_root = tmp.path().join("music");
    let month_path = music_root.join("2026/01.json");
    write_text_file(&month_path, MONTHLY_FILE_JSON);

    let mut cmd = Command::cargo_bin("musictl").unwrap();
    cmd.arg("update")
        .arg("validate")
        .arg("--music-root-dir")
        .arg(music_root.to_string_lossy().to_string());

    cmd.assert().success();
}

#[test]
fn test_util_find_duplicate_id_e2e() {
    let tmp = tempfile::tempdir().unwrap();
    let music_root = tmp.path().join("music");
    let month_path = music_root.join("2026/01.json");
    write_text_file(&month_path, MONTHLY_FILE_JSON);

    let mut cmd = Command::cargo_bin("musictl").unwrap();
    cmd.arg("util")
        .arg("find")
        .arg("--ids")
        .arg("cFc9Ywpk0QU")
        .arg("--music-root-dir")
        .arg(music_root.to_string_lossy().to_string());

    cmd.assert()
        .success()
        .stdout(contains("Duplicate video IDs found"));
}
