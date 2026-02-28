fn main() {
    use clap::Parser;

    let cli = artistctl::cli::Cli::parse();
    let _tracing_guard =
        cmn_rs::tracing::apply_tracing_settings(cli.stdout_level(), cli.file_level());
    tracing::debug!("Command line arguments: {:?}", cli);

    let res = artistctl::cli_exec_handler::cli_exec_handler(cli);

    if let Err(e) = res {
        tracing::debug!("Error occurred: {e}");
        eprintln!("Error: {e}");
        std::process::exit(1);
    } else {
        println!("Command executed successfully.");
    }
}
