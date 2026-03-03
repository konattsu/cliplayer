pub(super) async fn handle_add(cmd: crate::cli::parser::AddCommands) -> Result<(), ()> {
    use crate::cli::parser::AddMode;
    match cmd.mode {
        AddMode::Apply(args) => handle_apply(args).await,
        AddMode::Validate(args) => handle_validate(args),
    }
}

async fn handle_apply(args: crate::cli::parser::AddApplyArgs) -> Result<(), ()> {
    use crate::music_file::MusicLibrary;

    let music_lib = MusicLibrary::load(args.music_root_dir.as_ref())
        .map_err(|e| tracing::error!("Failed to load music library: {e}"))?;

    let input_files = args.input.into_file_paths();
    let anonymous_videos = crate::validate::try_load_anonymous_videos(&input_files)
        .map_err(|e| {
            tracing::error!("{e}");
        })?;

    crate::apply::apply_add(
        music_lib,
        anonymous_videos,
        args.api_key,
        args.min_clips_path.as_ref(),
        args.min_videos_path.as_ref(),
    )
    .await
}

fn handle_validate(args: crate::cli::parser::AddValidateArgs) -> Result<(), ()> {
    let input_files = args.input.into_file_paths();

    let anonymous_videos = crate::validate::try_load_anonymous_videos(&input_files)
        .map_err(|e| tracing::error!("{e}"))?;

    if args.markdown {
        println!("# Music Data Summary\n\n{}", anonymous_videos.to_markdown());
    }

    Ok(())
}
