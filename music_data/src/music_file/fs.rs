/// 指定されたディレクトリを読み込んで配下にあるエントリ一覧を返す
///
/// # Errors:
/// - ディレクトリが存在しない場合
/// - 読み込みに失敗した場合
///   - e.g. 権限不足
pub(super) fn read_dir(
    path: &std::path::Path,
) -> Result<Vec<std::fs::DirEntry>, super::MusicFileError> {
    use super::MusicFileError;

    std::fs::read_dir(path)
        .map_err(|e| MusicFileError::ReadDirError {
            dir: path.display().to_string(),
            msg: e.to_string(),
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| MusicFileError::ReadDirError {
            dir: path.display().to_string(),
            msg: e.to_string(),
        })
}

/// jsonファイルから楽曲情報をデシリアライズする
pub(super) fn deserialize_from_file(
    file: &crate::util::FilePath,
) -> Result<crate::model::VerifiedVideos, super::MusicFileError> {
    use super::MusicFileError;

    let file_handle = std::fs::File::open(file.as_path()).map_err(|e| {
        MusicFileError::FileReadError {
            path: file.clone(),
            msg: e.to_string(),
            when: "deserializing from file".to_string(),
        }
    })?;

    let reader = std::io::BufReader::new(file_handle);

    serde_json::from_reader(reader).map_err(|e| MusicFileError::InvalidFileContent {
        path: file.clone(),
        msg: e.to_string(),
    })
}

/// jsonファイルに楽曲情報を書き込む
pub(super) fn serialize_to_file(
    file: &crate::util::FilePath,
    content: &crate::model::VerifiedVideos,
) -> Result<(), super::MusicFileError> {
    use super::MusicFileError;

    let file_handle = std::fs::File::create(file.as_path()).map_err(|e| {
        MusicFileError::FileReadError {
            path: file.clone(),
            msg: e.to_string(),
            when: "serializing to file".to_string(),
        }
    })?;

    let write = std::io::BufWriter::new(file_handle);

    serde_json::to_writer(write, content).map_err(|e| MusicFileError::FileWriteError {
        path: file.clone(),
        msg: e.to_string(),
    })
}
