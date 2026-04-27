#[tokio::main]
async fn main() {
    use clap::Parser;

    let cli = musictl::cli::Cli::parse();
    let _tracing_guard = enable_tracing_log(&cli);
    tracing::debug!("Command line arguments: {:?}", cli);

    if let Err(e) = musictl::cli_exec_handler::cli_exec_handler(cli).await {
        tracing::error!("Command failed: {e}");
        std::process::exit(1);
    }
    tracing::info!("Command executed successfully.");
}

fn enable_tracing_log(
    cli: &musictl::cli::Cli,
) -> tracing_appender::non_blocking::WorkerGuard {
    cmn_rs::tracing::apply_tracing_settings(
        "musictl",
        cli.stdout_level(),
        cli.file_level(),
        cli.is_quiet(),
    )
}
