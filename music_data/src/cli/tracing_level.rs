#[derive(Debug, Clone)]
pub enum TracingLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl TracingLevel {
    pub fn into_tracing_level_filter(self) -> tracing::level_filters::LevelFilter {
        match self {
            TracingLevel::Error => tracing::level_filters::LevelFilter::ERROR,
            TracingLevel::Warn => tracing::level_filters::LevelFilter::WARN,
            TracingLevel::Info => tracing::level_filters::LevelFilter::INFO,
            TracingLevel::Debug => tracing::level_filters::LevelFilter::DEBUG,
            TracingLevel::Trace => tracing::level_filters::LevelFilter::TRACE,
        }
    }
}

impl std::str::FromStr for TracingLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "error" => Ok(TracingLevel::Error),
            "warn" => Ok(TracingLevel::Warn),
            "info" => Ok(TracingLevel::Info),
            "debug" => Ok(TracingLevel::Debug),
            "trace" => Ok(TracingLevel::Trace),
            _ => Err(format!("Invalid tracing level: {}", s)),
        }
    }
}

impl std::fmt::Display for TracingLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let level_str = match self {
            TracingLevel::Error => "error",
            TracingLevel::Warn => "warn",
            TracingLevel::Info => "info",
            TracingLevel::Debug => "debug",
            TracingLevel::Trace => "trace",
        };
        write!(f, "{}", level_str)
    }
}
