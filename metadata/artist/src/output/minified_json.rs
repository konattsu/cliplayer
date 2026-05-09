pub(crate) type BuildMetadata = cmn_rs::min_json::BuildMetadata;

pub(crate) fn write_json<T>(
    path: &std::path::Path,
    data: &T,
    metadata: &BuildMetadata,
) -> anyhow::Result<()>
where
    T: serde::Serialize,
{
    use anyhow::Context;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).with_context(|| {
            format!("Failed to create output directory: {}", parent.display())
        })?;
    }

    let file = std::fs::File::create(path).with_context(|| {
        format!("Failed to create/truncate file at {}", path.display())
    })?;
    let envelope = cmn_rs::min_json::MinEnvelope::new(data, metadata);
    serde_json::to_writer(file, &envelope)
        .with_context(|| format!("Failed to write JSON to file: {}", path.display()))?;

    Ok(())
}
