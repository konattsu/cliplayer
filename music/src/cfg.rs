pub const DEFAULT_MUSIC_ROOT_DIR: &str = "music/data/music";
pub const DEFAULT_MIN_OUTPUT_CLIPS_PATH: &str = "public/music/clips.min.json";
pub const DEFAULT_MIN_OUTPUT_VIDEOS_PATH: &str = "public/music/videos.min.json";
pub const DEFAULT_MERGE_FILES_INPUT_DIR: &str = "./music/data/input/";
pub const DEFAULT_MERGED_FILE_OUTPUT_DIR: &str = "./music/data/input/";

pub fn default_music_root_dir() -> String {
    DEFAULT_MUSIC_ROOT_DIR.to_string()
}

pub fn default_min_output_clips_path() -> String {
    DEFAULT_MIN_OUTPUT_CLIPS_PATH.to_string()
}

pub fn default_min_output_videos_path() -> String {
    DEFAULT_MIN_OUTPUT_VIDEOS_PATH.to_string()
}

pub fn default_merge_files_dir() -> String {
    DEFAULT_MERGE_FILES_INPUT_DIR.to_string()
}

pub fn default_merged_file_dir() -> String {
    DEFAULT_MERGED_FILE_OUTPUT_DIR.to_string()
}
