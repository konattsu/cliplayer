/// 音楽情報のルートフォルダであることを保証する
#[derive(Debug, Clone)]
pub struct MusicRoot {
    root: crate::util::DirPath,
    files: Vec<MusicFilePath>,
}

/// 音楽情報のファイルパス
#[derive(Debug, Clone)]
pub struct MusicFilePath {
    year: usize,
    month: usize,
    file_path: crate::util::FilePath,
}

/// 音楽情報のルートディレクトリの内容
#[derive(Debug, Clone)]
pub struct MusicRootContent {
    root: crate::util::DirPath,
    files: Vec<MusicFilePathContent>,
}

/// 音楽情報のファイルパスの内容
#[derive(Debug, Clone)]
pub struct MusicFilePathContent {
    year: usize,
    month: usize,
    file_path: crate::util::FilePath,
    content: crate::model::VerifiedVideos,
}

/// `MusicFileError`をまとめたもの
///
/// 変換に関して制約はないので, 相互変換ご自由に
#[derive(Debug, Clone)]
pub struct MusicFileErrors {
    pub errs: Vec<MusicFileError>,
}

/// 音楽情報のファイルに関するエラー
#[derive(Debug, Clone)]
pub enum MusicFileError {
    // 月ファイルのエラー
    MonthFileError {
        underlying: String,
        msg: String,
    },
    // 年フォルダのエラー
    YearFolderError(String),
    // ディレクトリの読み込み失敗
    ReadDirError {
        dir: String,
        msg: String,
    },
    // ファイルの読み込み失敗
    FileReadError {
        path: crate::util::FilePath,
        msg: String,
        when: String,
    },
    FileWriteError {
        path: crate::util::FilePath,
        msg: String,
    },
    // ファイルの内容が不正
    InvalidFileContent {
        path: crate::util::FilePath,
        msg: String,
    },
    // 対応する年月ファイルが存在しない
    NonExistentMonthFile {
        year: usize,
        month: usize,
        id: crate::model::VideoId,
    },
}

impl MusicRoot {
    /// MusicRootを作成
    ///
    /// # Arguments:
    /// - `path`: 音楽情報のルートディレクトリパス
    ///
    /// # Errors:
    /// - ディレクトリ構造が不正な場合
    pub fn new(path: &crate::util::DirPath) -> Result<Self, MusicFileErrors> {
        let entries: Vec<std::fs::DirEntry> =
            super::fs::read_dir(path.as_path()).map_err(|e| e.into_errors())?;
        super::validate::collect_and_validate_year_directories(&entries).map(|files| {
            Self {
                root: path.clone(),
                files,
            }
        })
    }

    pub fn get_file_paths(&self) -> &[MusicFilePath] {
        &self.files
    }
}

impl MusicFilePath {
    pub fn new(year: usize, month: usize, file_path: crate::util::FilePath) -> Self {
        Self {
            year,
            month,
            file_path,
        }
    }

    pub fn get_year(&self) -> usize {
        self.year
    }

    pub fn get_month(&self) -> usize {
        self.month
    }

    pub fn get_file_path(&self) -> &crate::util::FilePath {
        &self.file_path
    }
}

impl MusicRootContent {
    /// 楽曲情報をファイルから読み取る
    pub fn load(root: &MusicRoot) -> Result<Self, MusicFileErrors> {
        let mut errs: Vec<MusicFileError> = Vec::new();
        let mut contents: Vec<MusicFilePathContent> = Vec::new();

        for file in root.files.iter() {
            match super::fs::deserialize_from_file(file.get_file_path()) {
                Ok(content) => contents.push(MusicFilePathContent {
                    year: file.get_year(),
                    month: file.get_month(),
                    file_path: file.get_file_path().clone(),
                    content,
                }),
                Err(e) => errs.push(e),
            }
        }

        if errs.is_empty() {
            Ok(Self {
                root: root.root.clone(),
                files: contents,
            })
        } else {
            Err(MusicFileErrors { errs })
        }
    }

    /// 楽曲情報をファイルに書き込む
    pub fn write(&self) -> Result<(), MusicFileErrors> {
        let mut errs: Vec<MusicFileError> = Vec::new();

        for file in self.files.iter() {
            match super::fs::serialize_to_file(&file.file_path, &file.content) {
                Ok(_ok) => (),
                Err(e) => {
                    errs.push(e);
                }
            }
        }

        if errs.is_empty() {
            Ok(())
        } else {
            Err(MusicFileErrors { errs })
        }
    }

    /// 動画を追加
    ///
    /// # Errors:
    /// - 引数の動画に対応する年月のファイルが存在しない場合
    pub fn append_video(
        &mut self,
        video: crate::model::VerifiedVideo,
    ) -> Result<(), MusicFileError> {
        let (year, month) = (video.get_year(), video.get_month());
        let file = match self.get_file_path_mut(year, month) {
            Some(file) => file,
            None => {
                return Err(MusicFileError::NonExistentMonthFile {
                    year,
                    month,
                    id: video.get_video_id().clone(),
                });
            }
        };

        file.push_content(video);
        Ok(())
    }

    /// 与えられた年/月に対応する音楽ファイルパスを返す
    fn get_file_path_mut(
        &mut self,
        year: usize,
        month: usize,
    ) -> Option<&mut MusicFilePathContent> {
        self.files
            .iter_mut()
            .find(|f| f.year == year && f.month == month)
    }

    /// 内部の動画をflat化する
    pub fn into_flattened_files(self) -> crate::model::VerifiedVideos {
        let vec_videos: Vec<crate::model::VerifiedVideo> = self
            .files
            .into_iter()
            .flat_map(|f| f.content.into_vec())
            .collect();
        crate::model::VerifiedVideos::new(vec_videos)
    }
}

impl MusicFilePathContent {
    pub fn push_content(&mut self, content: crate::model::VerifiedVideo) {
        self.content.push_video(content);
    }
}

impl MusicFileErrors {
    /// エラーメッセージを整形して返す
    ///
    /// 文字列の最後に`\n`が付与される
    pub fn to_pretty_string(&self) -> String {
        format!(
            "{}\n",
            self.errs
                .iter()
                .map(|e| e.to_pretty_string())
                .collect::<String>()
        )
    }
}

impl MusicFileError {
    /// エラーメッセージを整形して返す
    ///
    /// 文字列の最後に`\n`が付与される
    pub fn to_pretty_string(&self) -> String {
        match self {
            Self::MonthFileError { underlying, msg } => {
                format!("Month file error in {underlying}: {msg}\n")
            }
            Self::YearFolderError(msg) => {
                format!("Year folder error: {msg}\n")
            }
            Self::ReadDirError { dir, msg } => {
                format!("Failed to read directory {dir}: {msg}\n")
            }
            Self::FileReadError { path, msg, when } => {
                format!("Failed to read file {path} when {when}: {msg}\n")
            }
            Self::FileWriteError { path, msg } => {
                format!("Failed to write file {path}: {msg}\n")
            }
            Self::InvalidFileContent { path, msg } => {
                format!("Invalid content in file {path}: {msg}\n")
            }
            Self::NonExistentMonthFile { year, month, id } => {
                format!(
                    "No corresponding file for this video(id: {id}) in {year}/{month}\n"
                )
            }
        }
    }

    pub fn into_errors(self) -> MusicFileErrors {
        MusicFileErrors { errs: vec![self] }
    }
}
