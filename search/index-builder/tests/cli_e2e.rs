const MONTHLY_FILE_JSON: &str = r#"[
  {
    "videoId": "cFc9Ywpk0QU",
    "title": "Test Karaoke Stream",
    "channelId": "UC1111111111111111111111",
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

fn write_text_file(path: &std::path::Path, content: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(path, content).unwrap();
}

#[test]
fn test_build_writes_search_index_binary() {
    use assert_cmd::assert::OutputAssertExt;
    use assert_cmd::prelude::CommandCargoExt;

    let tempdir = tempfile::tempdir().unwrap();
    let music_root = tempdir.path().join("music");
    let output_path = tempdir.path().join("out").join("search_index.bin");
    write_text_file(&music_root.join("2026/01.json"), MONTHLY_FILE_JSON);

    let mut cmd = std::process::Command::cargo_bin("index-builder").unwrap();
    cmd.arg("build")
        .arg("--music-root-dir")
        .arg(&music_root)
        .arg("--output-path")
        .arg(&output_path);

    cmd.assert().success();
    assert!(output_path.exists());

    let bytes = std::fs::read(output_path).unwrap();
    let reader = index_core::binary::SearchIndexReader::new(&bytes).unwrap();
    assert!(reader.header().record_count > 0);
    assert_eq!(
        reader.metadata_view().unwrap().builder_version(),
        env!("CARGO_PKG_VERSION")
    );
}
