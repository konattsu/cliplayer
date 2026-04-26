/// 保持している楽曲情報を管理するライブラリ
#[derive(Debug, Clone)]
pub struct MusicLibrary {
    root_dir: std::path::PathBuf,
    /// (year, month)
    video_files:
        std::collections::HashMap<(usize, usize), crate::music_file::MusicFile>,
}

impl MusicLibrary {
    pub(crate) fn get_video_ids(&self) -> crate::model::VideoIds {
        self.video_files
            .values()
            .flat_map(|file| file.get_video_ids())
            .collect()
    }

    pub(crate) fn iter_files_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut crate::music_file::MusicFile> {
        self.video_files.values_mut()
    }

    /// 楽曲情報をファイルから指定のディレクトリ以下から読むこむ
    ///
    /// # Arguments
    /// - `dir`: 楽曲情報のルートディレクトリ
    #[tracing::instrument(level = tracing::Level::DEBUG)]
    pub(crate) fn load(dir: &std::path::Path) -> Result<Self, super::MusicFileErrors> {
        tracing::debug!(
            "Loading monthly music files from directory: `{}`",
            dir.display()
        );
        let files = Self::collect_music_file_paths(dir)?;
        if files.is_empty() {
            Err(super::MusicFileError::InvalidPath {
                path: dir.to_path_buf(),
                msg: format!(
                    "No monthly music files found in directory `{}`",
                    dir.display()
                ),
            }
            .into_errors())
        } else {
            Ok(Self {
                root_dir: dir.to_path_buf(),
                video_files: files,
            })
        }
    }

    /// 楽曲情報をファイルに保存する
    ///
    /// 単一のファイルたちのみ
    pub(crate) fn save_month_files(&self) -> Result<(), super::MusicFileErrors> {
        tracing::info!("Saving monthly music files to disk...");
        self.save_music_files()?;
        tracing::info!("Saved all monthly music files saved successfully.");
        Ok(())
    }

    /// 読み込んでいる動画情報を全て取得
    ///
    /// - `Err(_)`:
    pub(crate) fn into_videos(
        self,
    ) -> Result<crate::model::VerifiedVideos, super::MusicFileError> {
        let videos = self
            .video_files
            .into_values()
            .flat_map(|file| file.into_videos().into_sorted_vec())
            .collect::<Vec<_>>();
        crate::model::VerifiedVideos::try_from_vec(videos).map_err(|ids| {
            let msg = format!(
                "video_id(s) are duplicated ({ids:?}).\n\
                Each individual file guarantees no duplicate video_id, \
                but duplication occurred when integrating all files together. \
                Since videos are split by upload time, there should not be multiple instances of the same video_id. \
                This indicates either an implementation error or internal data inconsistency.",
            );
            super::MusicFileError::InvalidDatabaseContent { msg }
        })
    }

    /// minファイルを保存する
    pub(crate) fn save_min_file<T: serde::Serialize>(
        value: &T,
        save_path: &std::path::Path,
    ) -> Result<(), super::MusicFileError> {
        tracing::info!("Saving `{}` file to disk...", save_path.display());
        super::fs_util::serialize_to_file(save_path, value, true)?;
        tracing::info!("Saved `{}` file successfully.", save_path.display());
        Ok(())
    }

    /// 動画情報を一括で追加する
    pub(crate) fn extend_videos(&mut self, videos: crate::model::VerifiedVideos) {
        for video in videos.into_sorted_vec() {
            self.push_video(video);
        }
    }

    /// 楽曲情報を追加する
    ///
    /// 動画idが重複していれば上書き
    fn push_video(&mut self, video: crate::model::VerifiedVideo) {
        // 追加する動画の年月を特定して, 対応するファイルに追加
        let year_month = (video.get_year(), video.get_month());
        if let Some(music_file) = self.video_files.get_mut(&year_month) {
            // 対応する月ファイルが存在した時
            music_file
                .push_video(video)
                // Safety: 対応する月ファイルが存在していることを確認済み
                .expect("will match existing file");
        } else {
            // 対応する月ファイルが存在しないとき. 新規作成して, そこに追加
            let music_file = super::MusicFile::from_video(video, &self.root_dir);
            Self::insert_music_file(&mut self.video_files, music_file);
        }
    }

    /// 楽曲情報をファイルから指定のディレクトリ以下から読むこむ
    ///
    /// `dir`以下の全てのファイルに対して`MusicFile::load_file`を適用
    #[tracing::instrument(ret, level = tracing::Level::TRACE)]
    fn collect_music_file_paths(
        dir: &std::path::Path,
    ) -> Result<
        std::collections::HashMap<(usize, usize), super::MusicFile>,
        super::MusicFileErrors,
    > {
        let file_paths = Self::collect_music_file_paths_in_dir(dir);
        let (files, errs) = Self::load_music_files(file_paths, dir);
        if errs.is_empty() {
            let mut year_month = files.keys().collect::<Vec<_>>();
            year_month.sort_by(|y, m| y.0.cmp(&m.0).then(y.1.cmp(&m.1)));
            tracing::info!(
                "Loaded {} monthly music month files from directory `{}`",
                files.len(),
                dir.display()
            );
            Ok(files)
        } else {
            tracing::error!(
                "Loaded {} monthly music month files with {} errors from directory `{}`",
                files.len(),
                errs.len(),
                dir.display()
            );
            Err(errs.into())
        }
    }

    /// 指定ディレクトリ以下の全ファイルパスを収集
    fn collect_music_file_paths_in_dir(
        dir: &std::path::Path,
    ) -> Vec<std::path::PathBuf> {
        let mut file_paths = Vec::new();
        for entry in walkdir::WalkDir::new(dir) {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };
            if entry.file_type().is_file() {
                file_paths.push(entry.path().to_path_buf());
            }
        }
        file_paths
    }

    /// ファイルパスごとにMusicFileをロードし、HashMapとエラーVecを返す
    fn load_music_files(
        file_paths: Vec<std::path::PathBuf>,
        dir: &std::path::Path,
    ) -> (
        std::collections::HashMap<(usize, usize), super::MusicFile>,
        Vec<crate::music_file::MusicFileError>,
    ) {
        use std::collections::HashMap;
        let mut files: HashMap<(usize, usize), super::MusicFile> = HashMap::new();
        let mut errs: Vec<crate::music_file::MusicFileError> = Vec::new();
        for file_path in file_paths {
            match crate::music_file::MusicFile::load(file_path, dir) {
                Ok(music_file) => Self::insert_music_file(&mut files, music_file),
                Err(e) => errs.push(e),
            }
        }
        (files, errs)
    }

    /// 楽曲情報を追加する
    ///
    /// `args`: key: `(year, month)`
    ///
    /// - music_fileが重複していれば上書き
    // TODO `self`を引数に受け取って`files`受け取らないようにしたい. しかし, `self`が使えない環境から呼び出されることもあるので困る
    fn insert_music_file(
        files: &mut std::collections::HashMap<(usize, usize), super::MusicFile>,
        music_file: super::MusicFile,
    ) {
        let year_month = music_file.get_year_month();
        if let Some(_stale) = files.insert(year_month, music_file) {
            tracing::trace!(
                "Music file for year/month `{}/{}` already exists, \
                replacing stale file with new one.\n",
                year_month.0,
                year_month.1,
            );
        }
    }

    /// 楽曲情報(単品)を全て保存
    ///
    /// pretty形式で保存
    ///
    /// 一つでも保存できなかったら即座にエラーを返すのではなく, 全てのファイルに対して保存を試みて,
    /// 失敗したファイルのパスとエラー内容を全て返す.
    #[tracing::instrument(skip(self), level = tracing::Level::TRACE)]
    fn save_music_files(&self) -> Result<(), super::MusicFileErrors> {
        tracing::info!(
            "Saving {} monthly music files (total {} videos) to disk...",
            self.video_files.len(),
            self.get_video_ids().len()
        );

        let mut errs = Vec::new();

        for file in self.video_files.values() {
            if let Err(e) = file.save() {
                let e = super::MusicFileError::FileWrite {
                    path: file.get_path().to_path_buf(),
                    msg: e.to_string(),
                };
                errs.push(e);
            }
        }

        if errs.is_empty() {
            Ok(())
        } else {
            Err(errs.into())
        }
    }
}
