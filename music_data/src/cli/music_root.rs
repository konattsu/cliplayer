/// 主にコマンドライン引数から音楽情報のルートフォルダを受け取るための型
#[derive(Debug, Clone)]
pub struct MusicRootFromCli(String);

impl MusicRootFromCli {
    /// CLIから受け取った音楽情報のルートディレクトリを`MusicRoot`に変換
    ///
    /// Err(String)の時はエラーが綺麗に表示された文字列が返る
    pub fn try_into_music_root(self) -> Result<crate::music_file::MusicRoot, String> {
        let path = crate::util::DirPath::new(&std::path::PathBuf::from(self.0))?;
        crate::music_file::MusicRoot::new(&path).map_err(|e| e.to_pretty_string())
    }
}

impl Default for MusicRootFromCli {
    fn default() -> Self {
        use std::str::FromStr;
        const DEFAULT_MUSIC_ROOT: &str = "./data/music";

        MusicRootFromCli::from_str(DEFAULT_MUSIC_ROOT).unwrap_or_else(|_| {
            panic!("!!! Default music root path({DEFAULT_MUSIC_ROOT}) is invalid !!!")
        })
    }
}

impl std::fmt::Display for MusicRootFromCli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for MusicRootFromCli {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("MusicRoot cannot be empty");
        }
        Ok(MusicRootFromCli(s.to_string()))
    }
}
