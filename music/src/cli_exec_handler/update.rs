pub(super) fn handle_update(
    cmd: crate::cli::parser::UpdateCommands,
) -> Result<(), crate::cli_exec_handler::CliExecError> {
    use crate::cli::parser::UpdateMode;
    match cmd.mode {
        UpdateMode::Apply(args) => handle_apply(args),
        UpdateMode::Validate(args) => handle_validate(args),
    }
}

fn handle_apply(
    args: crate::cli::parser::UpdateApplyArgs,
) -> Result<(), crate::cli_exec_handler::CliExecError> {
    let music_lib =
        crate::music_file::MusicLibrary::load(args.music_root_dir.as_ref())?;
    crate::apply::apply_update(
        music_lib,
        args.min_clips_path.as_ref(),
        args.min_videos_path.as_ref(),
    )
    .map_err(Into::into)
}

fn handle_validate(
    args: crate::cli::parser::UpdateValidateArgs,
) -> Result<(), crate::cli_exec_handler::CliExecError> {
    // 読み込めてエラーがなければ検証成功
    let _music_lib =
        crate::music_file::MusicLibrary::load(args.music_root_dir.as_ref())?;
    Ok(())
}
