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
    use crate::music_file::MusicLibrary;

    let music_lib = MusicLibrary::load(args.music_root_dir.as_ref())?;

    let input_files = args.input.into_file_paths();
    let anonymous_videos = crate::validate::try_load_anonymous_videos(&input_files)?;

    let duplicate_video_policy = if args.allow_overwrite_existing_video {
        crate::music_file::DuplicateVideoPolicy::Overwrite
    } else {
        crate::music_file::DuplicateVideoPolicy::Reject
    };

    crate::apply::apply_add(
        music_lib,
        anonymous_videos,
        args.api_key,
        duplicate_video_policy,
        args.min_clips_path.as_ref(),
        args.min_videos_path.as_ref(),
    )
    .await
    .map_err(Into::into)
}

fn handle_validate(
    args: crate::cli::parser::AddValidateArgs,
) -> Result<(), crate::cli_exec_handler::CliExecError> {
    let input_files = args.input.into_file_paths();

    let anonymous_videos = crate::validate::try_load_anonymous_videos(&input_files)?;

    if args.markdown {
        println!("# Music Data Summary\n\n{}", anonymous_videos.to_markdown());
    }

    Ok(())
}
