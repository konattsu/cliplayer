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
    // 一応持っておく
    _root: crate::util::DirPath,
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

impl MusicRoot {
    /// MusicRootを作成
    ///
    /// # Arguments:
    /// - `path`: 音楽情報のルートディレクトリパス
    ///
    /// # Errors:
    /// - ディレクトリ構造が不正な場合
    pub fn new(path: &crate::util::DirPath) -> Result<Self, super::MusicFileErrors> {
        let entries: Vec<std::fs::DirEntry> =
            super::fs_util::read_dir(path.as_path()).map_err(|e| e.into_errors())?;
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
    pub fn load(root: &MusicRoot) -> Result<Self, super::MusicFileErrors> {
        let mut errs: Vec<super::MusicFileError> = Vec::new();
        let mut contents: Vec<MusicFilePathContent> = Vec::new();

        for file in root.files.iter() {
            match super::fs_util::deserialize_from_file(file.get_file_path()) {
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
                _root: root.root.clone(),
                files: contents,
            })
        } else {
            Err(super::MusicFileErrors { errs })
        }
    }

    /// 楽曲情報をファイルに書き込む
    ///
    /// 書き込めなかったらスキップする. 全てのファイルに書き込もうとする
    pub fn write(&self) -> Result<(), super::MusicFileErrors> {
        let mut errs: Vec<super::MusicFileError> = Vec::new();

        for file in self.files.iter() {
            match super::fs_util::serialize_to_file(&file.file_path, &file.content) {
                Ok(_ok) => (),
                Err(e) => {
                    errs.push(e);
                }
            }
        }

        if errs.is_empty() {
            Ok(())
        } else {
            Err(super::MusicFileErrors { errs })
        }
    }

    /// 楽曲情報をmin化して書き込む
    pub fn write_minified(
        self,
        min_path: &crate::util::FilePath,
    ) -> Result<(), super::MusicFileError> {
        let content = self.into_flattened_files();
        super::fs_util::serialize_to_file_min(min_path, &content)
    }

    /// クリップ情報をフラット化したものを書き込む
    pub fn write_flat_clips(
        self,
        min_flat_path: &crate::util::FilePath,
    ) -> Result<(), super::MusicFileError> {
        let clips: Vec<crate::model::VerifiedClip> = self
            .into_flattened_files()
            .into_inner()
            .into_iter()
            .flat_map(|v| v.into_clips())
            .collect();
        let clips = crate::model::FlatClips::new(clips);
        super::fs_util::serialize_to_file_min(min_flat_path, &clips)
    }

    /// 動画を追加
    ///
    /// # Errors:
    /// - 引数の動画に対応する年月のファイルが存在しない場合
    ///   - 途中でエラーが見つかっても, 引き続き別の動画は追加される
    pub fn append_videos(
        &mut self,
        videos: crate::model::VerifiedVideos,
    ) -> Result<(), super::MusicFileErrors> {
        let mut errs = Vec::new();

        for video in videos.into_inner() {
            let (year, month) = (video.get_year(), video.get_month());
            match self.get_file_path_mut(year, month) {
                Some(file) => {
                    file.push_content(video);
                }
                None => {
                    let err = super::MusicFileError::NonExistentMonthFile {
                        year,
                        month,
                        id: video.get_video_id().clone(),
                    };
                    errs.push(err);
                }
            }
        }

        if errs.is_empty() {
            Ok(())
        } else {
            Err(super::MusicFileErrors { errs })
        }
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
            .flat_map(|f| f.content.into_inner())
            .collect();
        crate::model::VerifiedVideos::new(vec_videos)
    }
}

impl MusicFilePathContent {
    pub fn push_content(&mut self, content: crate::model::VerifiedVideo) {
        self.content.push_video(content);
    }
}
