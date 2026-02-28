pub fn cli_exec_handler(cli: crate::cli::Cli) -> Result<(), String> {
    let args = cli.generate;
    crate::generate::generate(
        args.artist_output_dir,
        args.search_index_file_name,
        args.channel_file_name,
        args.artists_file_name,
        args.music_data_code_snippets_path,
    )
    .map_err(|e| e.to_string())
}
