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
    pub(crate) fn len(&self) -> usize {
        self.inner.len()
    }

    /// 新規作成
    pub(crate) fn new() -> Self {
        Self {
            inner: std::collections::HashMap::new(),
        }
    }

    /// `AnonymousVideos`と`ApiVideoInfo`から`VerifiedVideo`を作成
    ///
    /// # Error:
    /// - `anonymous_video`のクリップの情報が不正なとき
    /// - `AnonymousVideos`, `ApiVideoInfo`が完全に対応しないとき
    pub(crate) fn from_anonymous_video(
        anonymous_videos: crate::model::AnonymousVideos,
        api_info_list: crate::model::ApiVideoInfoList,
    ) -> Result<Self, super::error::VerifiedVideoErrors> {
        let mut videos = Vec::new();
        let mut errs = Vec::new();
        let mut api_info_list = api_info_list.inner;

        for (id, anonymous_video) in anonymous_videos.into_inner() {
            // `anonymous_video`に対応した`api_info`があるとき
            if let Some(api_info) = api_info_list.remove(&id) {
                match crate::model::VerifiedVideo::from_anonymous_video(
                    anonymous_video,
                    api_info,
                ) {
                    // 正常にverifyできたとき
                    Ok(video) => videos.push(video),
                    // verifyできなかったとき
                    Err(e) => errs.push(e),
                }
            // `anonymous_video`に対応した`api_info`がないとき
            } else {
                errs.push(crate::model::VerifiedVideoError::MissingApiInfo(id));
            }
            // NOTE api_info_listのみに動画idが存在しても検出できない機構やけどまぁいらんと思う
        }

        if errs.is_empty() {
            // `anonymous_videos`の時点でvideo_idが重複してない(hashmapで管理)ので
            // 変換の際にも重複しないからunwrap
            let videos = crate::model::VerifiedVideos::try_from_vec(videos).unwrap();
            Ok(videos)
        } else {
            Err(errs.into())
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

    /// 動画の詳細情報を更新
    pub(crate) fn with_new_api_info_list(
        self,
        mut api_list: crate::model::ApiVideoInfoList,
    ) -> Result<Self, super::error::VerifiedVideoErrors> {
        let mut new_videos = Vec::new();
        let mut errs = Vec::new();

        for video in self.inner.into_values() {
            let api_info = api_list.inner.remove(video.get_video_id());
            // 対応する動画の詳細情報が見つかったとき
            if let Some(api_info) = api_info {
                match video.with_new_api_info(api_info) {
                    // 成功したとき
                    Ok(new_video) => new_videos.push(new_video),
                    // 動画の詳細情報は見つかったが失敗したとき
                    Err(e) => errs.push(e),
                }
            // 対応する動画の詳細情報が見つからなかったとき
            } else {
                errs.push(super::VerifiedVideoError::MissingApiInfo(
                    video.get_video_id().clone(),
                ));
            }
        }

        if errs.is_empty() {
            // 引数の`self`で動画idは一意であり, それを順に処理しているため`new_videos`も一意
            // そのため失敗することはない
            Ok(Self::try_from_vec(new_videos).unwrap())
        } else {
            Err(errs.into())
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
