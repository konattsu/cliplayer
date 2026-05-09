pub(super) fn handle_minify(
    cmd: crate::cli::parser::BuildMinifyCommands,
) -> Result<(), crate::cli_exec_handler::CliExecError> {
    let music_lib = crate::music_file::MusicLibraryRepository::load(
        cmd.music_root.music_root_dir.as_path(),
    )?;

    crate::min_output::write_minified(
        &music_lib,
        cmd.min_output.min_clips_path.as_path(),
        cmd.min_output.min_videos_path.as_path(),
        cmd.dataset_build_id.dataset_build_id,
    )
    .map_err(|error| {
        crate::cli_exec_handler::CliExecError::MusicFile(error.into_errors())
    })?;

    Ok(())
}
