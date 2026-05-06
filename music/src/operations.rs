mod find_duplicate_video_ids;
mod merge_input_files;

pub(crate) use find_duplicate_video_ids::find_duplicate_video_ids;
pub(crate) use merge_input_files::merge_input_files;

#[derive(thiserror::Error, Debug)]
pub enum OperationError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    AnonymousVideoValidation(#[from] crate::validate::AnonymousVideoValidateErrors),
}
