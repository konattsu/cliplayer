/// 複数の[`AnonymousVideoValidateError`]をまとめたもの
#[derive(Debug)]
pub struct AnonymousVideoValidateErrors {
    errs: Vec<AnonymousVideoValidateError>,
}

impl std::error::Error for AnonymousVideoValidateErrors {}

/// anonymous videoの検証エラー
#[derive(Debug)]
pub(crate) enum AnonymousVideoValidateError {
    /// 動画idが重複
    DuplicateVideoId(Vec<crate::model::VideoId>),
    /// ファイルの読み込み失敗
    FileReadError {
        path: std::path::PathBuf,
        msg: String,
    },
    /// ファイルの内容が不正
    InvalidFileContent {
        path: std::path::PathBuf,
        msg: String,
    },
}

impl std::fmt::Display for AnonymousVideoValidateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateVideoId(ids) => {
                let ids_str = ids
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "duplicate video IDs found: {ids_str}")
            }
            Self::FileReadError { path, msg } => {
                write!(f, "failed to read file {}: {msg}", path.display())
            }
            Self::InvalidFileContent { path, msg } => {
                write!(f, "invalid content in file {}: {msg}", path.display())
            }
        }
    }
}

impl std::error::Error for AnonymousVideoValidateError {}

impl std::fmt::Display for AnonymousVideoValidateErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.errs
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl From<AnonymousVideoValidateErrors> for Vec<AnonymousVideoValidateError> {
    fn from(value: AnonymousVideoValidateErrors) -> Self {
        value.errs
    }
}

/// anonymous videosのファイルを読み込む
pub fn try_load_anonymous_videos(
    files: &[std::path::PathBuf],
) -> Result<crate::model::AnonymousVideos, AnonymousVideoValidateErrors> {
    let mut videos = crate::model::AnonymousVideos::new();
    let mut duplicate_ids: Vec<crate::model::VideoId> = Vec::new();
    let mut errs: Vec<AnonymousVideoValidateError> = Vec::new();

    for file in files {
        match deserialize_anonymous_from_file(file) {
            Ok(anonymous_videos) => {
                if let Some(ids) = videos.extend(anonymous_videos) {
                    duplicate_ids.extend(ids);
                }
            }
            Err(e) => errs.push(e),
        }
    }

    if !duplicate_ids.is_empty() {
        let duplicate_ids = duplicate_ids
            .into_iter()
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        errs.push(AnonymousVideoValidateError::DuplicateVideoId(duplicate_ids));
    }

    if errs.is_empty() {
        Ok(videos)
    } else {
        Err(AnonymousVideoValidateErrors { errs })
    }
}

fn deserialize_anonymous_from_file(
    file: &std::path::Path,
) -> Result<crate::model::AnonymousVideos, AnonymousVideoValidateError> {
    let file_handle = std::fs::File::open(file).map_err(|e| {
        AnonymousVideoValidateError::FileReadError {
            path: file.to_path_buf(),
            msg: e.to_string(),
        }
    })?;

    let reader = std::io::BufReader::new(file_handle);

    serde_json::from_reader(reader).map_err(|e| {
        AnonymousVideoValidateError::InvalidFileContent {
            path: file.to_path_buf(),
            msg: e.to_string(),
        }
    })
}
