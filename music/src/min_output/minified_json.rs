#[derive(Debug, Clone)]
pub(crate) struct BuildMetadata {
    data_build_id: String,
    generated_at: String,
}

impl BuildMetadata {
    const SCHEMA_VERSION: u32 = 1;

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
    ) -> Result<(), crate::music_file::MusicFileError>
    where
        T: serde::Serialize,
    {
        use std::hash::Hash;

        let bytes =
            serde_json::to_vec(value).map_err(|e| crate::music_file::MusicFileError::FileWrite {
                msg: e.to_string(),
                path: std::path::PathBuf::from("<memory>"),
            })?;
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
) -> Result<(), crate::music_file::MusicFileError>
where
    T: serde::Serialize,
{
    let envelope = MinifiedEnvelope {
        schema_version: BuildMetadata::SCHEMA_VERSION,
        data_build_id: &metadata.data_build_id,
        generated_at: &metadata.generated_at,
        data,
    };

    crate::music_file::fs_util::serialize_to_file(path, &envelope, true)
}
