/// 主にコマンドライン引数からmin_clipsファイルの出力先のパス受け取るための型
#[derive(Debug, Clone)]
pub struct OutputMinClipsPathFromCli(String);

impl OutputMinClipsPathFromCli {
    pub fn try_into_path(self) -> Result<crate::util::FilePath, String> {
        crate::util::FilePath::new(&std::path::PathBuf::from(self.0))
    }
}

impl Default for OutputMinClipsPathFromCli {
    fn default() -> Self {
        use std::str::FromStr;
        const DEFAULT_MIN_PATH: &str = "../public/music_data/clips.min.json";

        OutputMinClipsPathFromCli::from_str(DEFAULT_MIN_PATH).unwrap_or_else(|_| {
            panic!("!!! Default output min path({DEFAULT_MIN_PATH}) is invalid !!!")
        })
    }
}

impl std::fmt::Display for OutputMinClipsPathFromCli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for OutputMinClipsPathFromCli {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("Output min clips file cannot be empty");
        }
        Ok(OutputMinClipsPathFromCli(s.to_string()))
    }
}
