fn main() {
    use clap::Parser;

    let cli = metadata::cli::Cli::parse();
    let _tracing_guard = cmn_rs::tracing::apply_tracing_settings(
        "metadata",
        cli.stdout_level(),
        cli.file_level(),
        cli.is_quiet(),
    );
    tracing::debug!("Command line arguments: {:?}", cli);

    let res = metadata::cli_exec_handler::cli_exec_handler(cli);

    if let Err(error) = res {
        tracing::debug!("Error occurred: {error}");
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}
