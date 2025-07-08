/// ディレクトリを表す構造体
///
/// - ディレクトリが存在することを保証
#[derive(Debug, Clone)]
pub struct DirPath(std::path::PathBuf);

impl DirPath {
    pub fn new(path: &str) -> Result<Self, String> {
        let path_buf = std::path::PathBuf::from(path);
        Self::is_dir(&path_buf)?;
        Ok(DirPath(path_buf))
    }

    pub fn into_path_buf(self) -> std::path::PathBuf {
        self.0
    }

    fn is_dir(path: &std::path::Path) -> Result<(), String> {
        if path.is_dir() {
            Ok(())
        } else if path.exists() {
            Err(format!(
                "Path is exists but not a directory: {}",
                path.display()
            ))
        } else {
            Err(format!("Path does not exist: {}", path.display()))
        }
    }
}

impl std::str::FromStr for DirPath {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DirPath::new(s)
    }
}

impl std::fmt::Display for DirPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}
