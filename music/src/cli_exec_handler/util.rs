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
    let music_lib = crate::music_file::MusicLibraryRepository::load(
        args.music_root.music_root_dir.as_path(),
    )?;
    let duplicated_ids =
        crate::operations::find_duplicate_video_ids(&music_lib, args.ids.as_ids());

    if duplicated_ids.is_empty() {
        println!("No duplicate video IDs found.");
    } else {
        println!("Duplicate video IDs found: {}", duplicated_ids);
    }

    Ok(())
}

fn handle_merge(
    args: crate::cli::parser::MergeFilesArgs,
) -> Result<(), crate::cli_exec_handler::CliExecError> {
    let result = crate::operations::merge_input_files(
        args.directories.input_dir.as_path(),
        args.directories.output_dir.as_path(),
        args.remove_source_files,
    )?;
    println!("Merged file written to {}", result.output_file.display());
    Ok(())
}
