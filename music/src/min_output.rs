mod clips;
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

    crate::music_file::fs_util::serialize_to_file(min_clips_path, &flat_clips, true)?;
    crate::music_file::fs_util::serialize_to_file(min_videos_path, &flat_videos, true)?;

    Ok(())
}
