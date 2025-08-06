/// `MusicFileError`をまとめたもの
#[derive(Debug, Clone)]
pub struct MusicFileErrors {
    pub errs: Vec<MusicFileError>,
}

/// 音楽情報のファイルに関するエラー
#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum MusicFileError {
    /// 無効なパス
    #[error("Invalid path: {path}, {msg}")]
    InvalidPath {
        path: crate::util::FilePath,
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
        file_path: crate::util::FilePath,
    },
    /// ファイルを作成する際にエラーが発生
    #[error("Failed to create file {path}: {msg}")]
    FileCreate { path: String, msg: String },
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
        path: crate::util::FilePath,
        msg: String,
    },
    /// ディレクトリの読み込みに失敗
    #[error("Failed to read directory {dir}: {msg}")]
    ReadDir {
        dir: crate::util::DirPath,
        msg: String,
    },
    /// ファイルの内容のデシリアライズに失敗
    #[error("Failed to deserialize file {path}: {msg}")]
    Deserialize {
        path: crate::util::FilePath,
        msg: String,
    },
    /// 実装上のエラー
    #[error("Implementation error: {msg}")]
    ImplementationErr { msg: String },
}

impl MusicFileErrors {
    /// エラーメッセージを整形して返す
    pub fn to_pretty_string(&self) -> String {
        self.errs
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl MusicFileError {
    pub fn into_errors(self) -> MusicFileErrors {
        MusicFileErrors { errs: vec![self] }
    }

    /// エラーメッセージを整形して返す
    pub fn to_pretty_string(&self) -> String {
        self.to_string()
    }
}

impl From<Vec<MusicFileError>> for MusicFileErrors {
    fn from(errs: Vec<MusicFileError>) -> Self {
        Self { errs }
    }
}
