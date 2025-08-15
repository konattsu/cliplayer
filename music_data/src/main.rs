#[tokio::main]
async fn main() {
    use clap::Parser;

    let cli = musictl::cli::Cli::parse();
    let _tracing_guard = enable_tracing_log(&cli);
    tracing::debug!("Command line arguments: {:?}", cli);

    let res = musictl::cli_exec_handler::cli_exec_handler(cli).await;

    if let Err(e) = res {
        tracing::debug!("Error occurred: {e}");
        eprintln!("Error: {e}");
        std::process::exit(1);
    } else {
        println!("Command executed successfully.");
    }
}

fn enable_tracing_log(
    cli: &musictl::cli::Cli,
) -> tracing_appender::non_blocking::WorkerGuard {
    musictl::util::apply_tracing_settings(cli.stdout_level(), cli.file_level())
}
