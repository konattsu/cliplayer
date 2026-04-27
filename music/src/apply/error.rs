#[derive(thiserror::Error, Debug)]
pub enum ApplyError {
    #[error("YouTube API request failed: {0}")]
    YouTubeApi(#[from] crate::fetcher::YouTubeApiError),
    #[error("Failed to verify video data: {0}")]
    VerifyVideos(#[from] crate::model::VerifiedVideoErrors),
    #[error("Music file operation failed: {0}")]
    MusicFile(#[from] crate::music_file::MusicFileError),
    #[error("Music file operations failed: {0}")]
    MusicFiles(#[from] crate::music_file::MusicFileErrors),
    #[error("Some files failed during sync:\n{0}")]
    SyncPartialFailure(String),
}
