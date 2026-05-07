#[derive(Debug, Clone)]
pub(crate) struct BuildMetadata {
    data_build_id: String,
    generated_at: String,
}

impl BuildMetadata {
    pub(crate) const SCHEMA_VERSION: u32 = 1;

    pub(crate) fn from_hash(hasher: std::collections::hash_map::DefaultHasher) -> Self {
        let now = chrono::Utc::now();
        Self {
            data_build_id: format!("{:016x}", std::hash::Hasher::finish(&hasher)),
            generated_at: now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        }
    }

    pub(crate) fn hash_serializable<T>(
        hasher: &mut std::collections::hash_map::DefaultHasher,
        value: &T,
    ) -> anyhow::Result<()>
    where
        T: serde::Serialize,
    {
        use std::hash::Hash;

        let bytes = serde_json::to_vec(value)?;
        bytes.hash(hasher);
        Ok(())
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct MinifiedEnvelope<'a, T>
where
    T: serde::Serialize,
{
    schema_version: u32,
    data_build_id: &'a str,
    generated_at: &'a str,
    data: &'a T,
}

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

    let file = std::fs::File::create(path)
        .with_context(|| format!("Failed to create/truncate file at {}", path.display()))?;
    let envelope = MinifiedEnvelope {
        schema_version: BuildMetadata::SCHEMA_VERSION,
        data_build_id: &metadata.data_build_id,
        generated_at: &metadata.generated_at,
        data,
    };
    serde_json::to_writer(file, &envelope)
        .with_context(|| format!("Failed to write JSON to file: {}", path.display()))?;

    Ok(())
}
