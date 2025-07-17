/// `MusicFileError`をまとめたもの
#[derive(Debug, Clone)]
pub struct MusicFileErrors {
    pub errs: Vec<MusicFileError>,
}

/// 音楽情報のファイルに関するエラー
#[derive(Debug, Clone)]
pub enum MusicFileError {
    // 月ファイルのエラー
    MonthFileError {
        underlying: String,
        msg: String,
    },
    // 年フォルダのエラー
    YearFolderError(String),
    // ディレクトリの読み込み失敗
    ReadDirError {
        dir: String,
        msg: String,
    },
    // ファイルの読み込み失敗
    FileReadError {
        path: crate::util::FilePath,
        msg: String,
        when: String,
    },
    FileWriteError {
        path: crate::util::FilePath,
        msg: String,
    },
    // ファイルの内容が不正
    InvalidFileContent {
        path: crate::util::FilePath,
        msg: String,
    },
    // 対応する年月ファイルが存在しない
    NonExistentMonthFile {
        year: usize,
        month: usize,
        id: Option<crate::model::VideoId>,
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
            Self::MonthFileError { underlying, msg } => {
                format!("Month file error in {underlying}: {msg}\n")
            }
            Self::YearFolderError(msg) => {
                format!("Year folder error: {msg}\n")
            }
            Self::ReadDirError { dir, msg } => {
                format!("Failed to read directory {dir}: {msg}\n")
            }
            Self::FileReadError { path, msg, when } => {
                format!("Failed to read file {path} when {when}: {msg}\n")
            }
            Self::FileWriteError { path, msg } => {
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
