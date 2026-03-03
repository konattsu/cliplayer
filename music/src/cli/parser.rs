mod cli_default_vals {
    pub(super) const MUSIC_ROOT_DIR: &str = "music/data/music";
    pub(super) const MIN_OUTPUT_CLIPS_PATH: &str = "public/music/clips.min.json";
    pub(super) const MIN_OUTPUT_VIDEOS_PATH: &str = "public/music/videos.min.json";
    pub(super) const MERGE_FILES_INPUT_DIR: &str = "./music/data/input/";
    pub(super) const MERGED_FILE_OUTPUT_DIR: &str = "./music/data/input/";

    pub(super) fn default_music_root_dir() -> String {
        MUSIC_ROOT_DIR.to_string()
    }
    pub(super) fn default_min_output_clips_path() -> String {
        MIN_OUTPUT_CLIPS_PATH.to_string()
    }
    pub(super) fn default_min_output_videos_path() -> String {
        MIN_OUTPUT_VIDEOS_PATH.to_string()
    }
    pub(super) fn default_merge_files_dir() -> String {
        MERGE_FILES_INPUT_DIR.to_string()
    }
    pub(super) fn default_merged_file_dir() -> String {
        MERGED_FILE_OUTPUT_DIR.to_string()
    }
}

#[derive(Debug, clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,

    #[clap(flatten)]
    pub(crate) trace_level: cmn_rs::tracing::CliTraceOps,
}

#[derive(Debug, clap::Subcommand)]
pub(crate) enum Commands {
    /// Add new music entries to the library using one or more input files.
    Add(AddCommands),
    /// Update the existing music library based on its current contents.
    Update(UpdateCommands),
    /// Synchronize the library with YouTube using the current library state.
    Sync(SyncCommands),
    /// Run utility commands that are outside the core music‑library workflows.
    Util(UtilCommands),
}

// 入力値は形式が正しければ成功とみなす. 例えば指定されたパスが存在しないなら後続の処理でエラーを出す

// MARK: add

#[derive(Debug, clap::Args)]
pub(crate) struct AddCommands {
    #[command(subcommand)]
    pub(crate) mode: AddMode,
}

#[derive(Debug, clap::Subcommand)]
pub(crate) enum AddMode {
    /// Validate the input files without applying changes to the music library
    Validate(AddValidateArgs),
    /// Apply changes to the music library based on the input files
    Apply(AddApplyArgs),
}

#[derive(Debug, clap::Args)]
pub(crate) struct AddValidateArgs {
    /// Comma-separated file paths containing new music data to validate
    #[arg(short, long, value_name = "FILES")]
    pub(crate) input: crate::cli::FilePathsFromCli,
    /// If set, output the parsed information in markdown format
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub(crate) markdown: bool,
}

#[derive(Debug, clap::Args)]
pub(crate) struct AddApplyArgs {
    /// Comma-separated file paths containing new music data to apply
    #[arg(short, long, value_name = "FILES")]
    pub(crate) input: crate::cli::FilePathsFromCli,
    /// The key of YouTube Data v3 api to fetch data
    #[arg(short, long, env = "YOUTUBE_API_KEY", hide_env_values = true)]
    pub(crate) api_key: crate::fetcher::YouTubeApiKey,
    /// Directory where the results will be written
    #[arg(long, value_name = "DIR", default_value_t = cli_default_vals::default_music_root_dir())]
    pub(crate) music_root_dir: String,
    // Path to the output file for minimized clips data
    #[arg(long, value_name = "FILE", default_value_t = cli_default_vals::default_min_output_clips_path())]
    /// Path to the output file for minimized videos data
    pub(crate) min_clips_path: String,
    // Path to the output file for minimized videos data
    #[arg(long, value_name = "FILE", default_value_t = cli_default_vals::default_min_output_videos_path())]
    pub(crate) min_videos_path: String,
}

// MARK: update

#[derive(Debug, clap::Args)]
pub(crate) struct UpdateCommands {
    #[command(subcommand)]
    pub(crate) mode: UpdateMode,
}

#[derive(Debug, clap::Subcommand)]
pub(crate) enum UpdateMode {
    /// Validate the existing music library data without applying changes
    Validate(UpdateValidateArgs),
    /// Apply changes to the existing music library data
    Apply(UpdateApplyArgs),
}

#[derive(Debug, clap::Args)]
pub(crate) struct UpdateValidateArgs {
    /// Directory of the music data to use for validation
    #[arg(long, value_name = "DIR", default_value_t = cli_default_vals::default_music_root_dir())]
    pub(crate) music_root_dir: String,
}

#[derive(Debug, clap::Args)]
pub(crate) struct UpdateApplyArgs {
    /// Directory where the results will be written
    #[arg(long, value_name = "DIR", default_value_t = cli_default_vals::default_music_root_dir())]
    pub(crate) music_root_dir: String,
    /// Path to the output file for minimized clips data
    #[arg(long, value_name = "FILE", default_value_t = cli_default_vals::default_min_output_clips_path())]
    pub(crate) min_clips_path: String,
    /// Path to the output file for minimized videos data
    #[arg(long, value_name = "FILE", default_value_t = cli_default_vals::default_min_output_videos_path())]
    pub(crate) min_videos_path: String,
}

// MARK: sync

#[derive(Debug, clap::Args)]
pub(crate) struct SyncCommands {
    /// The key of YouTube Data v3 api to fetch data
    #[arg(short, long, env = "YOUTUBE_API_KEY", hide_env_values = true)]
    pub(crate) api_key: crate::fetcher::YouTubeApiKey,
    /// Directory of the music data to synchronize with
    #[arg(long, value_name = "DIR", default_value_t = cli_default_vals::default_music_root_dir())]
    pub(crate) music_root_dir: String,
    /// Path to the output file for minimized clips data
    #[arg(long, value_name = "FILE", default_value_t = cli_default_vals::default_min_output_clips_path())]
    pub(crate) min_clips_path: String,
    /// Path to the output file for minimized videos data
    #[arg(long, value_name = "FILE", default_value_t = cli_default_vals::default_min_output_videos_path())]
    pub(crate) min_videos_path: String,
}

// MARK: util

#[derive(Debug, clap::Args)]
pub(crate) struct UtilCommands {
    #[command(subcommand)]
    pub(crate) mode: UtilMode,
}

#[derive(Debug, clap::Subcommand)]
pub(crate) enum UtilMode {
    /// Merge multiple files containing input music data into a single file
    Merge(MergeFilesArgs),
    /// Check for duplicate video IDs in the input
    Find(FindDuplicateIdsArgs),
}

#[derive(Debug, clap::Args)]
pub(crate) struct FindDuplicateIdsArgs {
    /// Comma-separated video IDs to check for duplicates
    #[arg(short, long, value_name = "String")]
    pub(crate) ids: crate::cli::VideoIdsFromCli,
    /// Directory of the music data to use for duplicate checking
    #[arg(long, value_name = "DIR", default_value_t = cli_default_vals::default_music_root_dir())]
    pub(crate) music_root_dir: String,
}

#[derive(Debug, clap::Args)]
pub(crate) struct MergeFilesArgs {
    /// Directory containing the json files to be merged
    #[arg(short, long, value_name = "DIR", default_value_t = cli_default_vals::default_merge_files_dir())]
    pub(crate) input_dir: String,
    /// Directory where the json files will be merged
    #[arg(short, long, value_name = "DIR", default_value_t = cli_default_vals::default_merged_file_dir())]
    pub(crate) output_dir: String,
}

// MARK: impl

impl Cli {
    pub fn file_level(&self) -> Option<tracing::level_filters::LevelFilter> {
        self.trace_level
            .file_tracing_level
            .map(|lv| lv.into_tracing_level_filter())
    }

    pub fn stdout_level(&self) -> Option<tracing::level_filters::LevelFilter> {
        Some(
            self.trace_level
                .stdout_tracing_level
                .into_tracing_level_filter(),
        )
    }

    pub fn is_quiet(&self) -> bool {
        self.trace_level.quiet
    }
}
