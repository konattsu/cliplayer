pub struct AnonymousVideoValidateErrors {
    errs: Vec<AnonymousVideoValidateError>,
}

pub enum AnonymousVideoValidateError {
    /// 動画idが重複
    DuplicateVideoId(Vec<crate::model::VideoId>),
    /// ファイルの読み込み失敗
    FileReadError {
        path: crate::util::FilePath,
        msg: String,
    },
    /// ファイルの内容が不正
    InvalidFileContent {
        path: crate::util::FilePath,
        msg: String,
    },
}

impl From<AnonymousVideoValidateErrors> for Vec<AnonymousVideoValidateError> {
    fn from(value: AnonymousVideoValidateErrors) -> Self {
        value.errs
    }
}

impl AnonymousVideoValidateErrors {
    pub fn to_pretty_string(&self) -> String {
        self.errs
            .iter()
            .map(|e| e.to_pretty_string())
            .collect::<String>()
    }
}

impl AnonymousVideoValidateError {
    /// エラーメッセージを整形して返す
    ///
    /// 文字列の最後に`\n`が付与される
    pub fn to_pretty_string(&self) -> String {
        match self {
            Self::DuplicateVideoId(ids) => {
                let ids_str = ids
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("Duplicate video IDs found: {ids_str}\n")
            }
            Self::FileReadError { path, msg } => {
                format!("Failed to read file {path}: {msg}\n",)
            }
            Self::InvalidFileContent { path, msg } => {
                format!("Invalid content in file {path}: {msg}\n",)
            }
        }
    }
}

/// 新規の楽曲情報ファイルの形式を検証
///
/// # Return
/// - Ok(String): 検証成功. 入力をmd形式に見やすくした文字列
/// - Err(String): 検証失敗. エラーメッセージ
pub fn validate_new_input_md(
    files: &[crate::util::FilePath],
) -> Result<String, String> {
    let videos = try_load_anonymous_videos(files).map_err(|e| e.to_pretty_string())?;

    let mut md_str = "# Music Data Summary\n".to_string();
    // TODO ソートするようにしてもいいかも
    for video in videos.into_inner().values() {
        md_str.push_str(&video.to_markdown());
    }

    Ok(md_str)
}

/// 新規の楽曲情報ファイルの形式を検証
///
/// # Return
/// - Ok(()): 検証成功.
/// - Err(String): 検証失敗. エラーメッセージ
pub fn validate_new_input(files: &[crate::util::FilePath]) -> Result<(), String> {
    // deserializeできたらok
    let _videos = try_load_anonymous_videos(files).map_err(|e| e.to_pretty_string())?;
    Ok(())
}

/// anonymous videosのファイルを読み込む
pub fn try_load_anonymous_videos(
    files: &[crate::util::FilePath],
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
    file: &crate::util::FilePath,
) -> Result<crate::model::AnonymousVideos, AnonymousVideoValidateError> {
    let file_handle = std::fs::File::open(file.as_path()).map_err(|e| {
        AnonymousVideoValidateError::FileReadError {
            path: file.clone(),
            msg: e.to_string(),
        }
    })?;

    let reader = std::io::BufReader::new(file_handle);

    serde_json::from_reader(reader).map_err(|e| {
        AnonymousVideoValidateError::InvalidFileContent {
            path: file.clone(),
            msg: e.to_string(),
        }
    })
}
