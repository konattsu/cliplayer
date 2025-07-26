/// `VerifiedVideo`のリスト
///
/// 内部の情報はシリアライズ時に`published_at`順にソートされていることを保証
#[derive(Debug, Clone)]
pub(crate) struct VerifiedVideos {
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
                    found duplicated video_id(s): {e}",
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
    /// 新規作成
    pub(crate) fn new() -> Self {
        Self {
            inner: std::collections::HashMap::new(),
        }
    }

    /// `VerifiedVideo`のリストを`VerifiedVideos`に変換
    ///
    /// Err: 動画のvideo_idが重複している場合
    pub(crate) fn try_from_vec(
        videos: Vec<super::VerifiedVideo>,
    ) -> Result<Self, crate::model::VideoIds> {
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
            Err(duplicated_ids
                .into_iter()
                .collect::<Vec<crate::model::VideoId>>()
                .into())
        }
    }

    /// 動画を追加
    ///
    /// 動画のvideo_idが重複していれば上書き
    ///
    /// # Returns
    /// - `Some(動画)`: 追加前に同じvideo_idの動画が存在していた場合
    /// - `None`: 新規追加の場合
    pub(crate) fn insert_video(
        &mut self,
        video: super::VerifiedVideo,
    ) -> Option<super::VerifiedVideo> {
        self.inner.insert(video.get_video_id().clone(), video)
    }

    /// 動画情報を動画idを基に削除
    ///
    /// `Some(VerifiedVideo)`: 動画idが存在していた場合
    pub(crate) fn delete_video(
        &mut self,
        video_id: &crate::model::VideoId,
    ) -> Option<super::VerifiedVideo> {
        self.inner.remove(video_id)
    }

    /// 内部の動画が全て同じ年/月であるかを検証
    ///
    /// Err: 同じ年/月でない動画のvideo_idのリスト
    pub(crate) fn ensure_same_year_month(
        &self,
        year: usize,
        month: usize,
    ) -> Result<(), crate::model::VideoIds> {
        let mut non_same: Vec<crate::model::VideoId> = Vec::new();

        for video in self.inner.values() {
            if video.get_year() != year || video.get_month() != month {
                non_same.push(video.get_video_id().clone());
            }
        }

        if non_same.is_empty() {
            Ok(())
        } else {
            Err(non_same.into())
        }
    }

    /// 内部の動画をソートして返す
    pub(crate) fn into_sorted_vec(self) -> Vec<super::VerifiedVideo> {
        let mut vec = self.inner.into_values().collect::<Vec<_>>();
        vec.sort_by_key(|video| video.get_published_at().as_secs());
        vec
    }

    pub(crate) fn to_video_ids(&self) -> crate::model::VideoIds {
        self.inner
            .keys()
            .cloned()
            .collect::<Vec<crate::model::VideoId>>()
            .into()
    }

    fn to_sorted_vec(&self) -> Vec<super::VerifiedVideo> {
        let mut vec = self.inner.values().cloned().collect::<Vec<_>>();
        vec.sort_by_key(|video| video.get_published_at().as_secs());
        vec
    }
}

// MARK: For Tests

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
        assert_eq!(videos.to_video_ids().into_vec().len(), 2);
    }

    #[test]
    fn test_verified_videos_try_from_vec_duplicate() {
        let video1 = super::super::VerifiedVideo::self_a();
        let video2 = super::super::VerifiedVideo::self_a(); // 同じvideo_id
        let result = VerifiedVideos::try_from_vec(vec![video1, video2]);
        assert!(result.is_err());
        let err_ids = result.unwrap_err().into_vec();
        assert_eq!(err_ids.len(), 1);
        assert_eq!(err_ids[0], crate::model::VideoId::test_id_1());
    }

    #[test]
    fn test_verified_videos_ensure_same_year_month() {
        let videos = different_self();
        let res = videos.ensure_same_year_month(2024, 1);
        let non_same_id: Vec<crate::model::VideoId> = res.unwrap_err().into();
        assert_eq!(non_same_id.len(), 1);
        assert_eq!(non_same_id[0], crate::model::VideoId::test_id_2());
    }
}
