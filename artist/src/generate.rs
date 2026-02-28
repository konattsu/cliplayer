pub fn generate(
    artist_output_dir: String,
    search_index_file_name: String,
    channel_file_name: String,
    artists_file_name: String,
    music_data_code_snippets_path: String,
) -> anyhow::Result<()> {
    tracing::info!("Start generate artist data...");

    let livers_data: crate::model::Livers = crate::model::LOADED_LIVER_DATA.clone();
    let official_channel_data: crate::model::OfficialChannels =
        crate::model::LOADED_OFFICIAL_CHANNEL_DATA.clone();

    generate_artist_search_index(
        livers_data.clone(),
        &artist_output_dir,
        &search_index_file_name,
    )?;
    generate_channels(
        &livers_data,
        official_channel_data,
        &artist_output_dir,
        &channel_file_name,
    )?;
    generate_snippet(&livers_data, &music_data_code_snippets_path)?;
    generate_artists(livers_data, &artist_output_dir, &artists_file_name)?;

    tracing::info!("Generating artist data completed successfully");
    Ok(())
}

fn generate_artist_search_index(
    livers_data: crate::model::Livers,
    artist_output_dir: &str,
    search_index_file_name: &str,
) -> anyhow::Result<()> {
    let output_artists = crate::output::ArtistSearchIndex::new(livers_data);
    output_artists.output_as_json(std::path::Path::new(&format!(
        "{artist_output_dir}{search_index_file_name}",
    )))?;
    Ok(())
}

fn generate_channels(
    livers_data: &crate::model::Livers,
    official_channel_data: crate::model::OfficialChannels,
    artist_output_dir: &str,
    channel_file_name: &str,
) -> anyhow::Result<()> {
    let channels = crate::output::Channels::new(livers_data, official_channel_data);

    channels.output_json(std::path::Path::new(&format!(
        "{artist_output_dir}{channel_file_name}",
    )))?;
    Ok(())
}

fn generate_artists(
    livers_data: crate::model::Livers,
    artist_output_dir: &str,
    artists_file_name: &str,
) -> anyhow::Result<()> {
    let output_artists = crate::output::OutputLivers::new(livers_data);
    output_artists.output_json(std::path::Path::new(&format!(
        "{artist_output_dir}{artists_file_name}",
    )))?;
    Ok(())
}

fn generate_snippet(
    livers_data: &crate::model::Livers,
    music_data_code_snippets_path: &str,
) -> anyhow::Result<()> {
    let music_data_code_snippets_path =
        std::path::Path::new(music_data_code_snippets_path);

    let mut snippet = crate::output::Snippet::load(music_data_code_snippets_path)?;
    snippet.output_json(music_data_code_snippets_path, livers_data)
}
