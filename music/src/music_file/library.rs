/// 保持している楽曲情報を管理するライブラリ
#[derive(Debug, Clone)]
pub struct MusicLibrary {
    root_dir: std::path::PathBuf,
    /// (year, month)
    video_files:
        std::collections::HashMap<(usize, usize), crate::music_file::MusicFile>,
}

impl MusicLibrary {
    pub(crate) fn new(
        root_dir: std::path::PathBuf,
        video_files: std::collections::HashMap<
            (usize, usize),
            crate::music_file::MusicFile,
        >,
    ) -> Self {
        Self {
            root_dir,
            video_files,
        }
    }

    pub(crate) fn iter_files(
        &self,
    ) -> impl Iterator<Item = &crate::music_file::MusicFile> {
        self.video_files.values()
    }

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

    /// 読み込んでいる動画情報を全て取得
    ///
    /// - `Err(_)`:
    pub fn into_videos(
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

    /// 動画情報を一括で追加する
    pub(crate) fn extend_videos(
        &mut self,
        videos: crate::model::VerifiedVideos,
        duplicate_video_policy: crate::music_file::DuplicateVideoPolicy,
    ) -> Result<(), crate::music_file::MusicFileErrors> {
        let mut errs = Vec::new();
        for video in videos.into_sorted_vec() {
            if let Err(e) = self.push_video(video, duplicate_video_policy) {
                errs.push(e);
            }
        }

        if errs.is_empty() {
            Ok(())
        } else {
            Err(errs.into())
        }
    }

    /// 楽曲情報を追加する
    ///
    fn push_video(
        &mut self,
        video: crate::model::VerifiedVideo,
        duplicate_video_policy: crate::music_file::DuplicateVideoPolicy,
    ) -> Result<(), crate::music_file::MusicFileError> {
        // 追加する動画の年月を特定して, 対応するファイルに追加
        let year_month = (video.get_year(), video.get_month());
        if let Some(music_file) = self.video_files.get_mut(&year_month) {
            // 対応する月ファイルが存在した時
            music_file.push_video(video, duplicate_video_policy)
        } else {
            // 対応する月ファイルが存在しないとき. 新規作成して, そこに追加
            let music_file = super::MusicFile::from_video(video, &self.root_dir);
            let existing = self.video_files.insert(year_month, music_file);
            debug_assert!(
                existing.is_none(),
                "newly created month file must not conflict with existing keys"
            );
            Ok(())
        }
    }
}
