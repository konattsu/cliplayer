pub fn generate(
    output_dir: String,
    min_tags_file_name: String,
    code_snippets_path: String,
) -> anyhow::Result<()> {
    tracing::info!("Start generate tag data...");

    let video_tags = crate::model::LOADED_VIDEO_TAG_DATA.clone();

    minify_impl(&video_tags, &output_dir, &min_tags_file_name)?;
    generate_snippet_impl(&video_tags, &code_snippets_path)?;

    tracing::info!("Generating tag data completed successfully");
    Ok(())
}

pub fn generate_snippet(code_snippets_path: String) -> anyhow::Result<()> {
    tracing::info!("Start generate tag snippet...");

    let video_tags = crate::model::LOADED_VIDEO_TAG_DATA.clone();
    generate_snippet_impl(&video_tags, &code_snippets_path)?;

    tracing::info!("Generating tag snippet completed successfully");
    Ok(())
}

pub fn minify(output_dir: String, min_tags_file_name: String) -> anyhow::Result<()> {
    tracing::info!("Start generate tag min data...");

    let video_tags = crate::model::LOADED_VIDEO_TAG_DATA.clone();
    minify_impl(&video_tags, &output_dir, &min_tags_file_name)?;

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
) -> anyhow::Result<()> {
    let output = crate::output::MinVideoTags::new(video_tags);
    let path = std::path::Path::new(output_dir).join(min_tags_file_name);
    output.output_json(&path)
}
