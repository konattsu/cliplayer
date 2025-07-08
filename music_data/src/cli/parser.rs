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
        /// Glob pattern for the input file(s) containing new music data to apply
        #[arg(short, long, value_name = "GLOB")]
        input: crate::cli::GlobPattern,
        #[clap(flatten)]
        common_opts: CommonOpts,
        /// Directory where the results will be written
        #[arg(long, value_name = "DIR", default_value_t = output_dir_default())]
        output_dir: String,
        /// Path to the output file for minimized JSON data
        #[arg(
            long,
            value_name = "FILE",
            default_value_t = output_min_file_default(),
        )]
        output_min_file: String,
    },
    /// Update existing music data from input files
    Update {
        /// Glob pattern for the input file(s) containing existing music data to update
        #[arg(short, long, value_name = "GLOB")]
        input: crate::cli::GlobPattern,
        #[clap(flatten)]
        common_opts: CommonOpts,
        /// Directory where the results will be written
        #[arg(long, value_name = "DIR", default_value_t = output_dir_default())]
        output_dir: String,
        /// Path to the output file for minimized JSON data
        #[arg(
            long,
            value_name = "FILE",
            default_value_t = output_min_file_default(),
        )]
        output_min_file: String,
    },
    /// Synchronize music data with the existing music directory using the Web API
    Sync {
        /// Glob pattern for the input file(s) to synchronize
        #[arg(short, long, value_name = "GLOB")]
        input: crate::cli::GlobPattern,
        #[clap(flatten)]
        common_opts: CommonOpts,
        /// Directory of the music data to synchronize with
        #[arg(long, value_name = "DIR", default_value_t = output_dir_default())]
        music_dir: String,
        /// Path to the output file for minimized JSON data
        #[arg(
            long,
            value_name = "FILE",
            default_value_t = output_min_file_default(),
        )]
        output_min_file: String,
    },
}

// あくまで`validate`ってことに注意しながら実装

#[derive(Debug, clap::Subcommand)]
pub enum ValidateCommands {
    /// Validate new music data input files
    NewInput {
        /// Glob pattern for the input file(s) containing new music data to validate
        #[arg(short, long, value_name = "GLOB")]
        input: crate::cli::GlobPattern,
        #[clap(flatten)]
        common_opts: CommonOpts,
    },
    /// Validate existing music data input files
    UpdateInput {
        /// Glob pattern for the input file(s) containing existing music data to validate
        #[arg(short, long, value_name = "GLOB")]
        input: crate::cli::GlobPattern,
        #[clap(flatten)]
        common_opts: CommonOpts,
    },
    /// Check for duplicate video IDs in the input
    Duplicate {
        /// Comma-separated video IDs to check for duplicates
        #[arg(short, long, value_name = "String")]
        input: crate::cli::VideoIds,
        #[clap(flatten)]
        common_opts: CommonOpts,
    },
}

#[derive(Debug, clap::Args)]
pub struct CommonOpts {
    /// Tracing level for file operations
    #[arg(long, value_name = "LEVEL")]
    pub file_tracing_level: Option<crate::cli::TracingLevel>,
    /// Tracing level for stdout output
    #[arg(long, value_name = "LEVEL")]
    pub stdout_tracing_level: Option<crate::cli::TracingLevel>,
}

fn output_min_file_default() -> String {
    "../public/music_data/music.min.json".to_string()
}

fn output_dir_default() -> String {
    "./data/music/".to_string()
}
