/// 既存の楽曲情報ファイルから, 指定された動画IDが重複しているかどうか確認
///
/// Arguments:
/// - `music_root`: 楽曲情報のルートフォルダ
/// - `video_ids`: 重複を確認したい動画IDのリスト
///
/// Returns:
/// - Ok(a): 重複していた動画id
pub fn find_video_ids(
    music_lib: &crate::music_file::MusicLibrary,
    video_ids: &[crate::model::VideoId],
) -> crate::model::VideoIds {
    let video_ids_set: std::collections::HashSet<_> =
        music_lib.get_video_ids().into_iter().collect();

    video_ids
        .iter()
        .filter_map(|id| video_ids_set.get(id))
        .cloned()
        .collect()
}

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
/// Error時はエラーメッセージを成形して表示
pub fn validate_new_input(file: &crate::util::FilePath) -> Result<(), String> {
    // deserializeできたらok
    let _videos = try_load_anonymous_videos(file).map_err(|e| e.to_pretty_string())?;
    Ok(())
}

// TODO どうしよう
pub fn try_load_anonymous_videos(
    file: &crate::util::FilePath,
) -> Result<crate::model::AnonymousVideos, AnonymousVideoValidateError> {
    deserialize_from_file(file)
}

fn deserialize_from_file(
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
