pub(super) mod fs;
pub(super) mod validate;

mod model;

pub use model::{
    MusicFileError, MusicFileErrors, MusicFilePath, MusicFilePathContent, MusicRoot,
    MusicRootContent,
};
