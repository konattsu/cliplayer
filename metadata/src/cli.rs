#[derive(Debug, clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,

    #[command(flatten)]
    pub(crate) trace_level: cmn_rs::tracing::CliTraceOps,
}

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    /// Generate artist-related metadata.
    Artist(ArtistArgs),
    /// Generate video-tag-related metadata.
    Tag(TagArgs),
}

#[derive(Debug, clap::Args)]
pub struct ArtistArgs {
    /// Directory for minified data output
    #[arg(long, default_value_t = default_artist_output_dir())]
    pub(crate) output_dir: String,
    /// File name for the search index output
    #[arg(long, default_value_t = default_min_livers_search_index_file_name())]
    pub(crate) min_livers_search_index_file_name: String,
    /// File name for the channels info output
    #[arg(long, default_value_t = default_min_channels_file_name())]
    pub(crate) min_channels_file_name: String,
    /// File name for the artists info output
    #[arg(long, default_value_t = default_min_livers_file_name())]
    pub(crate) min_livers_file_name: String,
    /// File name for the official channels info output
    #[arg(long, default_value_t = default_min_official_channels_file_name())]
    pub(crate) min_official_channels_file_name: String,
    /// Path to the VS Code code-snippets file to update
    #[arg(long, default_value_t = default_music_code_snippets_path())]
    pub(crate) music_code_snippets_path: String,
}

#[derive(Debug, clap::Args)]
pub struct TagArgs {
    /// Path to the VS Code code-snippets file to update
    #[arg(long, default_value_t = default_tag_code_snippets_path())]
    pub(crate) code_snippets_path: String,
}

fn default_artist_output_dir() -> String {
    "src/music".to_string()
}

fn default_min_livers_search_index_file_name() -> String {
    "livers_search_index.min.json".to_string()
}

fn default_min_channels_file_name() -> String {
    "channels.min.json".to_string()
}

fn default_min_livers_file_name() -> String {
    "livers.min.json".to_string()
}

fn default_min_official_channels_file_name() -> String {
    "official_channels.min.json".to_string()
}

fn default_music_code_snippets_path() -> String {
    ".vscode/music.code-snippets".to_string()
}

fn default_tag_code_snippets_path() -> String {
    ".vscode/tags.code-snippets".to_string()
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
