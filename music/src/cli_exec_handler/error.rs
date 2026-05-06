#[derive(thiserror::Error, Debug)]
pub enum CliExecError {
    #[error(transparent)]
    Apply(#[from] crate::apply::ApplyError),
    #[error(transparent)]
    MusicFile(#[from] crate::music_file::MusicFileErrors),
    #[error(transparent)]
    AnonymousVideoValidation(#[from] crate::validate::AnonymousVideoValidateErrors),
    #[error(transparent)]
    Operation(#[from] crate::operations::OperationError),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    Message(String),
}
