pub(super) async fn handle_sync(
    cmd: crate::cli::parser::SyncCommands,
) -> Result<(), ()> {
    use crate::music_file::MusicLibrary;

    let music_lib = MusicLibrary::load(cmd.music_root_dir.as_ref())
        .map_err(|e| tracing::error!("Failed to load music library: {e}"))?;

    crate::apply::apply_sync(
        music_lib,
        cmd.api_key,
        cmd.min_clips_path.as_ref(),
        cmd.min_videos_path.as_ref(),
    )
    .await
}
