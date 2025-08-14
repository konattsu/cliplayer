#[derive(Debug, clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    Apply(ApplyCommands),
    #[command(subcommand)]
    Validate(ValidateCommands),
    #[command(subcommand)]
    Dev(UtilCommands),
}

// 入力値は形式が正しければ成功とみなす. 例えば指定されたパスが存在しないなら後続の処理でエラーを出す

#[derive(Debug, clap::Subcommand)]
pub enum ApplyCommands {
    /// Apply new music data from input files
    New {
        /// Comma-separated file paths containing new music data to apply
        #[arg(short, long, value_name = "FILES")]
        input: crate::cli::FilePathsFromCli,
        /// The key of YouTube Data v3 api to fetch data
        #[arg(short, long, env = "YOUTUBE_API_KEY", hide_env_values = true)]
        api_key: crate::fetcher::YouTubeApiKey,
        /// Directory where the results will be written
        #[arg(long, value_name = "DIR", default_value_t = crate::cli::MusicLibraryCli::default())]
        music_root: crate::cli::MusicLibraryCli,
        /// Path to the output file for minimized JSON data
        #[arg(long, value_name = "FILE", default_value_t = default_output_min_file())]
        output_min_file: String,
        /// Path to the output file for minimized clips JSON data
        #[arg(long, value_name = "FILE", default_value_t = default_output_min_clips_file())]
        output_min_clips_file: String,

        #[clap(flatten)]
        trace_level: TraceLevel,
    },
    /// Update existing music data from input files
    Update {
        /// Directory where the results will be written
        #[arg(long, value_name = "DIR", default_value_t = crate::cli::MusicLibraryCli::default())]
        music_root: crate::cli::MusicLibraryCli,
        /// Path to the output file for minimized JSON data
        #[arg(long, value_name = "FILE", default_value_t = default_output_min_file())]
        output_min_file: String,
        /// Path to the output file for minimized clips JSON data
        #[arg(long, value_name = "FILE", default_value_t = default_output_min_clips_file())]
        output_min_clips_file: String,

        #[clap(flatten)]
        trace_level: TraceLevel,
    },
    /// Synchronize music data with the existing music directory using the Web API
    Sync {
        /// The key of YouTube Data v3 api to fetch data
        #[arg(short, long, env = "YOUTUBE_API_KEY", hide_env_values = true)]
        api_key: crate::fetcher::YouTubeApiKey,
        /// Directory of the music data to synchronize with
        #[arg(long, value_name = "DIR", default_value_t = crate::cli::MusicLibraryCli::default())]
        music_root: crate::cli::MusicLibraryCli,
        /// Path to the output file for minimized JSON data
        #[arg( long, value_name = "FILE", default_value_t = default_output_min_file())]
        output_min_file: String,
        /// Path to the output file for minimized clips JSON data
        #[arg(long, value_name = "FILE", default_value_t = default_output_min_clips_file())]
        output_min_clips_file: String,

        #[clap(flatten)]
        trace_level: TraceLevel,
    },
}

#[derive(Debug, clap::Subcommand)]
pub enum ValidateCommands {
    /// Validate new music data input files
    New {
        /// Comma-separated file paths containing new music data to validate
        #[arg(short, long, value_name = "FILES")]
        input: crate::cli::FilePathsFromCli,

        #[clap(flatten)]
        trace_level: TraceLevel,
    },
    /// Validate existing music data input files
    Update {
        /// Directory of the music data to use for validation
        #[arg(long, value_name = "DIR", default_value_t = crate::cli::MusicLibraryCli::default())]
        music_root: crate::cli::MusicLibraryCli,
        /// Path to the output file for minimized JSON data
        #[arg(long, value_name = "FILE", default_value_t = default_output_min_file())]
        output_min_file: String,
        /// Path to the output file for minimized clips JSON data
        #[arg(long, value_name = "FILE", default_value_t = default_output_min_clips_file())]
        output_min_clips_file: String,

        #[clap(flatten)]
        trace_level: TraceLevel,
    },
    /// Validate new music data input files and output parsed info
    NewMd {
        /// Comma-separated file paths containing new music data to validate
        #[arg(short, long, value_name = "FILES")]
        input: crate::cli::FilePathsFromCli,

        #[clap(flatten)]
        trace_level: TraceLevel,
    },
}

#[derive(Debug, clap::Subcommand)]
pub enum UtilCommands {
    /// Generate artist-related data
    GenerateArtist {
        /// Path to the input artist data file
        #[arg(long, default_value_t = default_input_artists_data_path())]
        input_artists_data_path: String,
        /// Directory for artist output
        #[arg(long, default_value_t = default_artist_output_dir())]
        artist_output_dir: String,
        /// Path to the search index file
        #[arg(long, default_value_t = default_search_index_file_path())]
        search_index_file_name: String,
        /// Path to the channel info file
        #[arg(long, default_value_t = default_channel_file_path())]
        channel_file_name: String,
        /// Path to the artist info file
        #[arg(long, default_value_t = default_artists_file_path())]
        artists_file_name: String,
        #[arg(long, default_value_t = default_code_snippets_path())]
        music_data_code_snippets_path: String,

        #[clap(flatten)]
        trace_level: TraceLevel,
    },
    /// Check for duplicate video IDs in the input
    DuplicateIds {
        /// Comma-separated video IDs to check for duplicates
        #[arg(short, long, value_name = "String")]
        ids: crate::cli::VideoIdsFromCli,
        /// Directory of the music data to use for duplicate checking
        #[arg(long, value_name = "DIR", default_value_t = crate::cli::MusicLibraryCli::default())]
        music_lib: crate::cli::MusicLibraryCli,
        /// Path to the output file for minimized JSON data
        #[arg(long, value_name = "FILE", default_value_t = default_output_min_file())]
        output_min_file: String,
        /// Path to the output file for minimized clips JSON data
        #[arg(long, value_name = "FILE", default_value_t = default_output_min_clips_file())]
        output_min_clips_file: String,

        #[clap(flatten)]
        trace_level: TraceLevel,
    },
    /// Merge multiple files containing input music data into a single file
    MergeFiles {
        /// Comma-separated file paths containing new music data to merge
        #[arg(short, long, value_name = "FILES")]
        files: Option<crate::cli::FilePathsFromCli>,
        /// Directory where the json files will be merged
        #[arg(short, long, value_name = "DIR", default_value_t = default_merge_files_dir())]
        dir: String,
        /// Path to the output directory for merged data
        #[arg(short, long, value_name = "DIR", default_value_t = default_merged_dir())]
        output_dir: String,

        #[clap(flatten)]
        trace_level: TraceLevel,
    },
}

fn default_output_min_file() -> String {
    "../public/music_data/music.min.json".to_string()
}
fn default_output_min_clips_file() -> String {
    "../public/music_data/clips.min.json".to_string()
}
fn default_input_artists_data_path() -> String {
    "data/artists_data.json".to_string()
}
fn default_artist_output_dir() -> String {
    "../src/music_data/".to_string()
}
fn default_search_index_file_path() -> String {
    "artist_search_index.min.json".to_string()
}
fn default_channel_file_path() -> String {
    "channels.min.json".to_string()
}
fn default_artists_file_path() -> String {
    "artists.min.json".to_string()
}
fn default_code_snippets_path() -> String {
    "../.vscode/music_data.code-snippets".to_string()
}
fn default_merge_files_dir() -> String {
    "./data/input/".to_string()
}
fn default_merged_dir() -> String {
    "./data/input/".to_string()
}

#[derive(Debug, clap::Args)]
pub struct TraceLevel {
    /// Tracing level for file operations
    #[arg(long, value_name = "LEVEL")]
    pub file_tracing_level: Option<crate::cli::TracingLevel>,
    /// Tracing level for stdout output
    #[arg(long, value_name = "LEVEL")]
    pub stdout_tracing_level: Option<crate::cli::TracingLevel>,
}

impl Cli {
    pub fn file_level(&self) -> Option<tracing::level_filters::LevelFilter> {
        let level = match self.command {
            Commands::Apply(ref apply_cmd) => match apply_cmd {
                ApplyCommands::New { trace_level, .. } => {
                    &trace_level.file_tracing_level
                }
                ApplyCommands::Update { trace_level, .. } => {
                    &trace_level.file_tracing_level
                }
                ApplyCommands::Sync { trace_level, .. } => {
                    &trace_level.file_tracing_level
                }
            },
            Commands::Validate(ref validate_cmd) => match validate_cmd {
                ValidateCommands::New { trace_level, .. } => {
                    &trace_level.file_tracing_level
                }
                ValidateCommands::Update { trace_level, .. } => {
                    &trace_level.file_tracing_level
                }
                ValidateCommands::NewMd { trace_level, .. } => {
                    &trace_level.file_tracing_level
                }
            },
            Commands::Dev(ref util_cmd) => match util_cmd {
                UtilCommands::GenerateArtist { trace_level, .. } => {
                    &trace_level.file_tracing_level
                }
                UtilCommands::DuplicateIds { trace_level, .. } => {
                    &trace_level.file_tracing_level
                }
                UtilCommands::MergeFiles { trace_level, .. } => {
                    &trace_level.file_tracing_level
                }
            },
        };
        level.clone().map(|lv| lv.into_tracing_level_filter())
    }

    pub fn stdout_level(&self) -> Option<tracing::level_filters::LevelFilter> {
        let level = match self.command {
            Commands::Apply(ref apply_cmd) => match apply_cmd {
                ApplyCommands::New { trace_level, .. } => {
                    &trace_level.stdout_tracing_level
                }
                ApplyCommands::Update { trace_level, .. } => {
                    &trace_level.stdout_tracing_level
                }
                ApplyCommands::Sync { trace_level, .. } => {
                    &trace_level.stdout_tracing_level
                }
            },
            Commands::Validate(ref validate_cmd) => match validate_cmd {
                ValidateCommands::New { trace_level, .. } => {
                    &trace_level.stdout_tracing_level
                }
                ValidateCommands::Update { trace_level, .. } => {
                    &trace_level.stdout_tracing_level
                }
                ValidateCommands::NewMd { trace_level, .. } => {
                    &trace_level.stdout_tracing_level
                }
            },
            Commands::Dev(ref util_cmd) => match util_cmd {
                UtilCommands::GenerateArtist { trace_level, .. } => {
                    &trace_level.stdout_tracing_level
                }
                UtilCommands::DuplicateIds { trace_level, .. } => {
                    &trace_level.stdout_tracing_level
                }
                UtilCommands::MergeFiles { trace_level, .. } => {
                    &trace_level.stdout_tracing_level
                }
            },
        };
        level.clone().map(|lv| lv.into_tracing_level_filter())
    }
}
