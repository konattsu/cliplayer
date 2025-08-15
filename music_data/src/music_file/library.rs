/// 保持している楽曲情報を管理するライブラリ
#[derive(Debug, Clone)]
pub struct MusicLibrary {
    root_dir: crate::util::DirPath,
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
    /// `dir`以下の全てのファイルに対して`MusicFile::load_file`を適用
    ///
    /// # Arguments
    /// - `dir`: 楽曲情報のルートディレクトリ
    #[tracing::instrument(level = tracing::Level::DEBUG)]
    pub fn load(dir: &crate::util::DirPath) -> Result<Self, super::MusicFileErrors> {
        tracing::debug!("Loading music files from directory: `{dir}`",);
        Self::collect_music_file_paths(dir).map(|file| Self {
            root_dir: dir.clone(),
            video_files: file,
        })
    }

    /// 楽曲情報をファイルに保存する
    ///
    /// 単一のファイルたちのみ
    #[tracing::instrument(
        skip(self),
        fields(
            root_dir = %self.root_dir,
        ),
        level = tracing::Level::DEBUG
    )]
    pub(crate) fn save_month_files(&self) -> Result<(), super::MusicFileError> {
        println!("Saving music month files to disk...");
        self.save_music_files()?;
        println!("Music month files saved successfully.");
        Ok(())
    }

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
            tracing::error!("{msg}");
            super::MusicFileError::ImplementationErr { msg }
        })
    }

    /// minファイルを保存する
    pub(crate) fn save_min_file<T: serde::Serialize>(
        value: &T,
        save_path: &crate::util::FilePath,
    ) -> Result<(), super::MusicFileError> {
        super::fs_util::serialize_to_file(save_path, value, true)
    }

    /// 楽曲情報を追加する
    ///
    /// 動画idが重複していれば上書き
    pub(crate) fn push_video(
        &mut self,
        video: crate::model::VerifiedVideo,
    ) -> Result<(), super::MusicFileError> {
        let year_month = (video.get_year(), video.get_month());
        if let Some(music_file) = self.video_files.get_mut(&year_month) {
            music_file
                .push_video(video)
                .expect("will match existing file");
        } else {
            let music_file = super::MusicFile::from_video(video, &self.root_dir)?;
            Self::insert_music_file(&mut self.video_files, music_file);
        }
        Ok(())
    }

    /// 動画情報を一括で追加する
    pub(crate) fn extend_videos(
        &mut self,
        videos: crate::model::VerifiedVideos,
    ) -> Result<(), super::MusicFileError> {
        for video in videos.into_sorted_vec() {
            self.push_video(video)?;
        }
        Ok(())
    }

    /// 楽曲情報をファイルから指定のディレクトリ以下から読むこむ
    ///
    /// `dir`以下の全てのファイルに対して`MusicFile::load_file`を適用
    #[tracing::instrument(ret, level = tracing::Level::TRACE)]
    fn collect_music_file_paths(
        dir: &crate::util::DirPath,
    ) -> Result<
        std::collections::HashMap<(usize, usize), super::MusicFile>,
        super::MusicFileErrors,
    > {
        let file_paths = Self::collect_music_file_paths_in_dir(dir);
        let (files, errs) = Self::load_music_files(file_paths, dir);
        if errs.is_empty() {
            let mut debug_files = files.keys().collect::<Vec<_>>();
            debug_files.sort_by(|y, m| y.0.cmp(&m.0).then(y.1.cmp(&m.1)));
            tracing::debug!(
                "Loaded {} music files, dir `{}`, (month/year): {:?}",
                debug_files.len(),
                dir.as_path().display(),
                debug_files
            );
            Ok(files)
        } else {
            Err(errs.into())
        }
    }

    /// 指定ディレクトリ以下の全ファイルパスを収集
    fn collect_music_file_paths_in_dir(
        dir: &crate::util::DirPath,
    ) -> Vec<crate::util::FilePath> {
        let mut file_paths = Vec::new();
        for entry in walkdir::WalkDir::new(dir.as_path()) {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };
            if entry.file_type().is_file() {
                match crate::util::FilePath::new(entry.path()) {
                    Ok(path) => file_paths.push(path),
                    Err(_) => continue,
                }
            }
        }
        file_paths
    }

    /// ファイルパスごとにMusicFileをロードし、HashMapとエラーVecを返す
    fn load_music_files(
        file_paths: Vec<crate::util::FilePath>,
        dir: &crate::util::DirPath,
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
    fn insert_music_file(
        files: &mut std::collections::HashMap<(usize, usize), super::MusicFile>,
        music_file: super::MusicFile,
    ) {
        let year_month = music_file.get_year_month();
        if let Some(_stale) = files.insert(year_month, music_file) {
            tracing::trace!(
                "Music file for year/month `{}/{}` already exists, \
                replacing stale file with new one.\n",
                // Stale file: {:?}",
                year_month.0,
                year_month.1,
                // stale
            );
        }
    }

    /// 楽曲情報(単品)を全て保存
    ///
    /// pretty形式で保存
    ///
    /// 一つでも保存できなかったら即座にエラーを返す
    #[tracing::instrument(skip(self), level = tracing::Level::TRACE)]
    fn save_music_files(&self) -> Result<(), super::MusicFileError> {
        tracing::debug!(
            "Saving music files ({} videos) to disk",
            self.get_video_ids().len()
        );

        for file in self.video_files.values() {
            if let Err(e) = file.save() {
                return Err(super::MusicFileError::FileWrite {
                    path: file.get_path().clone(),
                    msg: e.to_string(),
                });
            }
        }
        Ok(())
    }
}
