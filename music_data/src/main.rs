#[tokio::main]
async fn main() {
    use clap::Parser;

    let cli = musictl::cli::Cli::parse();
    let _tracing_guard = enable_tracing_log(&cli);

    let res = match cli.command {
        musictl::cli::Commands::Apply(apply_cmd) => handle_apply(apply_cmd).await,
        musictl::cli::Commands::Validate(validate_cmd) => handle_validate(validate_cmd),
    };

    if let Err(e) = res {
        tracing::debug!("Error occurred: {e}");
        eprintln!("Error: {e}");
        std::process::exit(1);
    } else {
        println!("Command executed successfully.");
    }
}

fn enable_tracing_log(
    cli: &musictl::cli::Cli,
) -> tracing_appender::non_blocking::WorkerGuard {
    musictl::util::apply_tracing_settings(cli.stdout_level(), cli.file_level())
}

async fn handle_apply(apply_cmd: musictl::cli::ApplyCommands) -> Result<(), String> {
    match apply_cmd {
        musictl::cli::ApplyCommands::New {
            input,
            api_key,
            music_root,
            output_min_file,
            output_min_clips_file,
            ..
        } => {
            handle_apply_new(
                input,
                api_key,
                music_root,
                output_min_file,
                output_min_clips_file,
            )
            .await
        }
        musictl::cli::ApplyCommands::Update {
            music_root,
            output_min_file,
            output_min_clips_file,
            ..
        } => handle_apply_update(music_root, output_min_file, output_min_clips_file),
        musictl::cli::ApplyCommands::Sync {
            api_key,
            music_root,
            output_min_file,
            output_min_clips_file,
            ..
        } => {
            handle_apply_sync(
                api_key,
                music_root,
                output_min_file,
                output_min_clips_file,
            )
            .await
        }
    }
}

async fn handle_apply_new(
    input: musictl::cli::FilePathsFromCli,
    api_key: musictl::fetcher::YouTubeApiKey,
    music_root: musictl::cli::MusicRootFromCli,
    output_min_file: musictl::cli::OutputMinPathFromCli,
    output_min_clips_file: musictl::cli::OutputMinClipsPathFromCli,
) -> Result<(), String> {
    let files = input.try_into_vec()?;
    let music_root = music_root.try_into_music_root()?;
    let min_path = output_min_file.try_into_path()?;
    let min_flat_clips_path = output_min_clips_file.try_into_path()?;

    let anonymous_videos = musictl::validate::try_load_anonymous_videos(&files)
        .map_err(|e| e.to_pretty_string())?;

    musictl::apply::apply_new(
        anonymous_videos,
        api_key,
        music_root,
        &min_path,
        &min_flat_clips_path,
    )
    .await
}

fn handle_apply_update(
    music_root: musictl::cli::MusicRootFromCli,
    output_min_file: musictl::cli::OutputMinPathFromCli,
    output_min_clips_file: musictl::cli::OutputMinClipsPathFromCli,
) -> Result<(), String> {
    let music_root = music_root.try_into_music_root()?;
    let min_path = output_min_file.try_into_path()?;
    let min_flat_clips_path = output_min_clips_file.try_into_path()?;

    musictl::apply::apply_update(music_root, &min_path, &min_flat_clips_path)
}

async fn handle_apply_sync(
    api_key: musictl::fetcher::YouTubeApiKey,
    music_root: musictl::cli::MusicRootFromCli,
    output_min_file: musictl::cli::OutputMinPathFromCli,
    output_min_clips_file: musictl::cli::OutputMinClipsPathFromCli,
) -> Result<(), String> {
    let music_root = music_root.try_into_music_root()?;
    let min_path = output_min_file.try_into_path()?;
    let min_flat_clips_path = output_min_clips_file.try_into_path()?;

    musictl::apply::apply_sync(api_key, music_root, &min_path, &min_flat_clips_path)
        .await
}

fn handle_validate(validate_cmd: musictl::cli::ValidateCommands) -> Result<(), String> {
    match validate_cmd {
        musictl::cli::ValidateCommands::NewInput { input, .. } => {
            let files = input.try_into_vec()?;
            musictl::validate::validate_new_input(&files)
        }
        musictl::cli::ValidateCommands::UpdateInput { music_root, .. } => {
            let music_root = music_root.try_into_music_root()?;
            musictl::validate::validate_update_input(&music_root)
        }
        musictl::cli::ValidateCommands::Duplicate { id, music_root, .. } => {
            let ids = id.as_ids();
            let music_root = music_root.try_into_music_root()?;

            let duplicates = musictl::validate::duplicate_video_ids(&music_root, ids)?;

            if duplicates.is_empty() {
                println!("No duplicate video IDs found.");
            } else {
                println!("Duplicate video IDs found:");
                for dup in duplicates {
                    println!(
                        "Video ID: {}, Published At: {}",
                        dup.video_id, dup.published_at
                    );
                }
            }
            Ok(())
        }
    }
}
