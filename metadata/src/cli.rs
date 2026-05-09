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
    Artist(ArtistCommand),
    /// Generate video-tag-related metadata.
    Tag(TagCommand),
}

#[derive(Debug, clap::Args)]
pub struct ArtistCommand {
    #[command(subcommand)]
    pub(crate) mode: ArtistMode,
}

#[derive(Debug, clap::Subcommand)]
pub enum ArtistMode {
    /// Generate minified JSON for the frontend.
    Minify(ArtistMinifyArgs),
    /// Generate/update VS Code snippets.
    Snippet(ArtistSnippetArgs),
    /// Hash the source input set used for artist-derived artifacts.
    HashInputs,
}

#[derive(Debug, clap::Args)]
pub struct ArtistMinifyArgs {
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
    /// Opaque data build ID shared by every generated min JSON in the same build
    #[arg(long, value_name = "ID")]
    pub(crate) dataset_build_id: cmn_rs::min_json::DatasetBuildId,
}

#[derive(Debug, clap::Args)]
pub struct ArtistSnippetArgs {
    /// Path to the VS Code code-snippets file to update
    #[arg(long, default_value_t = default_music_code_snippets_path())]
    pub(crate) music_code_snippets_path: String,
}

#[derive(Debug, clap::Args)]
pub struct TagCommand {
    #[command(subcommand)]
    pub(crate) mode: TagMode,
}

#[derive(Debug, clap::Subcommand)]
pub enum TagMode {
    /// Generate minified JSON for the frontend.
    Minify(TagMinifyArgs),
    /// Generate/update VS Code snippets.
    Snippet(TagSnippetArgs),
    /// Hash the source input set used for tag-derived artifacts.
    HashInputs,
}

#[derive(Debug, clap::Args)]
pub struct TagMinifyArgs {
    /// Directory for minified data output
    #[arg(long, default_value_t = default_tag_output_dir())]
    pub(crate) output_dir: String,
    /// File name for the minified tags output
    #[arg(long, default_value_t = default_min_tags_file_name())]
    pub(crate) min_tags_file_name: String,
    /// Opaque data build ID shared by every generated min JSON in the same build
    #[arg(long, value_name = "ID")]
    pub(crate) dataset_build_id: cmn_rs::min_json::DatasetBuildId,
}

#[derive(Debug, clap::Args)]
pub struct TagSnippetArgs {
    /// Path to the VS Code code-snippets file to update
    #[arg(long, default_value_t = default_tag_code_snippets_path())]
    pub(crate) code_snippets_path: String,
}

fn default_artist_output_dir() -> String {
    "public/music".to_string()
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

fn default_tag_output_dir() -> String {
    "public/music".to_string()
}

fn default_min_tags_file_name() -> String {
    "tags.min.json".to_string()
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
