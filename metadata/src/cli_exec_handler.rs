pub fn cli_exec_handler(cli: crate::cli::Cli) -> Result<(), String> {
    match cli.command {
        crate::cli::Command::Artist(command) => match command.mode {
            crate::cli::ArtistMode::Minify(args) => artistctl::generate::minify(
                args.output_dir,
                args.min_livers_search_index_file_name,
                args.min_channels_file_name,
                args.min_livers_file_name,
                args.min_official_channels_file_name,
                args.dataset_build_id,
            )
            .map_err(|error| error.to_string()),
            crate::cli::ArtistMode::Snippet(args) => {
                artistctl::generate::snippet(args.music_code_snippets_path)
                    .map_err(|error| error.to_string())
            }
            crate::cli::ArtistMode::HashInputs => {
                let hash = artistctl::generate::hash_inputs()
                    .map_err(|error| error.to_string())?;
                println!("{hash}");
                Ok(())
            }
        },
        crate::cli::Command::Tag(command) => match command.mode {
            crate::cli::TagMode::Minify(args) => tagctl::generate::minify(
                args.output_dir,
                args.min_tags_file_name,
                args.dataset_build_id,
            )
            .map_err(|error| error.to_string()),
            crate::cli::TagMode::Snippet(args) => {
                tagctl::generate::snippet(args.code_snippets_path)
                    .map_err(|error| error.to_string())
            }
            crate::cli::TagMode::HashInputs => {
                let hash = tagctl::generate::hash_inputs()
                    .map_err(|error| error.to_string())?;
                println!("{hash}");
                Ok(())
            }
        },
    }
}
