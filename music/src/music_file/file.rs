/// 楽曲情報の単一のファイル
///
/// 以下を保証
/// - 楽曲情報のルートから `/YYYY/MM.json` の形式
/// - 動画情報の年と月がパスと同じ
#[derive(Debug, Clone)]
pub(crate) struct MusicFile {
    path: std::path::PathBuf,
    videos: super::videos::VideosSameYearMonth,
}

impl MusicFile {
    pub(crate) fn into_videos(self) -> crate::model::VerifiedVideos {
        self.videos.into_videos()
    }
    pub(crate) fn get_year(&self) -> usize {
        self.videos.get_year()
    }
    pub(crate) fn get_month(&self) -> usize {
        self.videos.get_month()
    }
    pub(crate) fn get_path(&self) -> &std::path::Path {
        &self.path
    }

    /// `(year, month)`
    pub(crate) fn get_year_month(&self) -> (usize, usize) {
        (self.get_year(), self.get_month())
    }
    pub(crate) fn get_video_ids(&self) -> crate::model::VideoIds {
        self.videos.get_videos().to_video_ids()
    }

    /// 動画情報から`Self`を作成
    ///
    /// - 戻り値の`MusicFile`に引数の`video`の情報が含まれる
    /// - 動画情報の年と月がパスと一致することを保証
    pub(super) fn from_video(
        video: crate::model::VerifiedVideo,
        root: &std::path::Path,
    ) -> Self {
        let (year, month) = (video.get_year(), video.get_month());
        let path = root
            // `<root>/YYYY/MM.json`の形式の文字列を生成
            .join(format!("{year:04}/{month:02}.json"));
        // Safety: 動画情報から`year`, `month`を抽出しパスを生成. このパスと動画情報の年/月は必ず一致するためunwrap
        let videos =
            super::videos::VideosSameYearMonth::new(year, month, video.into_videos())
                .unwrap();
        Self { path, videos }
    }

    /// ファイルから楽曲情報を読み込む
    pub(super) fn load(
        path: std::path::PathBuf,
        root: &std::path::Path,
    ) -> Result<Self, super::MusicFileError> {
        let videos = super::fs_util::deserialize_from_file(&path)?;
        Self::new(path, root, videos)
    }

    /// ファイルに楽曲情報を書き込む
    ///
    /// pretty形式
    pub(crate) fn save(&self) -> Result<(), super::MusicFileError> {
        super::fs_util::serialize_to_file(&self.path, self.videos.get_videos(), false)
    }

    /// 動画情報を追加
    ///
    /// - 動画のvideo_idが重複してれば上書き
    ///
    /// # Errors
    /// - 動画の投稿日の年/月がこのMusicFileの年/月と異なる場合
    pub(super) fn push_video(
        &mut self,
        video: crate::model::VerifiedVideo,
    ) -> Result<(), super::MusicFileError> {
        self.videos.push_video(video).map_err(|id| {
            super::MusicFileError::VideoPublishDateMismatch {
                ids: id.into_ids(),
                year: self.get_year(),
                month: self.get_month(),
                file_path: self.path.clone(),
            }
        })
    }

    /// 動画情報を置き換え
    ///
    /// `Err`: 動画の投稿日の年/月がMusicFileの年/月と異なる場合
    pub(crate) fn replace_videos(
        &mut self,
        videos: crate::model::VerifiedVideos,
    ) -> Result<(), super::MusicFileError> {
        self.videos.replace_videos(videos).map_err(|ids| {
            super::MusicFileError::VideoPublishDateMismatch {
                ids,
                year: self.get_year(),
                month: self.get_month(),
                file_path: self.path.clone(),
            }
        })
    }

    /// 楽曲情報の単一のファイルを作成
    ///
    /// `Err`:
    /// - 楽曲情報のルートから `/YYYY/MM.json` でない
    /// - 動画情報の年と月がパスと異なる
    fn new(
        path: std::path::PathBuf,
        root: &std::path::Path,
        videos: crate::model::VerifiedVideos,
    ) -> Result<Self, super::MusicFileError> {
        let (year, month) = Self::extract_year_month(&path, root)?;
        let videos = super::videos::VideosSameYearMonth::new(year, month, videos)
            .map_err(|ids| super::MusicFileError::VideoPublishDateMismatch {
                ids,
                year,
                month,
                file_path: path.clone(),
            })?;
        tracing::trace!(
            "music videos loaded: {year}-{month:02}, {} videos",
            videos.len()
        );
        Ok(Self { path, videos })
    }

    /// 引数の`root`から `/YYYY/MM.json` の形式であることを確認し, 年と月を抽出
    ///
    /// # Returns
    /// - `Ok((year, month))`: (年, 月)
    fn extract_year_month(
        path: &std::path::Path,
        root: &std::path::Path,
    ) -> Result<(usize, usize), super::MusicFileError> {
        let to_err = |msg: &str| super::MusicFileError::InvalidPath {
            path: path.to_path_buf(),
            msg: msg.to_string(),
        };

        let rel = path
            .strip_prefix(root)
            .map_err(|_e| to_err("Path is not relative to root"))?;

        // パスが "YYYY/MM.json" 形式か検証
        let mut components = rel.components();
        let year_str_dir = components
            .next()
            .and_then(|c| c.as_os_str().to_str())
            .ok_or_else(|| to_err("Path doesn't have a year dir"))?;

        let file_str = components
            .next()
            .and_then(|c| c.as_os_str().to_str())
            .ok_or_else(|| to_err("Path doesn't have a month file"))?;

        // 年をパース
        let year: usize = year_str_dir
            .parse()
            .map_err(|_e| to_err("Path doesn't have a year dir"))?;

        // 月をパース（"MM.json" から "MM" を抜き出す）
        if !file_str.ends_with(".json") || file_str.len() != 7 {
            return Err(to_err(
                "Path must end with '.json' and be 7 characters long",
            ));
        }
        let month_str = &file_str[..2];
        let month: usize = month_str
            .parse()
            .map_err(|_e| to_err("Path is invalid month name"))?;

        // 月の範囲を検証
        if !(1..=12).contains(&month) {
            return Err(to_err("Path is invalid month name"));
        }

        Ok((year, month))
    }
}

// MARK: Tests

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_dir(path: &str) -> std::path::PathBuf {
        std::path::PathBuf::from(path)
    }
    fn dummy_path(path: &str) -> std::path::PathBuf {
        std::path::PathBuf::from(path)
    }

    // `2024-01` の動画公開日をもつ
    fn dummy_verified_videos() -> crate::model::VerifiedVideos {
        let a = crate::model::VerifiedVideo::self_a();
        crate::model::VerifiedVideos::try_from_vec(vec![a]).unwrap()
    }

    /// `2024-01`, `2024-02` の動画公開日をもつ
    fn dummy_mismatched_videos() -> crate::model::VerifiedVideos {
        let a = crate::model::VerifiedVideo::self_a();
        let b = crate::model::VerifiedVideo::self_b();
        crate::model::VerifiedVideos::try_from_vec(vec![a, b]).unwrap()
    }

    #[test]
    fn test_music_file_extract_year_month_valid() {
        let root = dummy_dir("/tmp/music_root");
        let path = dummy_path("/tmp/music_root/2024/01.json");
        let (year, month) = MusicFile::extract_year_month(&path, &root).unwrap();
        assert_eq!(year, 2024);
        assert_eq!(month, 1);

        let root = dummy_dir("/tmp/music_root");
        let path = dummy_path("/tmp/music_root/2020/10.json");
        let (year, month) = MusicFile::extract_year_month(&path, &root).unwrap();
        assert_eq!(year, 2020);
        assert_eq!(month, 10);
    }

    #[test]
    fn test_music_file_extract_year_month_invalid_path() {
        let root = dummy_dir("/tmp/music_root");
        let path = dummy_path("/tmp/music_root/01.json");
        let err = MusicFile::extract_year_month(&path, &root).unwrap_err();
        assert!(matches!(
            err,
            super::super::MusicFileError::InvalidPath { .. }
        ));

        let root = dummy_dir("/tmp/music_root");
        let path = dummy_path("/tmp/music_root/2024/2025/01.json");
        let err = MusicFile::extract_year_month(&path, &root).unwrap_err();
        assert!(matches!(
            err,
            super::super::MusicFileError::InvalidPath { .. }
        ));
    }

    #[test]
    fn test_music_file_extract_year_month_invalid_year_and_month() {
        let root = dummy_dir("/tmp/music_root");
        let path = dummy_path("/tmp/music_root/2024/1.json"); // 月が1桁
        let err = MusicFile::extract_year_month(&path, &root).unwrap_err();
        assert!(matches!(
            err,
            super::super::MusicFileError::InvalidPath { .. }
        ));

        let root = dummy_dir("/tmp/music_root");
        let path = dummy_path("/tmp/music_root/2024/13.json"); // 13月
        let err = MusicFile::extract_year_month(&path, &root).unwrap_err();
        assert!(matches!(
            err,
            super::super::MusicFileError::InvalidPath { .. }
        ));

        let root = dummy_dir("/tmp/music_root");
        let path = dummy_path("/tmp/music_root/2024/13.webm"); // webm
        let err = MusicFile::extract_year_month(&path, &root).unwrap_err();
        assert!(matches!(
            err,
            super::super::MusicFileError::InvalidPath { .. }
        ));
    }

    #[test]
    fn test_music_file_new_success() {
        let root = dummy_dir("/tmp/music_root");
        let path = dummy_path("/tmp/music_root/2024/01.json");
        let videos = dummy_verified_videos();
        let mf = MusicFile::new(path, &root, videos);
        assert!(mf.is_ok());
    }

    #[test]
    fn test_music_file_new_video_date_mismatch() {
        let root = dummy_dir("/tmp/music_root");
        let path = dummy_path("/tmp/music_root/2024/02.json");
        let videos = dummy_verified_videos();
        let mf = MusicFile::new(path.clone(), &root, videos);
        let expect = super::super::MusicFileError::VideoPublishDateMismatch {
            ids: crate::model::VideoId::test_id_1().into_ids(),
            year: 2024,
            month: 2,
            file_path: path,
        };
        assert_eq!(mf.unwrap_err(), expect);
    }

    #[test]
    fn test_music_file_new_video_date_mismatch_2() {
        let root = dummy_dir("/tmp/music_root");
        let path = dummy_path("/tmp/music_root/2024/01.json");
        let videos = dummy_mismatched_videos();
        let mf = MusicFile::new(path.clone(), &root, videos);
        let expect = super::super::MusicFileError::VideoPublishDateMismatch {
            ids: crate::model::VideoId::test_id_2().into_ids(),
            year: 2024,
            month: 1,
            file_path: path,
        };
        assert_eq!(mf.unwrap_err(), expect);
    }
}
