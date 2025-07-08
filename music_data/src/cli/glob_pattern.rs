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
    /// GlobPattern にマッチしたファイルを返却
    ///
    /// Error:
    /// - `InvalidPattern`: globパターンが無効
    /// - `AccessError`: globパターンにマッチしたファイルへのアクセスに失敗
    pub fn matched_files(&self) -> Result<Vec<std::path::PathBuf>, GlobPatternError> {
        let mut files = Vec::new();
        for entry in glob::glob(&self.0)
            .map_err(|e| GlobPatternError::InvalidPattern(e.to_string()))?
        {
            match entry {
                Ok(path) => files.push(path),
                Err(e) => {
                    tracing::error!(
                        "Failed to access file: {}, given patten: {}",
                        e,
                        self.0
                    );
                    return Err(GlobPatternError::AccessError(e.to_string()));
                }
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
