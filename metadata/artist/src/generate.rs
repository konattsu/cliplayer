pub fn snippet(music_code_snippets_path: String) -> anyhow::Result<()> {
    tracing::info!("Start generate artist snippet...");

    let livers_data: crate::model::Livers = crate::model::LOADED_LIVER_DATA.clone();
    generate_snippet_impl(&livers_data, &music_code_snippets_path)?;

    tracing::info!("Generating artist snippet completed successfully");
    Ok(())
}

pub fn hash_inputs() -> anyhow::Result<String> {
    let livers = canonical_livers(crate::model::LOADED_LIVER_DATA.clone());
    let official_channels =
        canonical_official_channels(crate::model::LOADED_OFFICIAL_CHANNEL_DATA.clone());

    let mut builder =
        cmn_rs::min_json::InputSetHashBuilder::new("cliplayer:artist-inputs");
    builder.add_serializable("livers", &livers)?;
    builder.add_serializable("official_channels", &official_channels)?;
    Ok(builder.finish_hex())
}

pub fn minify(
    output_dir: String,
    livers_search_index_file_name: String,
    channels_file_name: String,
    livers_file_name: String,
    official_channels_file_name: String,
    dataset_build_id: cmn_rs::min_json::DatasetBuildId,
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
        dataset_build_id,
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

#[allow(clippy::too_many_arguments)] // 非公開関数で, いろいろ考えて警告無視
fn minify_impl(
    livers_data: crate::model::Livers,
    official_channels_data: crate::model::OfficialChannels,
    output_dir: &str,
    livers_search_index_file_name: &str,
    channels_file_name: &str,
    livers_file_name: &str,
    official_channels_file_name: &str,
    dataset_build_id: cmn_rs::min_json::DatasetBuildId,
) -> anyhow::Result<()> {
    let output_artists = crate::output::LiversSearchIndex::new(livers_data.clone());
    let channels =
        crate::output::Channels::new(&livers_data, official_channels_data.clone());
    let livers = crate::output::OutputLivers::new(livers_data);
    let official_channels =
        crate::output::OfficialChannels::new(official_channels_data);
    let build_metadata = build_metadata(dataset_build_id);

    let path = output_path(output_dir, livers_search_index_file_name);
    output_artists.output_as_json(&path, &build_metadata)?;

    let path = output_path(output_dir, channels_file_name);
    channels.output_json(&path, &build_metadata)?;

    let path = output_path(output_dir, livers_file_name);
    livers.output_json(&path, &build_metadata)?;

    let path = output_path(output_dir, official_channels_file_name);
    official_channels.output_json(&path, &build_metadata)?;

    Ok(())
}

fn output_path(output_dir: &str, file_name: &str) -> std::path::PathBuf {
    std::path::Path::new(output_dir).join(file_name)
}

fn output_snippet(
    livers_data: &crate::model::Livers,
    music_code_snippets_path: &str,
) -> anyhow::Result<()> {
    let music_data_code_snippets_path = std::path::Path::new(music_code_snippets_path);

    let mut snippet = crate::output::Snippet::load(music_data_code_snippets_path)?;
    snippet.output_json(music_data_code_snippets_path, livers_data)
}

fn build_metadata(
    dataset_build_id: cmn_rs::min_json::DatasetBuildId,
) -> crate::output::BuildMetadata {
    crate::output::BuildMetadata::new(dataset_build_id)
}

fn canonical_livers(
    livers: crate::model::Livers,
) -> Vec<(String, crate::model::Liver)> {
    let mut entries = livers
        .into_iter()
        .map(|(id, liver)| (id.as_str().to_string(), liver))
        .collect::<Vec<_>>();
    entries.sort_by(|left, right| left.0.cmp(&right.0));
    entries
}

fn canonical_official_channels(
    official_channels: crate::model::OfficialChannels,
) -> Vec<(String, crate::model::OfficialChannel)> {
    let mut entries = official_channels
        .into_iter()
        .map(|(id, channel)| (id.as_str().to_string(), channel))
        .collect::<Vec<_>>();
    entries.sort_by(|left, right| left.0.cmp(&right.0));
    entries
}
