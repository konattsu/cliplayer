/// 主にコマンドライン引数からファイルパスを受け取るための型
#[derive(Debug, Clone)]
pub struct FilePathsFromCli(Vec<String>);

impl FilePathsFromCli {
    pub fn try_into_vec(self) -> Result<Vec<crate::util::FilePath>, String> {
        self.0
            .into_iter()
            .map(|path| crate::util::FilePath::new(&std::path::PathBuf::from(path)))
            .collect()
    }
}

impl std::fmt::Display for FilePathsFromCli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let paths_str = self
            .0
            .iter()
            .map(|path| path.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{paths_str}")
    }
}

impl std::str::FromStr for FilePathsFromCli {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let paths: Vec<String> = s
            .split(|c| ",; \t\n\r".contains(c))
            .filter(|path| !path.is_empty())
            .map(|path| path.trim().to_string())
            .collect();
        if paths.is_empty() {
            return Err("FilePaths cannot be empty");
        }
        Ok(FilePathsFromCli(paths))
    }
}
