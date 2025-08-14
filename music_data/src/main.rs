// TODO handler分離したい

#[tokio::main]
async fn main() {
    use clap::Parser;

    let cli = musictl::cli::Cli::parse();
    let _tracing_guard = enable_tracing_log(&cli);
    tracing::debug!("Command line arguments: {:?}", cli);

    let res = match cli.command {
        musictl::cli::Commands::Apply(apply_cmd) => handle_apply(apply_cmd).await,
        musictl::cli::Commands::Validate(validate_cmd) => handle_validate(validate_cmd),
        musictl::cli::Commands::Dev(dev_cmd) => handle_dev(dev_cmd),
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
    input: musictl::cli::FilePathsFromCli,
    api_key: musictl::fetcher::YouTubeApiKey,
    music_lib: musictl::cli::MusicLibraryCli,
    output_min_file: String,
    output_min_clips_file: String,
) -> Result<(), String> {
    let input_anonymous_file = input.try_into_file_paths()?;

    let music_lib = music_lib
        .try_into_music_root_from_cli(output_min_file, output_min_clips_file)?;

    let anonymous_videos =
        musictl::validate::try_load_anonymous_videos(&input_anonymous_file)
            .map_err(|e| e.to_pretty_string())?;

    musictl::apply::apply_new(anonymous_videos, api_key, music_lib).await
}

fn handle_apply_update(
    music_lib: musictl::cli::MusicLibraryCli,
    output_min_file: String,
    output_min_clips_file: String,
) -> Result<(), String> {
    let music_lib = music_lib
        .try_into_music_root_from_cli(output_min_file, output_min_clips_file)?;

    musictl::apply::apply_update(music_lib)
}

async fn handle_apply_sync(
    api_key: musictl::fetcher::YouTubeApiKey,
    music_lib: musictl::cli::MusicLibraryCli,
    output_min_file: String,
    output_min_clips_file: String,
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
        musictl::cli::ValidateCommands::NewMd { input, .. } => {
            handle_validate_new_md(input)
        }
    }
}

fn handle_validate_new(
    input_file: musictl::cli::FilePathsFromCli,
) -> Result<(), String> {
    let files = input_file.try_into_file_paths()?;
    musictl::validate::validate_new_input(&files)
}

fn handle_validate_update(
    music_lib: musictl::cli::MusicLibraryCli,
    output_min_file: String,
    output_min_clips_file: String,
) -> Result<(), String> {
    let music_lib = music_lib
        .try_into_music_root_from_cli(output_min_file, output_min_clips_file)?;
    musictl::apply::apply_update(music_lib)
}

fn handle_validate_new_md(input_file: musictl::cli::FilePathsFromCli) -> ! {
    let files = match input_file.try_into_file_paths() {
        Ok(f) => f,
        Err(_e) => std::process::exit(1),
    };

    match musictl::validate::validate_new_input_md(&files) {
        Ok(md_str) => {
            println!("{md_str}");
            std::process::exit(0)
        }
        Err(_e) => std::process::exit(1),
    }
}

// MARK: dev

fn handle_dev(dev_cmd: musictl::cli::UtilCommands) -> Result<(), String> {
    match dev_cmd {
        musictl::cli::UtilCommands::GenerateArtist {
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
        musictl::cli::UtilCommands::MergeFiles {
            files,
            dir,
            output_dir,
            ..
        } => handle_dev_merge_files(files, dir, output_dir),
        musictl::cli::UtilCommands::DuplicateIds {
            ids,
            music_lib,
            output_min_file,
            output_min_clips_file,
            ..
        } => handle_dev_duplicate_ids(
            music_lib,
            output_min_file,
            output_min_clips_file,
            ids,
        ),
    }
}

fn handle_dev_merge_files(
    files: Option<musictl::cli::FilePathsFromCli>,
    dir: String,
    output_dir: String,
) -> Result<(), String> {
    let files = files.map(|f| f.try_into_file_paths()).transpose()?;
    let dir = musictl::util::DirPath::from_path_buf(std::path::PathBuf::from(dir))
        .map_err(|e| format!("Invalid directory path: {e}"))?;
    let output_dir =
        musictl::util::DirPath::from_path_buf(std::path::PathBuf::from(output_dir))
            .map_err(|e| format!("Invalid output directory path: {e}"))?;

    let files = musictl::dev::merge_files(files, &dir, output_dir)?;

    if dialoguer::Confirm::new()
        .with_prompt(format!(
            "{} files merged. Do you want to delete the original files?",
            files.len()
        ))
        .interact()
        .map_err(|e| e.to_string())?
    {
        for file in &files {
            if let Err(e) = std::fs::remove_file(file.as_path()) {
                eprintln!("Failed to delete file {file}: {e}");
            } else {
                println!("Deleted file: {file}");
            }
        }
    } else {
        println!("Original files were not deleted.");
    }
    Ok(())
}

fn handle_dev_duplicate_ids(
    music_lib: musictl::cli::MusicLibraryCli,
    output_min_file: String,
    output_min_clips_file: String,
    ids: musictl::cli::VideoIdsFromCli,
) -> Result<(), String> {
    let music_lib = music_lib
        .try_into_music_root_from_cli(output_min_file, output_min_clips_file)?;
    let video_ids = ids.as_ids();

    println!(
        "Checking if any of the {} provided video IDs are already registered in the database.",
        video_ids.len()
    );

    let duplicates = musictl::dev::find_video_ids(&music_lib, video_ids);

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
