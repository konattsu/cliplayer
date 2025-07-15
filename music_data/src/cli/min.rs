/// 主にコマンドライン引数からminファイルの出力先のパス受け取るための型
#[derive(Debug, Clone)]
pub struct OutputMinPathFromCli(String);

impl OutputMinPathFromCli {
    pub fn try_into_path(self) -> Result<crate::util::FilePath, String> {
        crate::util::FilePath::new(&std::path::PathBuf::from(self.0))
    }
}

impl Default for OutputMinPathFromCli {
    fn default() -> Self {
        use std::str::FromStr;
        const DEFAULT_MIN_PATH: &str = "../public/music_data/music.min.json";

        OutputMinPathFromCli::from_str(DEFAULT_MIN_PATH).unwrap_or_else(|_| {
            panic!("!!! Default output min path({DEFAULT_MIN_PATH}) is invalid !!!")
        })
    }
}

impl std::fmt::Display for OutputMinPathFromCli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for OutputMinPathFromCli {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("Output min file cannot be empty");
        }
        Ok(OutputMinPathFromCli(s.to_string()))
    }
}
