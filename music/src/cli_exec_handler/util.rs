pub(super) fn handle_util(cmd: crate::cli::parser::UtilCommands) -> Result<(), ()> {
    use crate::cli::parser::UtilMode;
    match cmd.mode {
        UtilMode::Find(args) => handle_find(args),
        UtilMode::Merge(args) => handle_merge(args),
    }
}

fn handle_find(args: crate::cli::parser::FindDuplicateIdsArgs) -> Result<(), ()> {
    use crate::model::{VideoId, VideoIds};
    use crate::music_file::MusicLibrary;
    use std::collections::HashSet;

    let music_lib = MusicLibrary::load(args.music_root_dir.as_ref())
        .map_err(|e| tracing::error!("Failed to load music library: {e}"))?;

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

fn handle_merge(args: crate::cli::parser::MergeFilesArgs) -> Result<(), ()> {
    use std::path::PathBuf;
    use tracing::error;

    let input_dir = args.input_dir.as_ref();
    let input_files = collect_json_files_in_dir(input_dir).map_err(|e| {
        error!(
            "Failed to collect json files under {}: {}",
            input_dir.display(),
            e
        )
    })?;

    let anonymous_videos = crate::validate::try_load_anonymous_videos(&input_files)
        .map_err(|e| error!("{e}"))?;

    let output_file = PathBuf::from(args.output_dir).join(format!(
        "{}.json",
        chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S")
    ));

    let file = std::fs::File::create(&output_file).map_err(|e| {
        error!(
            "Failed to create output file {}: {}",
            output_file.display(),
            e
        )
    })?;

    serde_json::to_writer_pretty(file, &anonymous_videos)
        .map_err(|e| error!("Failed to write: {e}"))?;

    dialoguer::Confirm::new()
        .with_prompt("Do you want to remove the original files?")
        .interact()
        .map_err(|e| error!("Failed to read user input: {e}"))?
        .then(|| {
            for file in input_files {
                if let Err(e) = std::fs::remove_file(&file) {
                    error!("Failed to remove file {}: {}", file.display(), e);
                }
            }
        });
    Ok(())
}

/// 特定のディレクトリ以下の`*.json`ファイルのパスを集める
///
/// - 再帰的でない
fn collect_json_files_in_dir(
    dir: &std::path::Path,
) -> Result<Vec<std::path::PathBuf>, String> {
    let mut json_files = Vec::new();

    for entry in
        std::fs::read_dir(dir).map_err(|e| format!("Failed to read directory: {e}"))?
    {
        let entry =
            entry.map_err(|e| format!("Failed to read directory entry: {e}"))?;
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
