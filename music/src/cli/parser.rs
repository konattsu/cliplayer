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
    /// Generate minimized public data files from the current music library.
    Minify(MinifyCommands),
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
    #[command(flatten)]
    pub(crate) input: crate::cli::InputFilesArgs,
    #[command(flatten)]
    pub(crate) markdown: crate::cli::MarkdownArgs,
}

#[derive(Debug, clap::Args)]
pub(crate) struct AddApplyArgs {
    #[command(flatten)]
    pub(crate) input: crate::cli::InputFilesArgs,
    #[command(flatten)]
    pub(crate) api_key: crate::cli::ApiKeyArgs,
    #[command(flatten)]
    pub(crate) duplicate_video_policy: crate::cli::DuplicateVideoPolicyArgs,
    #[command(flatten)]
    pub(crate) music_root: crate::cli::MusicRootArgs,
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
    #[command(flatten)]
    pub(crate) music_root: crate::cli::MusicRootArgs,
}

#[derive(Debug, clap::Args)]
pub(crate) struct UpdateApplyArgs {
    #[command(flatten)]
    pub(crate) music_root: crate::cli::MusicRootArgs,
}

// MARK: sync

#[derive(Debug, clap::Args)]
pub(crate) struct SyncCommands {
    #[command(flatten)]
    pub(crate) api_key: crate::cli::ApiKeyArgs,
    #[command(flatten)]
    pub(crate) music_root: crate::cli::MusicRootArgs,
}

// MARK: min

#[derive(Debug, clap::Args)]
pub(crate) struct MinifyCommands {
    #[command(flatten)]
    pub(crate) music_root: crate::cli::MusicRootArgs,
    #[command(flatten)]
    pub(crate) min_output: crate::cli::MinOutputArgs,
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
    #[command(flatten)]
    pub(crate) music_root: crate::cli::MusicRootArgs,
}

#[derive(Debug, clap::Args)]
pub(crate) struct MergeFilesArgs {
    #[command(flatten)]
    pub(crate) directories: crate::cli::MergeDirectoriesArgs,
    /// Remove source files after a successful merge
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub(crate) remove_source_files: bool,
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
