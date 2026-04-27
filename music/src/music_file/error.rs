/// `MusicFileError`をまとめたもの
#[derive(Debug, Clone)]
pub struct MusicFileErrors {
    pub errs: Vec<MusicFileError>,
}

impl std::error::Error for MusicFileErrors {}

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
    /// 既存ファイルと同じ年月を持つ別ファイルが見つかった
    #[error(
        "Duplicate monthly file detected for ({year}/{month}): \
        existing={existing_path}, duplicated={duplicated_path}"
    )]
    DuplicateYearMonthFile {
        year: usize,
        month: usize,
        existing_path: std::path::PathBuf,
        duplicated_path: std::path::PathBuf,
    },
    /// 同一ファイル内に重複した動画IDを追加しようとした
    #[error("Duplicate video_id `{id}` found in file {file_path}")]
    DuplicateVideoId {
        id: crate::model::VideoId,
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
