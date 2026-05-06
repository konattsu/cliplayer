#[derive(Debug, clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    pub(crate) build: BuildArgs,

    #[command(flatten)]
    pub(crate) trace_level: cmn_rs::tracing::CliTraceOps,
}

#[derive(Debug, clap::Args)]
pub(crate) struct BuildArgs {
    /// Directory of the music data used to build the search index.
    #[arg(long, value_name = "DIR", default_value_t = musictl::cfg::default_music_root_dir())]
    pub(crate) music_root_dir: String,
    /// Path to the output binary search index file.
    #[arg(long, value_name = "FILE", default_value_t = default_output_path())]
    pub(crate) output_path: String,
}

fn default_output_path() -> String {
    "public/search/search_index.bin".to_string()
}

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
