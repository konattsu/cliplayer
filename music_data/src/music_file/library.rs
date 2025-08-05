/// 保持している楽曲情報を管理するライブラリ
#[derive(Debug, Clone)]
pub struct MusicLibrary {
    root_dir: crate::util::DirPath,
    min_videos_path: crate::util::FilePath,
    min_flat_clips_path: crate::util::FilePath,
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
    /// - `min_videos_path`: 最小限の動画情報を保持するファイルパス
    /// - `min_flat_clips_path`: 最小限のフラットクリップ情報を保持するファイルパス
    #[tracing::instrument(level = tracing::Level::DEBUG)]
    pub fn load(
        dir: &crate::util::DirPath,
        min_videos_path: &crate::util::FilePath,
        min_flat_clips_path: &crate::util::FilePath,
    ) -> Result<Self, super::MusicFileErrors> {
        Self::collect_music_file_paths(dir).map(|file| Self {
            root_dir: dir.clone(),
            min_videos_path: min_videos_path.clone(),
            min_flat_clips_path: min_flat_clips_path.clone(),
            video_files: file,
        })
    }

    /// 楽曲情報をファイルに保存する
    ///
    /// 単一のファイルたちとmin2種
    #[tracing::instrument(fields(
        root_dir = %self.root_dir,
        min_videos_path = %self.min_videos_path,
        min_flat_clips_path = %self.min_flat_clips_path
    ), level = tracing::Level::DEBUG)]
    pub(crate) fn save(self) -> Result<(), super::MusicFileError> {
        println!("Saving music files to disk...");
        self.save_music_files()?;

        let min_videos_path = self.min_videos_path.clone();
        let min_flat_clips_path = self.min_flat_clips_path.clone();

        let videos = self.into_videos()?;
        super::fs_util::serialize_to_file(&min_videos_path, &videos, true)?;

        let clips = crate::model::FlatClips::from_verified_videos(videos);
        super::fs_util::serialize_to_file(&min_flat_clips_path, &clips, true)?;

        println!("Music files saved successfully.");
        Ok(())
    }

    #[tracing::instrument(fields(
        root_dir = %self.root_dir,
        min_videos_path = %self.min_videos_path,
        min_flat_clips_path = %self.min_flat_clips_path
    ), ret, level = tracing::Level::TRACE)]
    pub(crate) fn save_only_min(self) -> Result<(), super::MusicFileError> {
        let min_videos_path = self.min_videos_path.clone();
        let min_flat_clips_path = self.min_flat_clips_path.clone();

        let videos = self.into_videos()?;
        super::fs_util::serialize_to_file(&min_videos_path, &videos, true)?;

        let clips = crate::model::FlatClips::from_verified_videos(videos);
        super::fs_util::serialize_to_file(&min_flat_clips_path, &clips, true)?;

        Ok(())
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
    fn collect_music_file_paths(
        dir: &crate::util::DirPath,
    ) -> Result<
        std::collections::HashMap<(usize, usize), super::MusicFile>,
        super::MusicFileErrors,
    > {
        use std::collections::HashMap;
        let mut files: HashMap<(usize, usize), super::MusicFile> = HashMap::new();
        let mut errs: Vec<crate::music_file::MusicFileError> = Vec::new();

        for entry in walkdir::WalkDir::new(dir.as_path()) {
            // ディレクトリ/ファイルなどのエントリを取得
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    errs.push(super::MusicFileError::ReadDir {
                        dir: dir.clone(),
                        msg: e.to_string(),
                    });
                    continue;
                }
            };
            // entryがファイルだと, 楽曲情報のファイルとみなす
            let file_path = if entry.file_type().is_file() {
                match crate::util::FilePath::new(entry.path()) {
                    // ファイルでFilePathの生成に成功したとき
                    Ok(path) => path,
                    // ファイルだったがFilePathの生成に失敗したとき
                    Err(e) => {
                        errs.push(super::MusicFileError::FileOpen {
                            path: entry.path().display().to_string(),
                            msg: e.to_string(),
                            when: "load music data".to_string(),
                        });
                        continue;
                    }
                }
            // ディレクトリ, シンボリックリンクなどのファイル以外のとき
            } else {
                continue;
            };
            // 楽曲情報のファイルを読み込む
            match crate::music_file::MusicFile::load(file_path, dir) {
                Ok(music_file) => Self::insert_music_file(&mut files, music_file),
                Err(e) => errs.push(e),
            }
        }

        if errs.is_empty() {
            tracing::trace!(
                "Loaded {} music files, dir {}",
                files.len(),
                dir.as_path().display()
            );
            Ok(files)
        } else {
            Err(errs.into())
        }
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
    #[tracing::instrument(level = tracing::Level::TRACE)]
    fn save_music_files(&self) -> Result<(), super::MusicFileError> {
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

    fn into_videos(
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
}
