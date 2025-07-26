/// jsonファイルから楽曲情報をデシリアライズする
pub(super) fn deserialize_from_file(
    file: &crate::util::FilePath,
) -> Result<crate::model::VerifiedVideos, super::MusicFileError> {
    use super::MusicFileError;

    let file_handle =
        std::fs::File::open(file.as_path()).map_err(|e| MusicFileError::FileOpen {
            path: file.to_string(),
            msg: e.to_string(),
            when: "deserializing from file".to_string(),
        })?;

    let reader = std::io::BufReader::new(file_handle);

    serde_json::from_reader(reader).map_err(|e| MusicFileError::Deserialize {
        path: file.clone(),
        msg: e.to_string(),
    })
}

/// jsonファイルに楽曲情報を書き込む
///
/// # Arguments
/// - `file`: 書き込むファイルのパス
/// - `content`: 書き込む内容
/// - `is_minimized`: minimizedさせるかどうか
pub(super) fn serialize_to_file<T>(
    file: &crate::util::FilePath,
    content: &T,
    is_minimized: bool,
) -> Result<(), super::MusicFileError>
where
    T: serde::Serialize,
{
    use super::MusicFileError;

    let file_handle = std::fs::File::create(file.as_path()).map_err(|e| {
        MusicFileError::FileOpen {
            path: file.to_string(),
            msg: e.to_string(),
            when: "serializing to file".to_string(),
        }
    })?;

    let write = std::io::BufWriter::new(file_handle);

    if is_minimized {
        serde_json::to_writer(write, content)
    } else {
        serde_json::to_writer_pretty(write, content)
    }
    .map_err(|e| MusicFileError::FileWrite {
        path: file.clone(),
        msg: e.to_string(),
    })
}
