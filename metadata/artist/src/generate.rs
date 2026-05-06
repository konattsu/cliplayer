pub fn generate(
    output_dir: String,
    livers_search_index_file_name: String,
    channels_file_name: String,
    livers_file_name: String,
    official_channels_file_name: String,
    music_code_snippets_path: String,
) -> anyhow::Result<()> {
    tracing::info!("Start generate artist data...");

    let livers_data: crate::model::Livers = crate::model::LOADED_LIVER_DATA.clone();
    let official_channels_data: crate::model::OfficialChannels =
        crate::model::LOADED_OFFICIAL_CHANNEL_DATA.clone();

    minify_impl(
        livers_data.clone(),
        official_channels_data.clone(),
        &output_dir,
        &livers_search_index_file_name,
        &channels_file_name,
        &livers_file_name,
        &official_channels_file_name,
    )?;
    generate_snippet_impl(&livers_data, &music_code_snippets_path)?;

    tracing::info!("Generating artist data completed successfully");
    Ok(())
}

pub fn generate_snippet(music_code_snippets_path: String) -> anyhow::Result<()> {
    tracing::info!("Start generate artist snippet...");

    let livers_data: crate::model::Livers = crate::model::LOADED_LIVER_DATA.clone();
    generate_snippet_impl(&livers_data, &music_code_snippets_path)?;

    tracing::info!("Generating artist snippet completed successfully");
    Ok(())
}

pub fn minify(
    output_dir: String,
    livers_search_index_file_name: String,
    channels_file_name: String,
    livers_file_name: String,
    official_channels_file_name: String,
) -> anyhow::Result<()> {
    tracing::info!("Start generate artist min data...");

    let livers_data: crate::model::Livers = crate::model::LOADED_LIVER_DATA.clone();
    let official_channels_data: crate::model::OfficialChannels =
        crate::model::LOADED_OFFICIAL_CHANNEL_DATA.clone();

    minify_impl(
        livers_data,
        official_channels_data,
        &output_dir,
        &livers_search_index_file_name,
        &channels_file_name,
        &livers_file_name,
        &official_channels_file_name,
    )?;

    tracing::info!("Generating artist min data completed successfully");
    Ok(())
}

fn generate_snippet_impl(
    livers_data: &crate::model::Livers,
    music_code_snippets_path: &str,
) -> anyhow::Result<()> {
    output_snippet(livers_data, music_code_snippets_path)
}

fn minify_impl(
    livers_data: crate::model::Livers,
    official_channels_data: crate::model::OfficialChannels,
    output_dir: &str,
    livers_search_index_file_name: &str,
    channels_file_name: &str,
    livers_file_name: &str,
    official_channels_file_name: &str,
) -> anyhow::Result<()> {
    generate_artist_search_index(
        livers_data.clone(),
        output_dir,
        livers_search_index_file_name,
    )?;
    generate_channels(
        &livers_data,
        official_channels_data.clone(),
        output_dir,
        channels_file_name,
    )?;
    generate_livers(livers_data, output_dir, livers_file_name)?;
    generate_official_channels(
        official_channels_data,
        output_dir,
        official_channels_file_name,
    )?;

    Ok(())
}

fn output_path(output_dir: &str, file_name: &str) -> std::path::PathBuf {
    std::path::Path::new(output_dir).join(file_name)
}

fn generate_artist_search_index(
    livers_data: crate::model::Livers,
    output_dir: &str,
    livers_search_index_file_name: &str,
) -> anyhow::Result<()> {
    let output_artists = crate::output::LiversSearchIndex::new(livers_data);
    let path = output_path(output_dir, livers_search_index_file_name);

    output_artists.output_as_json(&path)
}

fn generate_channels(
    livers_data: &crate::model::Livers,
    official_channels_data: crate::model::OfficialChannels,
    output_dir: &str,
    channels_file_name: &str,
) -> anyhow::Result<()> {
    let channels = crate::output::Channels::new(livers_data, official_channels_data);
    let path = output_path(output_dir, channels_file_name);

    channels.output_json(&path)
}

fn generate_livers(
    livers_data: crate::model::Livers,
    output_dir: &str,
    artists_file_name: &str,
) -> anyhow::Result<()> {
    let output_artists = crate::output::OutputLivers::new(livers_data);
    let path = output_path(output_dir, artists_file_name);

    output_artists.output_json(&path)
}

fn output_snippet(
    livers_data: &crate::model::Livers,
    music_code_snippets_path: &str,
) -> anyhow::Result<()> {
    let music_data_code_snippets_path = std::path::Path::new(music_code_snippets_path);

    let mut snippet = crate::output::Snippet::load(music_data_code_snippets_path)?;
    snippet.output_json(music_data_code_snippets_path, livers_data)
}

fn generate_official_channels(
    official_channels_data: crate::model::OfficialChannels,
    output_dir: &str,
    official_channels_file_name: &str,
) -> anyhow::Result<()> {
    let official_channels =
        crate::output::OfficialChannels::new(official_channels_data);
    let path = output_path(output_dir, official_channels_file_name);

    official_channels.output_json(&path)
}
