pub fn generate(
    min_livers_output_dir: String,
    livers_search_index_file_name: String,
    channels_file_name: String,
    livers_file_name: String,
    music_code_snippets_path: String,
) -> anyhow::Result<()> {
    tracing::info!("Start generate artist data...");

    let livers_data: crate::model::Livers = crate::model::LOADED_LIVER_DATA.clone();
    let official_channel_data: crate::model::OfficialChannels =
        crate::model::LOADED_OFFICIAL_CHANNEL_DATA.clone();

    generate_artist_search_index(
        livers_data.clone(),
        &min_livers_output_dir,
        &livers_search_index_file_name,
    )?;
    generate_channels(
        &livers_data,
        official_channel_data,
        &min_livers_output_dir,
        &channels_file_name,
    )?;
    generate_snippet(&livers_data, &music_code_snippets_path)?;
    generate_livers(livers_data, &min_livers_output_dir, &livers_file_name)?;

    tracing::info!("Generating artist data completed successfully");
    Ok(())
}

fn generate_artist_search_index(
    livers_data: crate::model::Livers,
    min_livers_output_dir: &str,
    livers_search_index_file_name: &str,
) -> anyhow::Result<()> {
    let output_artists = crate::output::ArtistSearchIndex::new(livers_data);
    output_artists.output_as_json(std::path::Path::new(&format!(
        "{min_livers_output_dir}{livers_search_index_file_name}",
    )))?;
    Ok(())
}

fn generate_channels(
    livers_data: &crate::model::Livers,
    official_channel_data: crate::model::OfficialChannels,
    min_livers_output_dir: &str,
    channels_file_name: &str,
) -> anyhow::Result<()> {
    let channels = crate::output::Channels::new(livers_data, official_channel_data);

    channels.output_json(std::path::Path::new(&format!(
        "{min_livers_output_dir}{channels_file_name}",
    )))?;
    Ok(())
}

fn generate_livers(
    livers_data: crate::model::Livers,
    min_livers_output_dir: &str,
    artists_file_name: &str,
) -> anyhow::Result<()> {
    let output_artists = crate::output::OutputLivers::new(livers_data);
    output_artists.output_json(std::path::Path::new(&format!(
        "{min_livers_output_dir}{artists_file_name}",
    )))?;
    Ok(())
}

fn generate_snippet(
    livers_data: &crate::model::Livers,
    music_code_snippets_path: &str,
) -> anyhow::Result<()> {
    let music_data_code_snippets_path = std::path::Path::new(music_code_snippets_path);

    let mut snippet = crate::output::Snippet::load(music_data_code_snippets_path)?;
    snippet.output_json(music_data_code_snippets_path, livers_data)
}
