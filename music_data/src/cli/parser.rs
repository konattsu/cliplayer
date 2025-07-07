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

#[derive(Debug, clap::Subcommand)]
pub enum ApplyCommands {
    New {
        /// Glob pattern for the input file(s) with new data to apply
        #[arg(short, long, value_name = "GLOB")]
        input: crate::cli::GlobPattern,
        #[clap(flatten)]
        common_opts: CommonOpts,
    },
    Update {
        /// Glob pattern for the input file(s) with existing data to update
        #[arg(short, long, value_name = "GLOB")]
        input: crate::cli::GlobPattern,
        #[clap(flatten)]
        common_opts: CommonOpts,
    },
}

#[derive(Debug, clap::Subcommand)]
pub enum ValidateCommands {
    NewInput {
        /// Glob pattern for the input file(s) with new data to validate
        #[arg(short, long, value_name = "GLOB")]
        input: crate::cli::GlobPattern,
        #[clap(flatten)]
        common_opts: CommonOpts,
    },
    UpdateInput {
        /// Glob pattern for the input file(s) with existing data to validate
        #[arg(short, long, value_name = "GLOB")]
        input: crate::cli::GlobPattern,
        #[clap(flatten)]
        common_opts: CommonOpts,
    },
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
