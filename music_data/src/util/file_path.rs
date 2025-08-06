/// ファイルを表す構造体
///
/// - ファイルが存在することを保証
#[derive(Debug, Clone, PartialEq)]
pub struct FilePath(std::path::PathBuf);

impl FilePath {
    pub fn new(path: &std::path::Path) -> Result<Self, String> {
        Self::is_file(path)?;
        Ok(FilePath(path.to_owned()))
    }

    pub fn from_path_buf(path: std::path::PathBuf) -> Result<Self, String> {
        Self::is_file(&path)?;
        Ok(FilePath(path))
    }

    pub fn as_path(&self) -> &std::path::Path {
        &self.0
    }

    pub fn into_path_buf(self) -> std::path::PathBuf {
        self.0
    }

    fn is_file(path: &std::path::Path) -> Result<(), String> {
        if path.is_file() {
            Ok(())
        } else if path.exists() {
            Err(format!("Path is exists but not a file: {}", path.display()))
        } else {
            Err(format!("Path does not exist: {}", path.display()))
        }
    }
}

impl std::str::FromStr for FilePath {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        FilePath::new(std::path::Path::new(s))
    }
}

impl std::fmt::Display for FilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

// MARK: For Tests

#[cfg(test)]
impl FilePath {
    /// テスト用. 存在を確認せずにFilePathを生成
    pub(crate) fn new_uncheck_existence(path: &std::path::Path) -> Self {
        FilePath(path.to_owned())
    }
}
