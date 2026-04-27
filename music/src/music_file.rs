mod error;
mod file;
mod fs_util;
mod library;

pub(super) mod videos;

pub(crate) use error::{MusicFileError, MusicFileErrors};
pub(crate) use file::MusicFile;
pub(crate) use videos::DuplicateVideoPolicy;

pub use library::MusicLibrary;
