#[derive(Debug, clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    pub generate: GenerateArgs,

    #[command(flatten)]
    pub trace_level: cmn_rs::tracing::CliTraceLevel,
}

/// Generate artist-related data
#[derive(Debug, clap::Args)]
pub struct GenerateArgs {
    /// Path to the input artist data file
    #[arg(long, default_value_t = default_input_artists_data_path())]
    pub input_artists_data_path: String,
    /// Directory for artist output
    #[arg(long, default_value_t = default_artist_output_dir())]
    pub artist_output_dir: String,
    /// File name for the search index output
    #[arg(long, default_value_t = default_search_index_file_name())]
    pub search_index_file_name: String,
    /// File name for the channel info output
    #[arg(long, default_value_t = default_channel_file_name())]
    pub channel_file_name: String,
    /// File name for the artists info output
    #[arg(long, default_value_t = default_artists_file_name())]
    pub artists_file_name: String,
    /// Path to the VS Code code-snippets file to update
    #[arg(long, default_value_t = default_code_snippets_path())]
    pub music_data_code_snippets_path: String,
}

fn default_input_artists_data_path() -> String {
    "data/artists_data.json".to_string()
}
fn default_artist_output_dir() -> String {
    "../src/music/".to_string()
}
fn default_search_index_file_name() -> String {
    "artist_search_index.min.json".to_string()
}
fn default_channel_file_name() -> String {
    "channels.min.json".to_string()
}
fn default_artists_file_name() -> String {
    "artists.min.json".to_string()
}
fn default_code_snippets_path() -> String {
    "../.vscode/music.code-snippets".to_string()
}

impl Cli {
    pub fn file_level(&self) -> Option<tracing::level_filters::LevelFilter> {
        self.trace_level
            .file_tracing_level
            .map(|lv| lv.into_tracing_level_filter())
    }

    pub fn stdout_level(&self) -> Option<tracing::level_filters::LevelFilter> {
        self.trace_level
            .stdout_tracing_level
            .map(|lv| lv.into_tracing_level_filter())
    }
}
