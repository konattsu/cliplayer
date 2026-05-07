mod clips;
mod minified_json;
mod videos;

pub(crate) fn write_minified(
    library: &crate::music_file::MusicLibrary,
    min_clips_path: &std::path::Path,
    min_videos_path: &std::path::Path,
) -> Result<(), crate::music_file::MusicFileError> {
    tracing::info!(
        "Saving min files to disk: `{}` and `{}`",
        min_clips_path.display(),
        min_videos_path.display(),
    );

    let flat_clips = clips::FlatClips::from_library(library);
    let flat_videos = videos::FlatVideos::from_library(library)?;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    minified_json::BuildMetadata::hash_serializable(&mut hasher, &flat_clips)?;
    minified_json::BuildMetadata::hash_serializable(&mut hasher, &flat_videos)?;
    let metadata = minified_json::BuildMetadata::from_hash(hasher);

    minified_json::write_json(min_clips_path, &flat_clips, &metadata)?;
    minified_json::write_json(min_videos_path, &flat_videos, &metadata)?;

    Ok(())
}
