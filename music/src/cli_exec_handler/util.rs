pub(super) fn handle_util(
    cmd: crate::cli::parser::UtilCommands,
) -> Result<(), crate::cli_exec_handler::CliExecError> {
    use crate::cli::parser::UtilMode;
    match cmd.mode {
        UtilMode::Find(args) => handle_find(args),
        UtilMode::Merge(args) => handle_merge(args),
    }
}

fn handle_find(
    args: crate::cli::parser::FindDuplicateIdsArgs,
) -> Result<(), crate::cli_exec_handler::CliExecError> {
    use crate::model::{VideoId, VideoIds};
    use crate::music_file::MusicLibrary;
    use std::collections::HashSet;

    let music_lib = MusicLibrary::load(args.music_root_dir.as_ref())?;

    let video_ids_in_lib: HashSet<VideoId> =
        music_lib.get_video_ids().into_iter().collect();

    let duplicated_ids: Vec<VideoId> = args
        .ids
        .as_ids()
        .iter()
        .filter_map(|id| video_ids_in_lib.get(id))
        .cloned()
        .collect();

    if duplicated_ids.is_empty() {
        println!("No duplicate video IDs found.");
    } else {
        println!(
            "Duplicate video IDs found: {}",
            VideoIds::from(duplicated_ids)
        );
    }

    Ok(())
}

fn handle_merge(
    args: crate::cli::parser::MergeFilesArgs,
) -> Result<(), crate::cli_exec_handler::CliExecError> {
    use std::path::PathBuf;

    let input_dir = args.input_dir.as_ref();
    let input_files = collect_json_files_in_dir(input_dir)?;

    let anonymous_videos = crate::validate::try_load_anonymous_videos(&input_files)?;

    let output_file = PathBuf::from(args.output_dir).join(format!(
        "{}.json",
        chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S")
    ));

    let file = std::fs::File::create(&output_file)?;

    serde_json::to_writer_pretty(file, &anonymous_videos)?;

    if ask_for_removal_confirmation()? {
        for file in input_files {
            if let Err(e) = std::fs::remove_file(&file) {
                tracing::warn!(
                    "Failed to remove source file {}: {}",
                    file.display(),
                    e
                );
            }
        }
    }
    Ok(())
}

fn ask_for_removal_confirmation() -> Result<bool, crate::cli_exec_handler::CliExecError>
{
    dialoguer::Confirm::new()
        .with_prompt("Do you want to remove the original files?")
        .interact()
        .map_err(Into::into)
}

/// 特定のディレクトリ以下の`*.json`ファイルのパスを集める
///
/// - 再帰的でない
fn collect_json_files_in_dir(
    dir: &std::path::Path,
) -> Result<Vec<std::path::PathBuf>, crate::cli_exec_handler::CliExecError> {
    let mut json_files = Vec::new();

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.eq_ignore_ascii_case("json"))
            .unwrap_or(false)
        {
            json_files.push(path);
        }
    }
    json_files.sort_unstable();
    Ok(json_files)
}
