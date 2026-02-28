/// 主にコマンドライン引数からファイルパスを受け取るための型
#[derive(Debug, Clone)]
pub struct FilePathFromCli(String);

impl FilePathFromCli {
    pub fn into_file_path(self) -> std::path::PathBuf {
        self.0.into()
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
