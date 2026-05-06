mod file_paths;
mod shared_args;
mod tracing_level;
mod video_ids;

pub(super) mod parser;

pub use file_paths::FilePathsFromCli;
pub use parser::Cli;
pub(crate) use parser::Commands;
pub(crate) use shared_args::{
    ApiKeyArgs, DuplicateVideoPolicyArgs, InputFilesArgs, MarkdownArgs,
    MergeDirectoriesArgs, MinOutputArgs, MusicRootArgs,
};
pub use tracing_level::TracingLevel;
pub use video_ids::VideoIdsFromCli;
