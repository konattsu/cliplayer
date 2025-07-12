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

impl ValidateError {
    pub fn display_prettier(&self) -> String {
        match self {
            Self::DeserializeError(de) => {
                let mut msg = String::new();
                de.iter().for_each(|e| msg.push_str(&e.display_prettier()));
                msg
            }
            Self::FileOpenError(e) => e.display_prettier(),
        }
    }
}

impl ValidateErrorFileOpen {
    pub fn display_prettier(&self) -> String {
        format!("failed to open file ({}): {}\n", self.file, self.reason)
    }
}

impl ValidateErrorDeserialize {
    pub fn display_prettier(&self) -> String {
        format!("failed to deserialize @ {}: {}\n", self.file, self.reason)
    }
}

/// 既存の楽曲情報の一覧を取得
///
/// Argument
/// - `music_root`: 既存の楽曲情報が含まれているディレクトリ
pub fn get_videos_list_from_music_root(
    music_root: &crate::music_file::MusicRoot,
) -> Result<Vec<FileVideo>, ValidateError> {
    let mut invalid_files: Vec<ValidateErrorDeserialize> = Vec::new();
    let mut videos: Vec<FileVideo> = Vec::new();

    for file in music_root.get_file_paths() {
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
