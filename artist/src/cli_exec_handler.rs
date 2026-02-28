pub fn cli_exec_handler(cli: crate::cli::Cli) -> Result<(), String> {
    let args = cli.generate;
    crate::generate::generate(
        args.min_livers_output_dir,
        args.min_livers_search_index_file_name,
        args.min_channels_file_name,
        args.min_livers_file_name,
        args.music_code_snippets_path,
    )
    .map_err(|e| e.to_string())
}
