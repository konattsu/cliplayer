/// `MusicFileError`をまとめたもの
#[derive(Debug, Clone)]
pub struct MusicFileErrors {
    pub errs: Vec<MusicFileError>,
}

/// 音楽情報のファイルに関するエラー
#[derive(Debug, Clone)]
pub enum MusicFileError {
    /// 月ファイルのエラー
    MonthFile { underlying: String, msg: String },
    /// 年フォルダのエラー
    YearFolder(String),
    /// ディレクトリの読み込み失敗
    ReadDir { dir: String, msg: String },
    /// ファイルの読み込み失敗
    FileRead {
        path: crate::util::FilePath,
        msg: String,
        when: String,
    },
    /// ファイルの書き込み失敗
    FileWrite {
        path: crate::util::FilePath,
        msg: String,
    },
    /// ファイルの内容が不正
    InvalidFileContent {
        path: crate::util::FilePath,
        msg: String,
    },
    /// 対応する年月ファイルが存在しない
    NonExistentMonthFile {
        year: usize,
        month: usize,
        id: Option<crate::model::VideoId>,
    },
    /// 特定のファイル内で動画idが重複
    DuplicateVideoIdOnFile {
        id: crate::model::VideoId,
        file_path: crate::util::FilePath,
    },
    /// ファイル全体に渡ってみたときに, 動画idが重複
    DuplicatedVideoIdAcrossFiles { id: crate::model::VideoId },
    /// 内部の動画とファイル名が一致しない
    VideoFileNameMismatch {
        video_ids: Vec<crate::model::VideoId>,
        file_path: crate::util::FilePath,
    },
}

impl MusicFileErrors {
    /// エラーメッセージを整形して返す
    ///
    /// 文字列の最後に`\n`が付与される
    pub fn to_pretty_string(&self) -> String {
        format!(
            "{}\n",
            self.errs
                .iter()
                .map(|e| e.to_pretty_string())
                .collect::<String>()
        )
    }
}

impl MusicFileError {
    /// エラーメッセージを整形して返す
    ///
    /// 文字列の最後に`\n`が付与される
    pub fn to_pretty_string(&self) -> String {
        match self {
            Self::MonthFile { underlying, msg } => {
                format!("Month file error in {underlying}: {msg}\n")
            }
            Self::YearFolder(msg) => {
                format!("Year folder error: {msg}\n")
            }
            Self::ReadDir { dir, msg } => {
                format!("Failed to read directory {dir}: {msg}\n")
            }
            Self::FileRead { path, msg, when } => {
                format!("Failed to read file {path} when {when}: {msg}\n")
            }
            Self::FileWrite { path, msg } => {
                format!("Failed to write file {path}: {msg}\n")
            }
            Self::InvalidFileContent { path, msg } => {
                format!("Invalid content in file {path}: {msg}\n")
            }
            Self::NonExistentMonthFile { year, month, id } => {
                if let Some(id) = id {
                    format!(
                        "No corresponding file for this video(id: {id}) in {year}/{month}\n"
                    )
                } else {
                    format!("No corresponding file in {year}/{month}\n")
                }
            }
            Self::DuplicateVideoIdOnFile { id, file_path } => {
                format!("Duplicate video id `{id}` found in file {file_path}\n")
            }
            Self::DuplicatedVideoIdAcrossFiles { id } => {
                format!("Duplicate video id `{id}` found across files\n")
            }
            Self::VideoFileNameMismatch {
                video_ids,
                file_path,
            } => {
                format!(
                    "Video ids `{}` do not published year/month according to file name `{}`\n",
                    video_ids
                        .iter()
                        .map(|id| id.as_str())
                        .collect::<Vec<_>>()
                        .join(", "),
                    file_path
                )
            }
        }
    }

    pub fn into_errors(self) -> MusicFileErrors {
        MusicFileErrors { errs: vec![self] }
    }
}

impl From<Vec<MusicFileError>> for MusicFileErrors {
    fn from(errs: Vec<MusicFileError>) -> Self {
        Self { errs }
    }
}
