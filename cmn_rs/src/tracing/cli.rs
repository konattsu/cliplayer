mod cli_default_vals {
    use crate::tracing::TracingLevel;

    pub(super) const STDOUT_TRACING_LEVEL: TracingLevel = TracingLevel::Info;
}

fn default_stdout_tracing_level() -> crate::tracing::TracingLevel {
    cli_default_vals::STDOUT_TRACING_LEVEL
}

#[derive(Debug, clap::Args)]
pub struct CliTraceOps {
    /// Tracing level for file operations
    #[arg(long, value_name = "LEVEL", global = true)]
    pub file_tracing_level: Option<crate::tracing::TracingLevel>,
    /// Tracing level for stdout output
    #[arg(long, value_name = "LEVEL", global = true, default_value_t = default_stdout_tracing_level())]
    pub stdout_tracing_level: crate::tracing::TracingLevel,
    /// If set, suppress stdout tracing output
    #[arg(long, short, action = clap::ArgAction::SetTrue, global = true)]
    pub quiet: bool,
}
