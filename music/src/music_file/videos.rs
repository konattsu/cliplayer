/// 投稿日の年/月が同じ動画の情報を保持
#[derive(Debug, Clone)]
pub(super) struct VideosSameYearMonth {
    year: usize,
    month: usize,
    videos: crate::model::VerifiedVideos,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DuplicateVideoPolicy {
    Reject,
    Overwrite,
}

#[derive(Debug)]
pub(super) enum PushVideoError {
    YearMonthMismatch(crate::model::VideoId),
    DuplicateVideoId(crate::model::VideoId),
}

impl VideosSameYearMonth {
    pub(super) fn get_year(&self) -> usize {
        self.year
    }
    pub(super) fn get_month(&self) -> usize {
        self.month
    }
    pub(super) fn get_videos(&self) -> &crate::model::VerifiedVideos {
        &self.videos
    }
    pub(super) fn into_videos(self) -> crate::model::VerifiedVideos {
        self.videos
    }

    pub(super) fn len(&self) -> usize {
        self.videos.len()
    }

    /// 新規作成
    ///
    /// `Err`:
    /// 動画投稿日の年/月が引数に対応していないとき. 対応していない動画idのリストを返す.
    pub(super) fn new(
        year: usize,
        month: usize,
        videos: crate::model::VerifiedVideos,
    ) -> Result<Self, crate::model::VideoIds> {
        videos.ensure_same_year_month(year, month)?;

        Ok(Self {
            year,
            month,
            videos,
        })
    }

    /// 動画情報を追加
    ///
    /// # Errors
    /// - 動画の投稿日の年/月がこのVideosSameYearMonthの年/月と異なる場合
    pub(super) fn push_video(
        &mut self,
        video: crate::model::VerifiedVideo,
        duplicate_policy: DuplicateVideoPolicy,
    ) -> Result<(), PushVideoError> {
        self.ensure_same_year_month_from_video(&video)
            .map_err(PushVideoError::YearMonthMismatch)?;

        match duplicate_policy {
            DuplicateVideoPolicy::Reject => self
                .videos
                .insert_video(video)
                .map_err(PushVideoError::DuplicateVideoId),
            DuplicateVideoPolicy::Overwrite => {
                self.videos.upsert_video(video);
                Ok(())
            }
        }
    }

    /// 動画情報を全て置き換え
    ///
    /// `Err`: 動画の投稿日の年/月がこのVideosSameYearMonthの年/月と異なる場合
    pub(super) fn replace_videos(
        &mut self,
        videos: crate::model::VerifiedVideos,
    ) -> Result<(), crate::model::VideoIds> {
        videos.ensure_same_year_month(self.year, self.month)?;
        self.videos = videos;
        Ok(())
    }

    /// 引数の動画の投稿日の年/月が`self`の年/月と同じか
    ///
    /// - `Err(id)`: 同じでないとき. `id`は年/月が同じでない動画の`video_id`
    fn ensure_same_year_month_from_video(
        &self,
        video: &crate::model::VerifiedVideo,
    ) -> Result<(), crate::model::VideoId> {
        if video.get_year() != self.year || video.get_month() != self.month {
            Err(video.get_video_id().clone())
        } else {
            Ok(())
        }
    }
}
