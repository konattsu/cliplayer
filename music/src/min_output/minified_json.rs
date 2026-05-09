pub(crate) type BuildMetadata = cmn_rs::min_json::BuildMetadata;

pub(crate) fn write_json<T>(
    path: &std::path::Path,
    data: &T,
    metadata: &BuildMetadata,
) -> Result<(), crate::music_file::MusicFileError>
where
    T: serde::Serialize,
{
    let envelope = cmn_rs::min_json::MinEnvelope::new(data, metadata);

    crate::music_file::fs_util::serialize_to_file(path, &envelope, true)
}
