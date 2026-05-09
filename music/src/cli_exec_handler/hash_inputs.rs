pub(super) fn handle_hash_inputs(
    cmd: crate::cli::parser::BuildHashInputsCommands,
) -> Result<(), crate::cli_exec_handler::CliExecError> {
    let hash =
        crate::hash_inputs::hash_music_inputs(cmd.music_root.music_root_dir.as_path())?;
    println!("{hash}");
    Ok(())
}
