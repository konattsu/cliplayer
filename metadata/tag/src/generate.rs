pub fn snippet(code_snippets_path: String) -> anyhow::Result<()> {
    tracing::info!("Start generate tag snippet...");

    let video_tags = crate::model::LOADED_VIDEO_TAG_DATA.clone();
    generate_snippet_impl(&video_tags, &code_snippets_path)?;

    tracing::info!("Generating tag snippet completed successfully");
    Ok(())
}

pub fn hash_inputs() -> anyhow::Result<String> {
    let video_tags = canonical_video_tags(&crate::model::LOADED_VIDEO_TAG_DATA);

    let mut builder =
        cmn_rs::min_json::InputSetHashBuilder::new("cliplayer:tag-inputs");
    builder.add_serializable("video_tags", &video_tags)?;
    Ok(builder.finish_hex())
}

pub fn minify(
    output_dir: String,
    min_tags_file_name: String,
    dataset_build_id: cmn_rs::min_json::DatasetBuildId,
) -> anyhow::Result<()> {
    tracing::info!("Start generate tag min data...");

    let video_tags = crate::model::LOADED_VIDEO_TAG_DATA.clone();
    minify_impl(
        &video_tags,
        &output_dir,
        &min_tags_file_name,
        dataset_build_id,
    )?;

    tracing::info!("Generating tag min data completed successfully");
    Ok(())
}

fn generate_snippet_impl(
    video_tags: &crate::model::VideoTags,
    code_snippets_path: &str,
) -> anyhow::Result<()> {
    let snippets_path = std::path::Path::new(code_snippets_path);

    let mut snippet = crate::output::Snippet::load(snippets_path)?;
    snippet.output_json(snippets_path, video_tags)
}

fn minify_impl(
    video_tags: &crate::model::VideoTags,
    output_dir: &str,
    min_tags_file_name: &str,
    dataset_build_id: cmn_rs::min_json::DatasetBuildId,
) -> anyhow::Result<()> {
    let output = crate::output::MinVideoTags::new(video_tags);
    let metadata = crate::output::BuildMetadata::new(dataset_build_id);
    let path = std::path::Path::new(output_dir).join(min_tags_file_name);
    output.output_json(&path, &metadata)
}

fn canonical_video_tags(
    video_tags: &crate::model::VideoTags,
) -> Vec<(String, crate::model::VideoTag)> {
    let mut entries = video_tags
        .iter()
        .map(|(id, tag)| (id.as_str().to_string(), tag.clone()))
        .collect::<Vec<_>>();
    entries.sort_by(|left, right| left.0.cmp(&right.0));
    entries
}
