mod root;
mod util;

pub use root::MusicRoot;
pub use util::{
    FileVideo, ValidateError, ValidateErrorDeserialize, ValidateErrorFileOpen,
    deserialize_from_file, get_videos_list_from_music_root,
};
