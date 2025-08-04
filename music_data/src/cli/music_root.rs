/// 主にコマンドライン引数から音楽情報のルートフォルダを受け取るための型
#[derive(Debug, Clone)]
pub struct MusicLibraryCli(String);

impl MusicLibraryCli {
    /// CLIから受け取った音楽情報のルートディレクトリを`MusicRoot`に変換
    ///
    /// Err(String)の時はエラーが綺麗に表示された文字列が返る
    pub fn try_into_music_root_from_cli(
        &self,
        min_videos_path: super::OutputMinPathFromCli,
        min_flat_clips_path: super::OutputMinClipsPathFromCli,
    ) -> Result<crate::music_file::MusicLibrary, String> {
        let min_videos_path = min_videos_path.try_into_path()?;
        let min_flat_clips_path = min_flat_clips_path.try_into_path()?;

        self.clone()
            .try_into_music_root(&min_videos_path, &min_flat_clips_path)
    }

    fn try_into_music_root(
        self,
        min_videos_path: &crate::util::FilePath,
        min_flat_clips_path: &crate::util::FilePath,
    ) -> Result<crate::music_file::MusicLibrary, String> {
        let path = crate::util::DirPath::new(&std::path::PathBuf::from(self.0))?;
        crate::music_file::MusicLibrary::load(
            &path,
            min_videos_path,
            min_flat_clips_path,
        )
        .map_err(|e| e.to_pretty_string())
    }
}

impl Default for MusicLibraryCli {
    fn default() -> Self {
        use std::str::FromStr;
        const DEFAULT_MUSIC_ROOT: &str = "./data/music";

        MusicLibraryCli::from_str(DEFAULT_MUSIC_ROOT).unwrap_or_else(|_| {
            panic!("!!! Default music root path({DEFAULT_MUSIC_ROOT}) is invalid !!!")
        })
    }
}

impl std::fmt::Display for MusicLibraryCli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for MusicLibraryCli {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("MusicRoot cannot be empty");
        }
        Ok(MusicLibraryCli(s.to_string()))
    }
}
