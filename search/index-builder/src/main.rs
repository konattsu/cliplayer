fn main() {
    use clap::Parser;

    let cli = index_builder::cli::Cli::parse();
    let _tracing_guard = cmn_rs::tracing::apply_tracing_settings(
        "index-builder",
        cli.stdout_level(),
        cli.file_level(),
        cli.is_quiet(),
    );
    tracing::debug!("Command line arguments: {:?}", cli);

    if let Err(error) = index_builder::cli_exec_handler::cli_exec_handler(cli) {
        tracing::error!("Command failed: {error}");
        std::process::exit(1);
    }
}
