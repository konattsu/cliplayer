#[derive(Debug, clap::Args)]
pub struct CliTraceLevel {
    /// Tracing level for file operations
    #[arg(long, value_name = "LEVEL")]
    pub file_tracing_level: Option<crate::tracing::TracingLevel>,
    /// Tracing level for stdout output
    #[arg(long, value_name = "LEVEL")]
    pub stdout_tracing_level: Option<crate::tracing::TracingLevel>,
}
