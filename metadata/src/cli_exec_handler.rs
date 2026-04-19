pub fn cli_exec_handler(cli: crate::cli::Cli) -> Result<(), String> {
    match cli.command {
        crate::cli::Command::Artist(args) => artistctl::generate::generate(
            args.output_dir,
            args.min_livers_search_index_file_name,
            args.min_channels_file_name,
            args.min_livers_file_name,
            args.min_official_channels_file_name,
            args.music_code_snippets_path,
        )
        .map_err(|error| error.to_string()),
        crate::cli::Command::Tag(args) => {
            tagctl::generate::generate(args.code_snippets_path)
                .map_err(|error| error.to_string())
        }
    }
}
