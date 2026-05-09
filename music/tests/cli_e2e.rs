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

const TEST_DATASET_BUILD_ID: &str =
    "dataset-build-20260509abcdef0123456789abcdef0123456789abcdef01234567";

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

#[test]
fn test_min_command_writes_min_files() {
    let tmp = tempfile::tempdir().unwrap();
    let music_root = tmp.path().join("music");
    let month_path = music_root.join("2026/01.json");
    let min_clips = tmp.path().join("public/clips.min.json");
    let min_videos = tmp.path().join("public/videos.min.json");
    write_text_file(&month_path, MONTHLY_FILE_JSON);

    let mut cmd = Command::cargo_bin("musictl").unwrap();
    cmd.arg("build")
        .arg("minify")
        .arg("--music-root-dir")
        .arg(music_root.to_string_lossy().to_string())
        .arg("--min-clips-path")
        .arg(min_clips.to_string_lossy().to_string())
        .arg("--min-videos-path")
        .arg(min_videos.to_string_lossy().to_string())
        .arg("--dataset-build-id")
        .arg(TEST_DATASET_BUILD_ID);

    cmd.assert().success();
    assert!(min_clips.exists());
    assert!(min_videos.exists());

    let clips = read_json(&min_clips);
    assert_eq!(clips["schemaVersion"], 1);
    assert_eq!(clips["datasetBuildId"], TEST_DATASET_BUILD_ID);
    assert!(clips["data"]["11786ebd-4b42-428b-81f8-ecf791887326"].is_object());

    let videos = read_json(&min_videos);
    assert_eq!(videos["schemaVersion"], 1);
    assert_eq!(videos["datasetBuildId"], clips["datasetBuildId"]);
    assert!(videos["data"]["cFc9Ywpk0QU"].is_object());
}

#[test]
fn test_build_minify_command_requires_dataset_build_id() {
    let tmp = tempfile::tempdir().unwrap();
    let music_root = tmp.path().join("music");
    let month_path = music_root.join("2026/01.json");
    let min_clips = tmp.path().join("public/clips.min.json");
    let min_videos = tmp.path().join("public/videos.min.json");
    write_text_file(&month_path, MONTHLY_FILE_JSON);

    let mut cmd = Command::cargo_bin("musictl").unwrap();
    cmd.arg("build")
        .arg("minify")
        .arg("--music-root-dir")
        .arg(music_root.to_string_lossy().to_string())
        .arg("--min-clips-path")
        .arg(min_clips.to_string_lossy().to_string())
        .arg("--min-videos-path")
        .arg(min_videos.to_string_lossy().to_string());

    cmd.assert().failure();
}

#[test]
fn test_hash_inputs_prints_sha256() {
    use assert_cmd::assert::OutputAssertExt;

    let tmp = tempfile::tempdir().unwrap();
    let music_root = tmp.path().join("music");
    let month_path = music_root.join("2026/01.json");
    write_text_file(&month_path, MONTHLY_FILE_JSON);

    let mut cmd = Command::cargo_bin("musictl").unwrap();
    cmd.arg("--quiet")
        .arg("build")
        .arg("hash-inputs")
        .arg("--music-root-dir")
        .arg(music_root.to_string_lossy().to_string());

    let output = cmd.assert().success().get_output().stdout.clone();
    let hash = String::from_utf8(output).unwrap();
    let hash = hash.trim();
    assert_eq!(hash.len(), 64);
    assert!(hash.bytes().all(|byte| byte.is_ascii_hexdigit()));
}
