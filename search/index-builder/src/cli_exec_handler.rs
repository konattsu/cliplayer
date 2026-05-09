pub fn cli_exec_handler(cli: crate::cli::Cli) -> anyhow::Result<()> {
    handle_build(cli.build)
}

fn handle_build(args: crate::cli::BuildArgs) -> anyhow::Result<()> {
    let output_path = std::path::PathBuf::from(&args.output_path);

    if let Some(parent) = output_path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)?;
    }

    let binary = crate::build::build_search_index_binary(
        std::path::Path::new(&args.music_root_dir),
        args.dataset_build_id,
    )?;
    std::fs::write(&output_path, binary)?;

    tracing::info!(output_path = %output_path.display(), "search index written");
    Ok(())
}
