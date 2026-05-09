pub fn hash_music_inputs(
    music_root: &std::path::Path,
) -> Result<String, crate::music_file::MusicFileErrors> {
    let library = crate::music_file::MusicLibraryRepository::load(music_root)?;
    let videos = library.into_videos().map_err(|e| e.into_errors())?;

    let mut builder =
        cmn_rs::min_json::InputSetHashBuilder::new("cliplayer:music-inputs");
    builder
        .add_serializable("verified_videos", &videos)
        .map_err(serialization_error_to_music_file_error)
        .map_err(crate::music_file::MusicFileError::into_errors)?;

    Ok(builder.finish_hex())
}

fn serialization_error_to_music_file_error(
    error: serde_json::Error,
) -> crate::music_file::MusicFileError {
    crate::music_file::MusicFileError::InvalidDatabaseContent {
        msg: format!("failed to serialize canonical music input set: {error}"),
    }
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn test_hash_music_inputs_is_stable() {
        let tempdir = tempfile::tempdir().unwrap();
        let root = tempdir.path().join("music");
        std::fs::create_dir_all(root.join("2026")).unwrap();
        std::fs::write(root.join("2026/01.json"), MONTHLY_FILE_JSON).unwrap();

        let left = super::hash_music_inputs(&root).unwrap();
        let right = super::hash_music_inputs(&root).unwrap();

        assert_eq!(left, right);
    }
}
