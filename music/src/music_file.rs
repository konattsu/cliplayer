mod error;
mod file;
pub(crate) mod fs_util;
mod library;
mod repository;

pub(super) mod videos;

pub use error::{MusicFileError, MusicFileErrors};
pub(crate) use file::MusicFile;
pub(crate) use videos::DuplicateVideoPolicy;

pub use library::MusicLibrary;
pub use repository::MusicLibraryRepository;
