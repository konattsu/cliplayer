#[derive(Debug, Clone)]
pub enum ValidateError {
    /// ファイルを開けない
    FileOpenError(ValidateErrorFileOpen),
    /// デシリアライズに失敗
    DeserializeError(Vec<ValidateErrorDeserialize>),
}

#[derive(Debug, Clone)]
pub struct ValidateErrorFileOpen {
    pub file: crate::util::FilePath,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct ValidateErrorDeserialize {
    pub file: crate::util::FilePath,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct FileVideo {
    pub file: crate::util::FilePath,
    pub videos: crate::model::VerifiedVideos,
}

/// 既存の楽曲情報の一覧を取得
///
/// Arguments
/// - `music_dir`: 既存の楽曲情報が含まれているディレクトリ
///   このディレクトリ配下の`**/*.json`を対象とする
pub fn get_videos_list_from_exist_music_files(
    music_dir: crate::util::DirPath,
) -> Result<Vec<FileVideo>, ValidateError> {
    const FIND_EXT: &str = "json";
    // 形式が正しくないファイルパスを集める
    let mut invalid_files: Vec<ValidateErrorDeserialize> = Vec::new();
    let mut videos: Vec<FileVideo> = Vec::new();

    let files = crate::util::fs::find_files_by_extension(&music_dir, FIND_EXT);
    for file in files {
        match deserialize_from_file::<crate::model::VerifiedVideos>(&file) {
            Ok(verified_videos) => {
                videos.push(FileVideo {
                    file: file.clone(),
                    videos: verified_videos,
                });
            }
            Err(e) => match e {
                ValidateError::FileOpenError(inner) => {
                    // 1つでもファイルが開けなかったらすぐにエラーを返す
                    return Err(ValidateError::FileOpenError(inner));
                }
                ValidateError::DeserializeError(inner) => {
                    invalid_files.extend(inner);
                }
            },
        }
    }

    if invalid_files.is_empty() {
        Ok(videos)
    } else {
        Err(ValidateError::DeserializeError(invalid_files))
    }
}

/// JSONファイルからデシリアライズする
///
/// `T`: デシリアライズしたい型
pub fn deserialize_from_file<T>(
    file: &crate::util::FilePath,
) -> Result<T, ValidateError>
where
    T: serde::de::DeserializeOwned,
{
    let file_handle = std::fs::File::open(file.as_path()).map_err(|e| {
        ValidateError::FileOpenError(ValidateErrorFileOpen {
            file: file.clone(),
            reason: e.to_string(),
        })
    })?;
    let reader = std::io::BufReader::new(file_handle);

    serde_json::from_reader(reader).map_err(|e| {
        ValidateError::DeserializeError(vec![ValidateErrorDeserialize {
            file: file.clone(),
            reason: e.to_string(),
        }])
    })
}
