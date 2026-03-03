/// `MusicFileError`をまとめたもの
#[derive(Debug, Clone)]
pub(crate) struct MusicFileErrors {
    pub errs: Vec<MusicFileError>,
}

/// 音楽情報のファイルに関するエラー
#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum MusicFileError {
    /// 無効なパス
    #[error("Invalid path: {path}, {msg}")]
    InvalidPath {
        path: std::path::PathBuf,
        msg: String,
    },
    /// ファイル名(年/月を表す)に対して動画の公開日が適していない
    #[error(
        "Video publish date does not match file name: {ids} \
        in ({year}/{month}) {file_path}"
    )]
    VideoPublishDateMismatch {
        ids: crate::model::VideoIds,
        year: usize,
        month: usize,
        file_path: std::path::PathBuf,
    },
    /// ファイルを開く際にエラーが発生
    #[error("Failed to open {path} when {when}: {msg}")]
    FileOpen {
        path: String,
        msg: String,
        when: String,
    },
    /// ファイルの読み込みに失敗
    #[error("Failed to write content to file {path}: {msg}")]
    FileWrite {
        path: std::path::PathBuf,
        msg: String,
    },
    /// ファイルの内容のデシリアライズに失敗
    #[error("Failed to deserialize file {path}: {msg}")]
    Deserialize {
        path: std::path::PathBuf,
        msg: String,
    },
    /// データベースの内容が不正
    #[error("Invalid content in database: {msg}")]
    InvalidDatabaseContent { msg: String },
}

impl std::fmt::Display for MusicFileErrors {
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

impl MusicFileError {
    pub fn into_errors(self) -> MusicFileErrors {
        MusicFileErrors { errs: vec![self] }
    }
}

impl From<Vec<MusicFileError>> for MusicFileErrors {
    fn from(errs: Vec<MusicFileError>) -> Self {
        Self { errs }
    }
}
