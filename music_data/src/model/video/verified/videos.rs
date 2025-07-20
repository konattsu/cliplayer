/// `VerifiedVideo`のリスト
///
/// 内部の情報はシリアライズ時に`published_at`順にソートされていることを保証
#[derive(Debug, Clone)]
pub struct VerifiedVideos {
    // into_vecでunsortedな状態を取り出してほしくないのでprivate
    inner: std::collections::HashMap<crate::model::VideoId, super::VerifiedVideo>,
}

impl<'de> serde::Deserialize<'de> for VerifiedVideos {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct RawVerifiedVideos(Vec<super::VerifiedVideo>);

        let raw = RawVerifiedVideos::deserialize(deserializer)?;

        Self::try_from_vec(raw.0)
            .map_err(|e| {
                format!(
                    "Failed to deserialize VerifiedVideos: \
                    found duplicated video_id(s): {}",
                    e.iter()
                        .map(|id| id.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            })
            .map_err(serde::de::Error::custom)
    }
}

impl serde::Serialize for VerifiedVideos {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // serialize時に順番を保証するためにソート
        self.to_sorted_vec().serialize(serializer)
    }
}

impl VerifiedVideos {
    pub fn new_empty() -> Self {
        VerifiedVideos {
            inner: std::collections::HashMap::new(),
        }
    }

    pub fn get_video_ids(&self) -> Vec<crate::model::VideoId> {
        self.inner.keys().cloned().collect()
    }

    /// `VerifiedVideo`のリストを`VerifiedVideos`に変換
    ///
    /// Err: 動画のvideo_idが重複している場合
    pub fn try_from_vec(
        videos: Vec<super::VerifiedVideo>,
    ) -> Result<Self, Vec<crate::model::VideoId>> {
        use std::collections::{HashMap, HashSet};

        let mut inner = HashMap::with_capacity(videos.len());
        let mut duplicated_ids = HashSet::new();

        for video in videos {
            if let Some(prev_video) = inner.insert(video.get_video_id().clone(), video)
            {
                duplicated_ids.insert(prev_video.get_video_id().clone());
            }
        }

        if duplicated_ids.is_empty() {
            Ok(Self { inner })
        } else {
            Err(duplicated_ids.into_iter().collect())
        }
    }

    /// 動画を追加
    ///
    /// Err: 動画のvideo_idが重複している場合
    pub fn push_video(
        &mut self,
        video: super::VerifiedVideo,
    ) -> Result<(), crate::model::VideoId> {
        if let Some(prev_video) = self.inner.insert(video.get_video_id().clone(), video)
        {
            Err(prev_video.get_video_id().clone())
        } else {
            Ok(())
        }
    }

    /// 動画を追加
    ///
    /// Err: 動画のvideo_idが重複している場合
    pub fn extend_videos(
        &mut self,
        videos: VerifiedVideos,
    ) -> Result<(), Vec<crate::model::VideoId>> {
        let mut errs = std::collections::HashSet::new();

        for video in videos.inner.into_values() {
            if let Err(id) = self.push_video(video) {
                errs.insert(id);
            }
        }

        if errs.is_empty() {
            Ok(())
        } else {
            Err(errs.into_iter().collect())
        }
    }

    /// 内部の動画が全て同じ年/月であるかを検証
    ///
    /// Err: 同じ年/月でない動画のvideo_idのリスト
    pub fn ensure_same_year_month(
        &self,
        year: usize,
        month: usize,
    ) -> Result<(), Vec<crate::model::VideoId>> {
        let mut non_same: Vec<crate::model::VideoId> = Vec::new();

        for video in self.inner.values() {
            if video.get_year() != year || video.get_month() != month {
                non_same.push(video.get_video_id().clone());
            }
        }

        if non_same.is_empty() {
            Ok(())
        } else {
            Err(non_same)
        }
    }

    /// 内部の動画をソートして返す
    pub fn into_sorted_vec(self) -> Vec<super::VerifiedVideo> {
        let mut vec = self.inner.into_values().collect::<Vec<_>>();
        vec.sort_by_key(|video| video.get_published_at().as_secs());
        vec
    }

    fn to_sorted_vec(&self) -> Vec<super::VerifiedVideo> {
        let mut vec = self.inner.values().cloned().collect::<Vec<_>>();
        vec.sort_by_key(|video| video.get_published_at().as_secs());
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn different_self() -> VerifiedVideos {
        let video1 = super::super::VerifiedVideo::self_a();
        let video2 = super::super::VerifiedVideo::self_b();
        VerifiedVideos::try_from_vec(vec![video1, video2]).unwrap()
    }

    #[test]
    fn test_verified_videos_try_from_vec_valid() {
        let videos = different_self();
        assert_eq!(videos.get_video_ids().len(), 2);
    }

    #[test]
    fn test_verified_videos_try_from_vec_duplicate() {
        let video1 = super::super::VerifiedVideo::self_a();
        let video2 = super::super::VerifiedVideo::self_a(); // 同じvideo_id
        let result = VerifiedVideos::try_from_vec(vec![video1, video2]);
        assert!(result.is_err());
        let err_ids = result.unwrap_err();
        assert_eq!(err_ids.len(), 1);
        assert_eq!(err_ids[0], crate::model::VideoId::test_id_1());
    }

    #[test]
    fn test_verified_videos_ensure_same_year_month() {
        let videos = different_self();
        let res = videos.ensure_same_year_month(2024, 1);
        let non_same_id = res.unwrap_err();
        assert_eq!(non_same_id.len(), 1);
        assert_eq!(non_same_id[0], crate::model::VideoId::test_id_2());
    }
}
