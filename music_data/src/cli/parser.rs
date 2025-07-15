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
}

// 入力値は形式が正しければ成功とみなす. 例えば指定されたパスが存在しないなら後続の処理でエラーを出す

#[derive(Debug, clap::Subcommand)]
pub enum ApplyCommands {
    /// Apply new music data from input files
    New {
        /// Comma-separated file(s) containing new music data to apply
        #[arg(short, long, value_name = "FILES")]
        input: crate::cli::FilePathsFromCli,
        /// The key of YouTube Data v3 api to fetch data
        #[arg(short, long, env = "YOUTUBE_API_KEY")]
        api_key: crate::fetcher::YouTubeApiKey,
        /// Directory where the results will be written
        #[arg(long, value_name = "DIR", default_value_t = crate::cli::MusicRootFromCli::default())]
        music_root: crate::cli::MusicRootFromCli,
        /// Path to the output file for minimized JSON data
        #[arg(long, value_name = "FILE", default_value_t = crate::cli::OutputMinPathFromCli::default())]
        output_min_file: crate::cli::OutputMinPathFromCli,
        /// Path to the output file for minimized clips JSON data
        #[arg(long, value_name = "FILE", default_value_t = crate::cli::OutputMinClipsPathFromCli::default())]
        output_min_clips_file: crate::cli::OutputMinClipsPathFromCli,

        #[clap(flatten)]
        trace_level: TraceLevel,
    },
    /// Update existing music data from input files
    Update {
        /// Directory where the results will be written
        #[arg(long, value_name = "DIR", default_value_t = crate::cli::MusicRootFromCli::default())]
        music_root: crate::cli::MusicRootFromCli,
        /// Path to the output file for minimized JSON data
        #[arg(long, value_name = "FILE", default_value_t = crate::cli::OutputMinPathFromCli::default())]
        output_min_file: crate::cli::OutputMinPathFromCli,
        /// Path to the output file for minimized clips JSON data
        #[arg(long, value_name = "FILE", default_value_t = crate::cli::OutputMinClipsPathFromCli::default())]
        output_min_clips_file: crate::cli::OutputMinClipsPathFromCli,

        #[clap(flatten)]
        trace_level: TraceLevel,
    },
    /// Synchronize music data with the existing music directory using the Web API
    Sync {
        /// The key of YouTube Data v3 api to fetch data
        #[arg(short, long, env = "YOUTUBE_API_KEY")]
        api_key: crate::fetcher::YouTubeApiKey,
        /// Directory of the music data to synchronize with
        #[arg(long, value_name = "DIR", default_value_t = crate::cli::MusicRootFromCli::default())]
        music_root: crate::cli::MusicRootFromCli,
        /// Path to the output file for minimized JSON data
        #[arg( long, value_name = "FILE", default_value_t = crate::cli::OutputMinPathFromCli::default())]
        output_min_file: crate::cli::OutputMinPathFromCli,
        /// Path to the output file for minimized clips JSON data
        #[arg(long, value_name = "FILE", default_value_t = crate::cli::OutputMinClipsPathFromCli::default())]
        output_min_clips_file: crate::cli::OutputMinClipsPathFromCli,

        #[clap(flatten)]
        trace_level: TraceLevel,
    },
}

// あくまで`validate`ってことに注意しながら実装

#[derive(Debug, clap::Subcommand)]
pub enum ValidateCommands {
    /// Validate new music data input files
    NewInput {
        /// Comma-separated file(s) containing new music data to validate
        #[arg(short, long, value_name = "FILES")]
        input: crate::cli::FilePathsFromCli,

        #[clap(flatten)]
        trace_level: TraceLevel,
    },
    /// Validate existing music data input files
    UpdateInput {
        /// Directory of the music data to use for validation
        #[arg(long, value_name = "DIR", default_value_t = crate::cli::MusicRootFromCli::default())]
        music_root: crate::cli::MusicRootFromCli,

        #[clap(flatten)]
        trace_level: TraceLevel,
    },
    /// Check for duplicate video IDs in the input
    Duplicate {
        /// Comma-separated video IDs to check for duplicates
        #[arg(short, long, value_name = "String")]
        id: crate::cli::VideoIdsFromCli,
        /// Directory of the music data to use for duplicate checking
        #[arg(long, value_name = "DIR", default_value_t = crate::cli::MusicRootFromCli::default())]
        music_root: crate::cli::MusicRootFromCli,

        #[clap(flatten)]
        trace_level: TraceLevel,
    },
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
                ValidateCommands::NewInput { trace_level, .. } => {
                    &trace_level.file_tracing_level
                }
                ValidateCommands::UpdateInput { trace_level, .. } => {
                    &trace_level.file_tracing_level
                }
                ValidateCommands::Duplicate { trace_level, .. } => {
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
                ValidateCommands::NewInput { trace_level, .. } => {
                    &trace_level.stdout_tracing_level
                }
                ValidateCommands::UpdateInput { trace_level, .. } => {
                    &trace_level.stdout_tracing_level
                }
                ValidateCommands::Duplicate { trace_level, .. } => {
                    &trace_level.stdout_tracing_level
                }
            },
        };
        level.clone().map(|lv| lv.into_tracing_level_filter())
    }
}
