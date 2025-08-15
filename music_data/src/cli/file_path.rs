/// 主にコマンドライン引数からファイルパスを受け取るための型
#[derive(Debug, Clone)]
pub struct FilePathFromCli(String);

impl FilePathFromCli {
    /// コマンドライン引数から受け取ったファイルパスを`FilePath`に変換
    pub fn try_into_file_path(self) -> Result<crate::util::FilePath, String> {
        crate::util::FilePath::new(std::path::Path::new(&self.0))
            .map_err(|e| format!("Failed to parse file path: {e}"))
    }

    pub(super) fn new_unchecked(path: String) -> Self {
        FilePathFromCli(path)
    }
}

impl std::fmt::Display for FilePathFromCli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for FilePathFromCli {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = s.trim().to_string();
        Ok(FilePathFromCli(path))
    }
}
