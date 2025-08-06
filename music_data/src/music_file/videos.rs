/// 投稿日の年/月が同じ動画の情報を保持
#[derive(Debug, Clone)]
pub(crate) struct VideosSameYearMonth {
    year: usize,
    month: usize,
    videos: crate::model::VerifiedVideos,
}

impl VideosSameYearMonth {
    pub(crate) fn get_year(&self) -> usize {
        self.year
    }
    pub(crate) fn get_month(&self) -> usize {
        self.month
    }
    pub(crate) fn get_videos(&self) -> &crate::model::VerifiedVideos {
        &self.videos
    }
    pub(crate) fn into_videos(self) -> crate::model::VerifiedVideos {
        self.videos
    }

    pub(super) fn len(&self) -> usize {
        self.videos.len()
    }

    /// 新規作成
    ///
    /// `Err`:
    /// 動画投稿日の年/月が引数に対応していないとき. 対応していない動画idのリストを返す.
    pub(crate) fn new(
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

    /// 動画情報が空の状態で新規作成
    pub(crate) fn new_empty(year: usize, month: usize) -> Self {
        Self {
            year,
            month,
            videos: crate::model::VerifiedVideos::new(),
        }
    }

    /// 動画情報を追加
    ///
    /// - 動画のvideo_idが重複していれば上書き
    ///
    /// # Errors
    /// - 動画の投稿日の年/月がこのVideosSameYearMonthの年/月と異なる場合
    pub(crate) fn push_video(
        &mut self,
        video: crate::model::VerifiedVideo,
    ) -> Result<(), crate::model::VideoId> {
        self.ensure_same_year_month_from_video(&video)?;
        if let Some(prev_video) = self.videos.insert_video(video) {
            tracing::trace!(
                "Video with id `{}` already exists, \
                replacing stale video with new one.\n\
                Stale video: {:?}",
                prev_video.get_video_id(),
                prev_video
            );
        }
        Ok(())
    }

    /// 動画情報を置き換え
    ///
    /// `Err`: 動画の投稿日の年/月がこのVideosSameYearMonthの年/月と異なる場合
    pub(crate) fn replace_videos(
        &mut self,
        videos: crate::model::VerifiedVideos,
    ) -> Result<(), crate::model::VideoIds> {
        videos.ensure_same_year_month(self.year, self.month)?;
        self.videos = videos;
        Ok(())
    }

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
