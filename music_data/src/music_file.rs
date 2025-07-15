pub(super) mod fs_util;
pub(super) mod validate;

mod error;
mod root;

pub use error::{MusicFileError, MusicFileErrors};
pub use root::{MusicFilePath, MusicFilePathContent, MusicRoot, MusicRootContent};
