mod root;
mod validate;

pub use root::{MusicFilePath, MusicRoot};
pub use validate::{
    FileVideo, ValidateError, ValidateErrorDeserialize, ValidateErrorFileOpen,
    deserialize_from_file, get_videos_list_from_music_root,
};
