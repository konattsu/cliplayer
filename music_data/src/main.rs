#[tokio::main]
async fn main() {
    use clap::Parser;

    let cli = musictl::cli::Cli::parse();
    let _tracing_guard = enable_tracing_log(&cli);
    tracing::debug!("Command line arguments: {:?}", cli);

    let res = match cli.command {
        musictl::cli::Commands::Apply(apply_cmd) => handle_apply(apply_cmd).await,
        musictl::cli::Commands::Validate(validate_cmd) => handle_validate(validate_cmd),
        musictl::cli::Commands::Artist(artist_cmd) => handle_artist(artist_cmd),
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

// MARK: apply

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
    input: musictl::cli::FilePathFromCli,
    api_key: musictl::fetcher::YouTubeApiKey,
    music_lib: musictl::cli::MusicLibraryCli,
    output_min_file: musictl::cli::OutputMinPathFromCli,
    output_min_clips_file: musictl::cli::OutputMinClipsPathFromCli,
) -> Result<(), String> {
    let input_anonymous_file = input.try_into_file_path()?;

    let music_lib = music_lib
        .try_into_music_root_from_cli(output_min_file, output_min_clips_file)?;

    let anonymous_videos =
        musictl::validate::try_load_anonymous_videos(&input_anonymous_file)
            .map_err(|e| e.to_pretty_string())?;

    musictl::apply::apply_new(anonymous_videos, api_key, music_lib).await
}

fn handle_apply_update(
    music_lib: musictl::cli::MusicLibraryCli,
    output_min_file: musictl::cli::OutputMinPathFromCli,
    output_min_clips_file: musictl::cli::OutputMinClipsPathFromCli,
) -> Result<(), String> {
    let music_lib = music_lib
        .try_into_music_root_from_cli(output_min_file, output_min_clips_file)?;

    musictl::apply::apply_update(music_lib)
}

async fn handle_apply_sync(
    api_key: musictl::fetcher::YouTubeApiKey,
    music_lib: musictl::cli::MusicLibraryCli,
    output_min_file: musictl::cli::OutputMinPathFromCli,
    output_min_clips_file: musictl::cli::OutputMinClipsPathFromCli,
) -> Result<(), String> {
    let music_lib = music_lib
        .try_into_music_root_from_cli(output_min_file, output_min_clips_file)?;

    musictl::apply::apply_sync(music_lib, api_key).await
}

// MARK: validate

fn handle_validate(validate_cmd: musictl::cli::ValidateCommands) -> Result<(), String> {
    match validate_cmd {
        musictl::cli::ValidateCommands::New { input, .. } => handle_validate_new(input),
        musictl::cli::ValidateCommands::Update {
            music_root,
            output_min_file,
            output_min_clips_file,
            ..
        } => handle_validate_update(music_root, output_min_file, output_min_clips_file),
        musictl::cli::ValidateCommands::Duplicate {
            id,
            music_root,
            output_min_file,
            output_min_clips_file,
            ..
        } => handle_validate_duplicate(
            music_root,
            output_min_file,
            output_min_clips_file,
            id,
        ),
    }
}

fn handle_validate_new(
    input_file: musictl::cli::FilePathFromCli,
) -> Result<(), String> {
    let file = input_file.try_into_file_path()?;
    musictl::validate::validate_new_input(&file)
}

fn handle_validate_update(
    music_lib: musictl::cli::MusicLibraryCli,
    output_min_file: musictl::cli::OutputMinPathFromCli,
    output_min_clips_file: musictl::cli::OutputMinClipsPathFromCli,
) -> Result<(), String> {
    let music_lib = music_lib
        .try_into_music_root_from_cli(output_min_file, output_min_clips_file)?;
    musictl::apply::apply_update(music_lib)
}

fn handle_validate_duplicate(
    music_lib: musictl::cli::MusicLibraryCli,
    output_min_file: musictl::cli::OutputMinPathFromCli,
    output_min_clips_file: musictl::cli::OutputMinClipsPathFromCli,
    ids: musictl::cli::VideoIdsFromCli,
) -> Result<(), String> {
    let music_lib = music_lib
        .try_into_music_root_from_cli(output_min_file, output_min_clips_file)?;
    let video_ids = ids.as_ids();

    let duplicates = musictl::validate::find_video_ids(&music_lib, video_ids);

    if duplicates.is_empty() {
        println!("No duplicate video IDs found.");
    } else {
        println!("Duplicate video IDs found:");
        for id in duplicates {
            println!("Video ID: {id}",);
        }
    }

    Ok(())
}

// MARK: artist

fn handle_artist(artist_cmd: musictl::cli::ArtistCommands) -> Result<(), String> {
    match artist_cmd {
        musictl::cli::ArtistCommands::Generate {
            input_artists_data_path,
            artist_output_dir,
            search_index_file_name,
            channel_file_name,
            artists_file_name,
            music_data_code_snippets_path,
            ..
        } => musictl::artist::generate(
            input_artists_data_path,
            artist_output_dir,
            search_index_file_name,
            channel_file_name,
            artists_file_name,
            music_data_code_snippets_path,
        )
        .map_err(|e| e.to_string()),
    }
}
