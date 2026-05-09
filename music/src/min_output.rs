mod clips;
mod minified_json;
mod videos;

pub(crate) fn write_minified(
    library: &crate::music_file::MusicLibrary,
    min_clips_path: &std::path::Path,
    min_videos_path: &std::path::Path,
    dataset_build_id: cmn_rs::min_json::DatasetBuildId,
) -> Result<(), crate::music_file::MusicFileError> {
    tracing::info!(
        "Saving min files to disk: `{}` and `{}`",
        min_clips_path.display(),
        min_videos_path.display(),
    );

    let flat_clips = clips::FlatClips::from_library(library);
    let flat_videos = videos::FlatVideos::from_library(library)?;
    let metadata = minified_json::BuildMetadata::new(dataset_build_id);

    minified_json::write_json(min_clips_path, &flat_clips, &metadata)?;
    minified_json::write_json(min_videos_path, &flat_videos, &metadata)?;

    Ok(())
}
