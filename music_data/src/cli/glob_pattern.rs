#[derive(Debug, Clone)]
pub struct GlobPattern(String);

#[derive(Debug, thiserror::Error)]
pub enum GlobPatternError {
    #[error("Failed to parse glob pattern: {0}")]
    InvalidPattern(String),
    #[error("Failed to access matched files: {0}")]
    AccessError(String),
}

impl GlobPattern {
    pub fn matched_files(&self) -> Result<Vec<String>, GlobPatternError> {
        let mut files = Vec::new();
        for entry in glob::glob(&self.0)
            .map_err(|e| GlobPatternError::InvalidPattern(e.to_string()))?
        {
            match entry {
                Ok(path) => files.push(path.to_string_lossy().to_string()),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Ok(files)
    }
}

impl std::fmt::Display for GlobPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for GlobPattern {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(GlobPattern(s.to_owned()))
    }
}
