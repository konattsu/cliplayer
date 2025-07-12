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

        #[clap(flatten)]
        common_opts: TraceLevel,
    },
    /// Update existing music data from input files
    Update {
        /// Comma-separated file(s) containing existing music data to update
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

        #[clap(flatten)]
        common_opts: TraceLevel,
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

        #[clap(flatten)]
        level: TraceLevel,
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
        common_opts: TraceLevel,
    },
    /// Validate existing music data input files
    UpdateInput {
        /// Comma-separated file(s) containing existing music data to validate
        #[arg(short, long, value_name = "FILES")]
        input: crate::cli::FilePathsFromCli,

        #[clap(flatten)]
        common_opts: TraceLevel,
    },
    /// Check for duplicate video IDs in the input
    Duplicate {
        /// Comma-separated video IDs to check for duplicates
        #[arg(short, long, value_name = "String")]
        input: crate::cli::VideoIdsFromCli,
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
