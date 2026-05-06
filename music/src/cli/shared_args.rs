#[derive(Debug, Clone, clap::Args)]
pub(crate) struct InputFilesArgs {
    /// Comma-separated file paths containing input music data
    #[arg(short, long, value_name = "FILES")]
    pub(crate) input: crate::cli::FilePathsFromCli,
}

impl InputFilesArgs {
    pub(crate) fn into_file_paths(self) -> Vec<std::path::PathBuf> {
        self.input.into_file_paths()
    }
}

#[derive(Debug, Clone, clap::Args)]
pub(crate) struct MusicRootArgs {
    /// Directory of the music data to use
    #[arg(long, value_name = "DIR", default_value = crate::cfg::DEFAULT_MUSIC_ROOT_DIR)]
    pub(crate) music_root_dir: std::path::PathBuf,
}

#[derive(Debug, Clone, clap::Args)]
pub(crate) struct MinOutputArgs {
    /// Path to the output file for minimized clips data
    #[arg(long, value_name = "FILE", default_value = crate::cfg::DEFAULT_MIN_OUTPUT_CLIPS_PATH)]
    pub(crate) min_clips_path: std::path::PathBuf,
    /// Path to the output file for minimized videos data
    #[arg(long, value_name = "FILE", default_value = crate::cfg::DEFAULT_MIN_OUTPUT_VIDEOS_PATH)]
    pub(crate) min_videos_path: std::path::PathBuf,
}

#[derive(Debug, Clone, clap::Args)]
pub(crate) struct ApiKeyArgs {
    /// The key of YouTube Data v3 api to fetch data
    #[arg(short, long, env = "YOUTUBE_API_KEY", hide_env_values = true)]
    pub(crate) api_key: crate::fetcher::YouTubeApiKey,
}

#[derive(Debug, Clone, clap::Args)]
pub(crate) struct MarkdownArgs {
    /// If set, output the parsed information in markdown format
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub(crate) markdown: bool,
}

#[derive(Debug, Clone, clap::Args)]
pub(crate) struct DuplicateVideoPolicyArgs {
    /// If set, duplicate video IDs in existing month files are overwritten
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub(crate) allow_overwrite_existing_video: bool,
}

impl DuplicateVideoPolicyArgs {
    pub(crate) fn duplicate_video_policy(
        &self,
    ) -> crate::music_file::DuplicateVideoPolicy {
        if self.allow_overwrite_existing_video {
            crate::music_file::DuplicateVideoPolicy::Overwrite
        } else {
            crate::music_file::DuplicateVideoPolicy::Reject
        }
    }
}

#[derive(Debug, Clone, clap::Args)]
pub(crate) struct MergeDirectoriesArgs {
    /// Directory containing the json files to be merged
    #[arg(short, long, value_name = "DIR", default_value = crate::cfg::DEFAULT_MERGE_FILES_INPUT_DIR)]
    pub(crate) input_dir: std::path::PathBuf,
    /// Directory where the merged json file will be written
    #[arg(short, long, value_name = "DIR", default_value = crate::cfg::DEFAULT_MERGED_FILE_OUTPUT_DIR)]
    pub(crate) output_dir: std::path::PathBuf,
}
