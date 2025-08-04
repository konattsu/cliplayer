pub fn generate(
    input_artists_data_path: String,
    artist_output_dir: String,
    search_index_file_name: String,
    channel_file_name: String,
    artists_file_name: String,
    music_data_code_snippets_path: String,
) -> anyhow::Result<()> {
    tracing::info!("Start generate artist data...");

    let artist_data: crate::artist::model::Artists =
        crate::artist::model::Artists::load(&input_artists_data_path)?;

    generate_artist_search_index(
        artist_data.clone(),
        &artist_output_dir,
        &search_index_file_name,
    )?;
    generate_channels(&artist_data, &artist_output_dir, &channel_file_name)?;
    generate_snippet(&artist_data, &music_data_code_snippets_path)?;
    generate_artists(artist_data, &artist_output_dir, &artists_file_name)?;

    tracing::info!("Generating artist data completed successfully");
    Ok(())
}

fn generate_artist_search_index(
    artist_data: crate::artist::model::Artists,
    artist_output_dir: &str,
    search_index_file_name: &str,
) -> anyhow::Result<()> {
    let output_artists = crate::artist::output::ArtistSearchIndex::new(artist_data);
    output_artists.output_json(std::path::Path::new(&format!(
        "{artist_output_dir}{search_index_file_name}",
    )))?;
    Ok(())
}

fn generate_channels(
    artist_data: &crate::artist::model::Artists,
    artist_output_dir: &str,
    channel_file_name: &str,
) -> anyhow::Result<()> {
    let channels = crate::artist::output::Channels::new(artist_data);
    channels.output_json(std::path::Path::new(&format!(
        "{artist_output_dir}{channel_file_name}",
    )))?;
    Ok(())
}

fn generate_artists(
    artist_data: crate::artist::model::Artists,
    artist_output_dir: &str,
    artists_file_name: &str,
) -> anyhow::Result<()> {
    let output_artists = crate::artist::output::OutputArtists::new(artist_data);
    output_artists.output_json(std::path::Path::new(&format!(
        "{artist_output_dir}{artists_file_name}",
    )))?;
    Ok(())
}

fn generate_snippet(
    artist_data: &crate::artist::model::Artists,
    music_data_code_snippets_path: &str,
) -> anyhow::Result<()> {
    let music_data_code_snippets_path =
        std::path::Path::new(music_data_code_snippets_path);

    let mut snippet =
        crate::artist::output::Snippet::load(music_data_code_snippets_path)?;
    snippet.output_json(music_data_code_snippets_path, artist_data)
}
