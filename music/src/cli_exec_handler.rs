mod add;
mod sync;
mod update;
mod util;

// 将来的にここ定義して柔軟に色々エラーを扱いたいという気持ち
// pub struct CliExecError {
//     //
// }

pub async fn cli_exec_handler(cli: crate::cli::Cli) -> Result<(), ()> {
    match cli.command {
        crate::cli::Commands::Add(new_cmd) => add::handle_add(new_cmd).await,
        crate::cli::Commands::Update(update_cmd) => update::handle_update(update_cmd),
        crate::cli::Commands::Sync(sync_cmd) => sync::handle_sync(sync_cmd).await,
        crate::cli::Commands::Util(util_cmd) => util::handle_util(util_cmd),
    }
}
