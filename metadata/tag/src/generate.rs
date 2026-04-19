pub fn generate(code_snippets_path: String) -> anyhow::Result<()> {
    tracing::info!("Start generate tag data...");

    let video_tags = crate::model::LOADED_VIDEO_TAG_DATA.clone();
    generate_snippet(&video_tags, &code_snippets_path)?;

    tracing::info!("Generating tag data completed successfully");
    Ok(())
}

fn generate_snippet(
    video_tags: &crate::model::VideoTags,
    code_snippets_path: &str,
) -> anyhow::Result<()> {
    let snippets_path = std::path::Path::new(code_snippets_path);

    let mut snippet = crate::output::Snippet::load(snippets_path)?;
    snippet.output_json(snippets_path, video_tags)
}
