pub fn cli_exec_handler(cli: crate::cli::Cli) -> Result<(), String> {
    match cli.command {
        crate::cli::Command::Artist(args) => {
            use crate::cli::Operation;

            let res = match args.operation {
                Operation::GenerateSnippet => {
                    artistctl::generate::generate_snippet(args.music_code_snippets_path)
                }
                Operation::Minify => artistctl::generate::minify(
                    args.output_dir,
                    args.min_livers_search_index_file_name,
                    args.min_channels_file_name,
                    args.min_livers_file_name,
                    args.min_official_channels_file_name,
                ),
                Operation::All => artistctl::generate::generate(
                    args.output_dir,
                    args.min_livers_search_index_file_name,
                    args.min_channels_file_name,
                    args.min_livers_file_name,
                    args.min_official_channels_file_name,
                    args.music_code_snippets_path,
                ),
            };

            res.map_err(|error| error.to_string())
        }
        crate::cli::Command::Tag(args) => {
            use crate::cli::Operation;

            let res = match args.operation {
                Operation::GenerateSnippet => {
                    tagctl::generate::generate_snippet(args.code_snippets_path)
                }
                Operation::Minify => {
                    tagctl::generate::minify(args.output_dir, args.min_tags_file_name)
                }
                Operation::All => tagctl::generate::generate(
                    args.output_dir,
                    args.min_tags_file_name,
                    args.code_snippets_path,
                ),
            };

            res.map_err(|error| error.to_string())
        }
    }
}
