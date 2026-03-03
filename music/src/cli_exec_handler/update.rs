pub(super) fn handle_update(cmd: crate::cli::parser::UpdateCommands) -> Result<(), ()> {
    use crate::cli::parser::UpdateMode;
    match cmd.mode {
        UpdateMode::Apply(args) => handle_apply(args),
        UpdateMode::Validate(args) => handle_validate(args),
    }
}

fn handle_apply(args: crate::cli::parser::UpdateApplyArgs) -> Result<(), ()> {
    let music_lib = crate::music_file::MusicLibrary::load(args.music_root_dir.as_ref())
        .map_err(|e| tracing::error!("Failed to load music library: {e}"))?;
    crate::apply::apply_update(
        music_lib,
        args.min_clips_path.as_ref(),
        args.min_videos_path.as_ref(),
    )
}

fn handle_validate(args: crate::cli::parser::UpdateValidateArgs) -> Result<(), ()> {
    // 読み込めてエラーがなければ検証成功
    let _music_lib =
        crate::music_file::MusicLibrary::load(args.music_root_dir.as_ref())
            .map_err(|e| tracing::error!("Failed to validate music library: {e}"))?;
    Ok(())
}
