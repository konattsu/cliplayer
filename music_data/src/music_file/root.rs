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
    files: Vec<MusicFilePathVideos>,
}

/// 音楽情報のファイルパスの内容
#[derive(Debug, Clone)]
pub struct MusicFilePathVideos {
    year: usize,
    month: usize,
    file_path: crate::util::FilePath,
    videos: crate::model::VerifiedVideos,
}

/// 音楽情報のファイルパスの中身
#[derive(Debug, Clone)]
pub struct MusicFilePathVideosInner {
    pub year: usize,
    pub month: usize,
    pub file_path: crate::util::FilePath,
    pub videos: crate::model::VerifiedVideos,
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
    pub fn into_inner(self) -> Vec<MusicFilePathVideosInner> {
        self.files
            .into_iter()
            .map(|f| MusicFilePathVideosInner {
                year: f.year,
                month: f.month,
                file_path: f.file_path,
                videos: f.videos,
            })
            .collect()
    }

    /// 楽曲情報をファイルから読み取る
    pub fn load(root: &MusicRoot) -> Result<Self, super::MusicFileErrors> {
        let mut errs: Vec<super::MusicFileError> = Vec::new();
        let mut contents: Vec<MusicFilePathVideos> = Vec::new();

        for file in root.files.iter() {
            match super::fs_util::deserialize_from_file(file.get_file_path()) {
                Ok(content) => contents.push(MusicFilePathVideos {
                    year: file.get_year(),
                    month: file.get_month(),
                    file_path: file.get_file_path().clone(),
                    videos: content,
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

    /// 全ての楽曲情報をファイルに書き込む
    ///
    /// 書き込めなかったらスキップする. 全てのファイルに書き込もうとする
    pub fn write(&self) -> Result<(), super::MusicFileErrors> {
        let mut errs: Vec<super::MusicFileError> = Vec::new();

        for file in self.files.iter() {
            match super::fs_util::serialize_to_file(&file.file_path, &file.videos) {
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

    /// 特定の年月の楽曲情報をファイルに書き込む
    pub fn write_single_file(
        &self,
        year: usize,
        month: usize,
    ) -> Result<(), super::MusicFileErrors> {
        let file = self
            .files
            .iter()
            .find(|f| f.year == year && f.month == month);
        if let Some(file) = file {
            super::fs_util::serialize_to_file(&file.file_path, &file.videos)
                .map_err(|e| e.into_errors())
        } else {
            Err(super::MusicFileErrors {
                errs: vec![super::MusicFileError::NonExistentMonthFile {
                    year,
                    month,
                    id: None,
                }],
            })
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

    // TODO このファイル直す, src/validate.rs直す(ファイル分割地味に悩み中), src/apply/sync.rs作る

    /// クリップ情報をフラット化したものを書き込む
    pub fn write_flat_clips(
        self,
        min_flat_path: &crate::util::FilePath,
    ) -> Result<(), super::MusicFileError> {
        let clips: Vec<crate::model::VerifiedClip> = self
            .into_flattened_files()
            .into_sorted_vec()
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
                        id: Some(video.get_video_id().clone()),
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
    ) -> Option<&mut MusicFilePathVideos> {
        self.files
            .iter_mut()
            .find(|f| f.year == year && f.month == month)
    }

    /// 内部の動画をflat化する
    pub fn into_flattened_files(
        self,
    ) -> Result<crate::model::VerifiedVideos, Vec<crate::model::VideoId>> {
        let vec_videos: Vec<crate::model::VerifiedVideo> = self
            .files
            .into_iter()
            .flat_map(|f| f.videos.inner.into_values())
            .collect();
        crate::model::VerifiedVideos::try_from_vec(vec_videos)
    }
}

impl MusicFilePathVideos {
    pub fn push_content(&mut self, content: crate::model::VerifiedVideo) {
        self.videos.push_video(content);
    }
}

// TODO このファイル全体的にコードが微妙. バグもあるかも
