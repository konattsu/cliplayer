/// 音楽情報のルートフォルダであることを保証する
#[derive(Debug, Clone)]
pub struct MusicRoot {
    root: crate::util::DirPath,
    files: Vec<MusicFileEntry>,
}

/// 音楽情報のファイルエントリ
#[derive(Debug, Clone)]
pub struct MusicFileEntry {
    year: usize,
    month: usize,
    file_path: crate::util::FilePath,
}

// TODO x_foo.mdみて直す, 全部消す <- ガチでこのファイルが壊れているから, 全メソッド/関数/型が53

/// 音楽情報のルートディレクトリの内容
#[derive(Debug, Clone)]
pub struct MusicRootContent {
    files: MusicFileEntryWithVideosList,
}

/// 以下を保証
/// - 全ての`MusicFilePathVideo.videos.video_id`(動画id)は一意
#[derive(Debug, Clone)]
pub struct MusicFileEntryWithVideosList {
    inner: Vec<MusicFileEntryWithVideos>,
}

/// 音楽情報のファイルパスの内容
#[derive(Debug, Clone)]
pub struct MusicFileEntryWithVideos {
    year: usize,
    month: usize,
    file_path: crate::util::FilePath,
    videos: crate::model::VerifiedVideos,
}

/// 音楽情報のファイルパスの中身
#[derive(Debug, Clone)]
pub struct MusicFileEntryWithVideosInner {
    pub year: usize,
    pub month: usize,
    pub file_path: crate::util::FilePath,
    pub videos: crate::model::VerifiedVideos,
}

// MARK: NonContent

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

    /// 実際に音楽情報が格納されているファイルを返す
    pub fn get_file_paths(&self) -> &[MusicFileEntry] {
        &self.files
    }
}

impl MusicFileEntry {
    /// MusicFilePathを作成
    ///
    /// ファイル名, ディレクトリ名の制約があるため`pub(super)`に限定
    pub(super) fn new(
        year: usize,
        month: usize,
        file_path: crate::util::FilePath,
    ) -> Self {
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

// MARK: Content

impl MusicRootContent {
    /// MusicRootContentの内部を公開したもの
    /// `Vec<MusicFilePathVideoInner>`を返す
    pub fn into_inner(self) -> Vec<MusicFileEntryWithVideosInner> {
        self.files
            .inner
            .into_iter()
            .map(|f| MusicFileEntryWithVideosInner {
                year: f.year,
                month: f.month,
                file_path: f.file_path,
                videos: f.videos,
            })
            .collect()
    }

    pub fn from_inner(
        inners: Vec<MusicFileEntryWithVideosInner>,
    ) -> Result<Self, String> {
        let vec_entry_with_videos: Vec<MusicFileEntryWithVideos> = inners
            .into_iter()
            .map(|inner| MusicFileEntryWithVideos::from_inner(inner)?)
            .collect();

        let files = MusicFileEntryWithVideosList { inner };
        Self { files }
    }

    /// 全ての楽曲情報をファイルから読み取る
    pub fn load(root: &MusicRoot) -> Result<Self, super::MusicFileErrors> {
        let mut errs: Vec<super::MusicFileError> = Vec::new();
        let mut contents = MusicFileEntryWithVideosList::init(root);

        for file in root.files.iter() {
            match super::fs_util::deserialize_from_file(file.get_file_path()) {
                // ファイルの内容を読み取り, deserializeできたとき
                Ok(content) => {
                    // ファイル内の動画情報の年/月がファイル名に対応していないとき
                    if let Err(e) =
                        content.validate_same_year_month(file.year, file.month)
                    {
                        errs.push(super::MusicFileError::VideoFileNameMismatch {
                            video_ids: e,
                            file_path: file.file_path.clone(),
                        });
                    // (正常) ファイル内の動画情報の年/月がファイル名に対応しているとき
                    } else {
                        contents.inner.push(MusicFileEntryWithVideos {
                            year: file.year,
                            month: file.month,
                            file_path: file.file_path.clone(),
                            videos: content,
                        });
                    }
                }
                // ファイルの内容を読み取れなかったとき
                Err(e) => errs.push(e),
            }
        }

        // ここからは, 同一ファイルでの動画idの重複は起きない
        // ∵ `MusicFilePathVideo`の`videos`は, ファイル内の動画idが一意であることを保証している
        // 逆にファイル間での動画idの重複は起きる可能性がある
        if let Err(e) = contents.validate_duplicate_video_ids_across_files() {
            errs.extend(
                e.into_iter().map(|id| {
                    super::MusicFileError::DuplicatedVideoIdAcrossFiles { id }
                }),
            );
        }

        if errs.is_empty() {
            Ok(Self { files: contents })
        } else {
            Err(super::MusicFileErrors { errs })
        }
    }

    /// 全ての楽曲情報をファイルに書き込む
    ///
    /// 書き込めなかったらスキップする. 全てのファイルに書き込もうとする
    pub fn write(&self) -> Result<(), super::MusicFileErrors> {
        let mut errs: Vec<super::MusicFileError> = Vec::new();

        for file in self.files.inner.iter() {
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
            .inner
            .iter()
            .find(|f| f.year == year && f.month == month);
        if let Some(file) = file {
            super::fs_util::serialize_to_file(&file.file_path, &file.videos)
                .map_err(|e| e.into_errors())
        } else {
            Err(super::MusicFileError::NonExistentMonthFile {
                year,
                month,
                id: None,
            }
            .into_errors())
        }
    }

    /// 楽曲情報をmin化して書き込む
    pub fn write_minified(
        self,
        min_path: &crate::util::FilePath,
    ) -> Result<(), super::MusicFileErrors> {
        let content = self.into_flattened_files()?;
        super::fs_util::serialize_to_file_min(min_path, &content)
            .map_err(|e| e.into_errors())
    }

    /// クリップ情報をフラット化したものを書き込む
    pub fn write_flat_clips(
        self,
        min_flat_path: &crate::util::FilePath,
    ) -> Result<(), super::MusicFileErrors> {
        let clips: Vec<crate::model::VerifiedClip> = self
            .into_flattened_files()?
            .into_sorted_vec()
            .into_iter()
            .flat_map(|v| v.into_clips())
            .collect();
        let clips = crate::model::FlatClips::new(clips);
        super::fs_util::serialize_to_file_min(min_flat_path, &clips)
            .map_err(|e| e.into_errors())
    }

    /// 動画を追加
    ///
    /// # Errors:
    /// - 引数の動画に対応する年月のファイルが存在しない場合
    ///   - 途中でエラーが見つかっても, 引き続き別の動画は追加される
    ///
    /// # Errors:
    /// - 対応する年/月の`MUsicFilePathVideo`が`Self.inner`に含まれていないとき
    /// - 動画idが重複していたとき
    pub fn append_videos(
        &mut self,
        videos: crate::model::VerifiedVideos,
    ) -> Result<(), super::MusicFileErrors> {
        let mut errs = Vec::new();

        for video in videos.inner.into_values() {
            let (year, month) = (video.get_year(), video.get_month());
            if let Some(file) = self.get_file_path_mut(year, month) {
                if let Err(e) = file.append_video(video) {
                    errs.push(e);
                }
            } else {
                let err = super::MusicFileError::NonExistentMonthFile {
                    year,
                    month,
                    id: Some(video.get_video_id().clone()),
                };
                errs.push(err);
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
    ) -> Option<&mut MusicFileEntryWithVideos> {
        self.files
            .inner
            .iter_mut()
            .find(|f| f.year == year && f.month == month)
    }

    /// 内部の動画をflat化する
    ///
    /// `Err`: 動画idが重複している場合.
    /// `Self(MusicRootContent)`内の`MusicFilePathVideo`が存在する => そのファイル内の動画idは一意.
    /// また, 動画idと公開時刻は 1:1 で紐づく.
    ///
    /// そのため, 複数ファイル間で動画idが重複する => どちらかの動画idが持つ,動画の公開時刻が異常.
    // Self作成時に検証して通常はunwrapで落とす
    pub fn into_flattened_files(
        self,
    ) -> Result<crate::model::VerifiedVideos, super::MusicFileErrors> {
        let vec_videos: Vec<crate::model::VerifiedVideo> = self
            .files
            .inner
            .into_iter()
            .flat_map(|f| f.videos.inner.into_values())
            .collect();
        crate::model::VerifiedVideos::try_from_vec(vec_videos).map_err(|e| {
            tracing::error!(
                "Duplicate video_id(s) across files detected: `{}`. \
                This may indicate a bug or an incorrect manual modification of the file. \
                To be specific, video published date is incorrect for one of the files. \
                \n\
                Backtrace: {:?}",
                e.iter()
                    .map(|id| id.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
                , std::backtrace::Backtrace::capture()
            );

            e.into_iter()
                .map(|id| super::MusicFileError::DuplicatedVideoIdAcrossFiles { id })
                .collect::<Vec<_>>()
                .into()
        })
    }
}

impl MusicFileEntryWithVideosList {
    /// MusicFilePathVideosを初期化
    ///
    /// `MusicRoot`に含まれる年/月に対応する空の`MusicFilePathVideo`を作成
    pub fn init(root: &MusicRoot) -> Self {
        let inner: Vec<MusicFileEntryWithVideos> = root
            .files
            .iter()
            .map(|f| MusicFileEntryWithVideos {
                year: f.year,
                month: f.month,
                file_path: f.file_path.clone(),
                videos: crate::model::VerifiedVideos::new_empty(),
            })
            .collect();
        Self { inner }
    }

    /// ファイル間で動画idが重複していないか検証
    ///
    /// `Err`: 重複していた動画idのリスト
    fn validate_duplicate_video_ids_across_files(
        &self,
    ) -> Result<(), Vec<crate::model::VideoId>> {
        let mut video_ids = std::collections::HashSet::new();
        let mut duplicated_ids = std::collections::HashSet::new();

        for file in self.inner.iter() {
            for video in file.videos.inner.values() {
                if !video_ids.insert(video.get_video_id()) {
                    // 動画idの重複の重複は気にしないので結果は無視
                    let _res = duplicated_ids.insert(video.get_video_id().clone());
                }
            }
        }

        if duplicated_ids.is_empty() {
            Ok(())
        } else {
            Err(duplicated_ids.into_iter().collect())
        }
    }

    /// 動画情報を追加
    ///
    /// # Errors:
    /// - 対応する年/月の`MUsicFilePathVideo`が`Self.inner`に含まれていないとき
    /// - 動画idが重複していたとき
    pub fn push_content(
        &mut self,
        content: MusicFileEntryWithVideos,
    ) -> Result<(), super::MusicFileErrors> {
        use super::{MusicFileError, MusicFileErrors};
        let (year, month) = (content.year, content.month);

        if let Some(file) = self
            .inner
            .iter_mut()
            .find(|f| f.year == year && f.month == month)
        {
            // 既にある動画idと引数の動画idが重複していたとき
            if let Err(ids) = file.videos.extend_videos(content.videos) {
                Err(MusicFileErrors {
                    errs: ids
                        .into_iter()
                        .map(|id| MusicFileError::DuplicateVideoIdOnFile {
                            id,
                            file_path: file.file_path.clone(),
                        })
                        .collect(),
                })
            // 正常に追加できたとき
            } else {
                Ok(())
            }
        // 対応するファイルが見つからなかったとき
        } else {
            Err(MusicFileError::NonExistentMonthFile {
                year,
                month,
                id: None,
            }
            .into_errors())
        }
    }
}

impl MusicFileEntryWithVideos {
    /// `MusicFileEntryWithVideos`を作成
    ///
    /// `Err`: 動画の年/月が`MusicFilePathVideo`の年/月と異なるとき
    fn new(
        year: usize,
        month: usize,
        file_path: crate::util::FilePath,
        videos: crate::model::VerifiedVideos,
    ) -> Result<Self, Vec<crate::model::VideoId>> {
        if let Err(ids) = videos.validate_same_year_month(year, month) {
            Err(ids)
        } else {
            Ok(Self {
                year,
                month,
                file_path,
                videos,
            })
        }
    }

    /// `MusicFilePathVideo`に動画を追加
    ///
    /// # Errors:
    /// - 動画idが重複していたとき
    /// - 動画の年/月が`MusicFilePathVideo`の年/月と異なるとき
    fn append_video(
        &mut self,
        video: crate::model::VerifiedVideo,
    ) -> Result<(), super::MusicFileError> {
        if self.year == video.get_year() && self.month == video.get_month() {
            self.videos.push_video(video).map_err(|id| {
                super::MusicFileError::DuplicateVideoIdOnFile {
                    id,
                    file_path: self.file_path.clone(),
                }
            })
        } else {
            tracing::trace!(
                "Video year/month mismatch: file ({}, {}) vs video ({}, {}) \
                This may be a miscalling.\n Self: `{self:?}`\n Video: `{video:?}`",
                self.year,
                self.month,
                video.get_year(),
                video.get_month(),
            );

            Err(super::MusicFileError::VideoFileNameMismatch {
                video_ids: vec![video.get_video_id().clone()],
                file_path: self.file_path.clone(),
            })
        }
    }

    /// `MusicFilePathVideoInner`から`MusicFilePathVideo`を作成
    ///
    /// `Err`: 動画の年/月が`MusicFilePathVideo`の年/月と異なるとき
    fn from_inner(
        inner: MusicFileEntryWithVideosInner,
    ) -> Result<Self, Vec<crate::model::VideoId>> {
        Self::new(inner.year, inner.month, inner.file_path, inner.videos)
    }
}
