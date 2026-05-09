mod add;
mod error;
mod hash_inputs;
mod min;
mod sync;
mod update;
mod util;

pub use error::CliExecError;

pub async fn cli_exec_handler(cli: crate::cli::Cli) -> Result<(), CliExecError> {
    match cli.command {
        crate::cli::Commands::Add(new_cmd) => add::handle_add(new_cmd).await,
        crate::cli::Commands::Update(update_cmd) => update::handle_update(update_cmd),
        crate::cli::Commands::Sync(sync_cmd) => sync::handle_sync(sync_cmd).await,
        crate::cli::Commands::Build(build_cmd) => match build_cmd.mode {
            crate::cli::parser::BuildMode::Minify(minify_cmd) => {
                min::handle_minify(minify_cmd)
            }
            crate::cli::parser::BuildMode::HashInputs(hash_inputs_cmd) => {
                hash_inputs::handle_hash_inputs(hash_inputs_cmd)
            }
        },
        crate::cli::Commands::Util(util_cmd) => util::handle_util(util_cmd),
    }
}
