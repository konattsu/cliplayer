// 将来的にここ定義して柔軟に色々エラーを扱いたいという気持ち
// pub struct CliExecError {
//     //
// }

pub async fn cli_exec_handler(cli: crate::cli::Cli) -> Result<(), String> {
    match cli.command {
        crate::cli::Commands::Apply(apply_cmd) => handle_apply(apply_cmd).await,
        crate::cli::Commands::Validate(validate_cmd) => handle_validate(validate_cmd),
        crate::cli::Commands::Dev(dev_cmd) => handle_dev(dev_cmd),
    }
}

// MARK: apply

async fn handle_apply(apply_cmd: crate::cli::ApplyCommands) -> Result<(), String> {
    match apply_cmd {
        crate::cli::ApplyCommands::New {
            input,
            api_key,
            music_root,
            min_clips_path,
            min_videos_path,
            ..
        } => {
            handle_apply_new(
                input,
                api_key,
                music_root,
                min_clips_path,
                min_videos_path,
            )
            .await
        }

        crate::cli::ApplyCommands::Update {
            music_root,
            min_clips_path,
            min_videos_path,
            ..
        } => handle_apply_update(music_root, min_clips_path, min_videos_path),

        crate::cli::ApplyCommands::Sync {
            api_key,
            music_root,
            min_clips_path,
            min_videos_path,
            ..
        } => {
            handle_apply_sync(api_key, music_root, min_clips_path, min_videos_path)
                .await
        }
    }
}

async fn handle_apply_new(
    input: crate::cli::FilePathsFromCli,
    api_key: crate::fetcher::YouTubeApiKey,
    music_lib: crate::cli::MusicLibraryCli,
    min_clips_path: crate::cli::FilePathFromCli,
    min_videos_path: crate::cli::FilePathFromCli,
) -> Result<(), String> {
    let input_anonymous_file = input.try_into_file_paths()?;
    let min_clips_path = min_clips_path.try_into_file_path()?;
    let min_videos_path = min_videos_path.try_into_file_path()?;

    let music_lib = music_lib.try_into_music_root_from_cli()?;

    let anonymous_videos =
        crate::validate::try_load_anonymous_videos(&input_anonymous_file)
            .map_err(|e| e.to_pretty_string())?;

    crate::apply::apply_new(
        music_lib,
        anonymous_videos,
        api_key,
        &min_clips_path,
        &min_videos_path,
    )
    .await
}

fn handle_apply_update(
    music_lib: crate::cli::MusicLibraryCli,
    min_clips_path: crate::cli::FilePathFromCli,
    min_videos_path: crate::cli::FilePathFromCli,
) -> Result<(), String> {
    let min_clips_path = min_clips_path.try_into_file_path()?;
    let min_videos_path = min_videos_path.try_into_file_path()?;
    let music_lib = music_lib.try_into_music_root_from_cli()?;

    crate::apply::apply_update(music_lib, &min_clips_path, &min_videos_path)
}

async fn handle_apply_sync(
    api_key: crate::fetcher::YouTubeApiKey,
    music_lib: crate::cli::MusicLibraryCli,
    min_clips_path: crate::cli::FilePathFromCli,
    min_videos_path: crate::cli::FilePathFromCli,
) -> Result<(), String> {
    let min_clips_path = min_clips_path.try_into_file_path()?;
    let min_videos_path = min_videos_path.try_into_file_path()?;
    let music_lib = music_lib.try_into_music_root_from_cli()?;

    crate::apply::apply_sync(music_lib, api_key, &min_clips_path, &min_videos_path)
        .await
}

// MARK: validate

fn handle_validate(validate_cmd: crate::cli::ValidateCommands) -> Result<(), String> {
    match validate_cmd {
        crate::cli::ValidateCommands::New { input, .. } => handle_validate_new(input),
        crate::cli::ValidateCommands::Update { music_root, .. } => {
            handle_validate_update(music_root)
        }
        crate::cli::ValidateCommands::NewMd { input, .. } => {
            handle_validate_new_md(input)
        }
    }
}

fn handle_validate_new(input_file: crate::cli::FilePathsFromCli) -> Result<(), String> {
    let files = input_file.try_into_file_paths()?;
    crate::validate::validate_new_input(&files)
}

fn handle_validate_update(
    music_lib: crate::cli::MusicLibraryCli,
) -> Result<(), String> {
    // 正常に読み込めた => 全ての動画が検証済み
    let _music_lib = music_lib.try_into_music_root_from_cli()?;
    Ok(())
}

fn handle_validate_new_md(input_file: crate::cli::FilePathsFromCli) -> ! {
    let files = match input_file.try_into_file_paths() {
        Ok(f) => f,
        Err(_e) => std::process::exit(1),
    };

    match crate::validate::validate_new_input_md(&files) {
        Ok(md_str) => {
            // TODO 設計考え直してもいい. 落とすのは何かなぁ~
            println!("{md_str}");
            std::process::exit(0)
        }
        Err(_e) => std::process::exit(1),
    }
}

// MARK: dev

fn handle_dev(dev_cmd: crate::cli::UtilCommands) -> Result<(), String> {
    match dev_cmd {
        crate::cli::UtilCommands::GenerateArtist {
            input_artists_data_path,
            artist_output_dir,
            search_index_file_name,
            channel_file_name,
            artists_file_name,
            music_data_code_snippets_path,
            ..
        } => crate::artist::generate(
            input_artists_data_path,
            artist_output_dir,
            search_index_file_name,
            channel_file_name,
            artists_file_name,
            music_data_code_snippets_path,
        )
        .map_err(|e| e.to_string()),
        crate::cli::UtilCommands::MergeFiles {
            files,
            dir,
            output_dir,
            ..
        } => handle_dev_merge_files(files, dir, output_dir),
        crate::cli::UtilCommands::DuplicateIds { ids, music_lib, .. } => {
            handle_dev_duplicate_ids(music_lib, ids)
        }
    }
}

fn handle_dev_merge_files(
    files: Option<crate::cli::FilePathsFromCli>,
    dir: String,
    output_dir: String,
) -> Result<(), String> {
    let files = files.map(|f| f.try_into_file_paths()).transpose()?;
    let dir = crate::util::DirPath::from_path_buf(std::path::PathBuf::from(dir))
        .map_err(|e| format!("Invalid directory path: {e}"))?;
    let output_dir =
        crate::util::DirPath::from_path_buf(std::path::PathBuf::from(output_dir))
            .map_err(|e| format!("Invalid output directory path: {e}"))?;

    let files = crate::dev::merge_files(files, &dir, output_dir)?;

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
    music_lib: crate::cli::MusicLibraryCli,
    ids: crate::cli::VideoIdsFromCli,
) -> Result<(), String> {
    let music_lib = music_lib.try_into_music_root_from_cli()?;
    let video_ids = ids.as_ids();

    println!(
        "Checking if any of the {} provided video IDs are already registered in the database.",
        video_ids.len()
    );

    let duplicates = crate::dev::find_video_ids(&music_lib, video_ids);

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
