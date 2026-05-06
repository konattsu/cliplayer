pub(super) async fn handle_sync(
    cmd: crate::cli::parser::SyncCommands,
) -> Result<(), crate::cli_exec_handler::CliExecError> {
    let music_lib = crate::music_file::MusicLibraryRepository::load(
        cmd.music_root.music_root_dir.as_path(),
    )?;

    crate::apply::apply_sync(music_lib, cmd.api_key.api_key)
        .await
        .map_err(Into::into)
}
