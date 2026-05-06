pub(super) async fn handle_add(
    cmd: crate::cli::parser::AddCommands,
) -> Result<(), crate::cli_exec_handler::CliExecError> {
    use crate::cli::parser::AddMode;
    match cmd.mode {
        AddMode::Apply(args) => handle_apply(args).await,
        AddMode::Validate(args) => handle_validate(args),
    }
}

async fn handle_apply(
    args: crate::cli::parser::AddApplyArgs,
) -> Result<(), crate::cli_exec_handler::CliExecError> {
    let music_lib = crate::music_file::MusicLibraryRepository::load(
        args.music_root.music_root_dir.as_path(),
    )?;

    let input_files = args.input.into_file_paths();
    let anonymous_videos = crate::validate::try_load_anonymous_videos(&input_files)?;

    crate::apply::apply_add(
        music_lib,
        anonymous_videos,
        args.api_key.api_key,
        args.duplicate_video_policy.duplicate_video_policy(),
    )
    .await
    .map_err(Into::into)
}

fn handle_validate(
    args: crate::cli::parser::AddValidateArgs,
) -> Result<(), crate::cli_exec_handler::CliExecError> {
    let input_files = args.input.into_file_paths();

    let anonymous_videos = crate::validate::try_load_anonymous_videos(&input_files)?;

    if args.markdown.markdown {
        println!(
            "# Music Data Summary\n\n{}",
            crate::report::anonymous_videos_to_markdown(&anonymous_videos)
        );
    }

    Ok(())
}
