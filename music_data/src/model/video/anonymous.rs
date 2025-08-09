/// 構造と型だけ適している動画情報
///
/// 内部のclipsが`stat_time`順でソートされていることを保証
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct AnonymousVideo {
    /// 動画の情報
    local_record: super::LocalVideoInfo,
    /// クリップ
    clips: Vec<crate::model::AnonymousClip>,
}

/// `AnonymousVideo`のリスト
///
/// 内部の動画のvideo_idは重複しないことを保証
#[derive(Debug, Clone)]
pub struct AnonymousVideos {
    inner: std::collections::HashMap<crate::model::VideoId, AnonymousVideo>,
}

#[derive(thiserror::Error, Debug)]
/// `AnonymousVideo`動画の検証エラー
pub(crate) enum AnonymousVideoValidateError {
    /// 動画idが重複
    #[error("Duplicate video ID(s): {0}")]
    DuplicateVideoId(crate::model::VideoIds),
    /// クリップを持たない
    #[error("Video ID {0} has no clips")]
    NoClips(crate::model::VideoId),
    /// クリップの指定時間が重複している
    #[error("Clips overlap in video ID {id}: song titles`{clips_titles:?}`")]
    ClipsOverlap {
        id: crate::model::VideoId,
        clips_titles: Vec<String>,
    },
}

// `Self`の存在条件を検証するためのカスタムデシリアライザ
impl<'de> serde::Deserialize<'de> for AnonymousVideo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[serde(deny_unknown_fields)]
        struct RawAnonymousVideo {
            #[serde(flatten)]
            video_brief: super::LocalVideoInfo,
            clips: Vec<crate::model::AnonymousClip>,
        }

        let raw = RawAnonymousVideo::deserialize(deserializer)?;
        let video_brief = raw.video_brief;
        let clips = raw.clips;
        AnonymousVideo::new(video_brief, clips).map_err(serde::de::Error::custom)
    }
}

impl serde::Serialize for AnonymousVideo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct SerializableAnonymousVideo<'a> {
            #[serde(flatten)]
            video_brief: &'a super::LocalVideoInfo,
            clips: Vec<&'a crate::model::AnonymousClip>,
        }

        let clips = self.to_sorted_clips();
        let serializable = SerializableAnonymousVideo {
            video_brief: &self.local_record,
            clips: clips.iter().collect::<Vec<_>>(),
        };

        serializable
            .serialize(serializer)
            .map_err(serde::ser::Error::custom)
    }
}

impl AnonymousVideo {
    pub(crate) fn get_video_id(&self) -> &crate::model::VideoId {
        self.local_record.get_video_id()
    }
    pub(crate) fn into_inner(
        self,
    ) -> (super::LocalVideoInfo, Vec<crate::model::AnonymousClip>) {
        (self.local_record, self.clips)
    }

    /// `AnonymousVideo`を生成
    ///
    /// `clips`は`start_time`順にソートされていることを保証
    ///
    /// # Errors:
    /// - クリップの指定時間が重複しているとき
    /// - クリップが空のとき
    pub(crate) fn new(
        video_brief: super::LocalVideoInfo,
        clips: Vec<crate::model::AnonymousClip>,
    ) -> Result<Self, AnonymousVideoValidateError> {
        let mut clips = clips;
        clips.sort_by_key(|clip| clip.get_start_time().as_secs());
        Self::validate_consistency(&clips, &video_brief)?;
        Ok(Self {
            local_record: video_brief,
            clips,
        })
    }

    /// 動画のクリップの整合性を検証
    ///
    /// - クリップに指定した範囲が重複していないか
    /// - クリップが空でないか
    fn validate_consistency(
        clips: &[crate::model::AnonymousClip],
        video_brief: &super::LocalVideoInfo,
    ) -> Result<(), AnonymousVideoValidateError> {
        // クリップが空でないか
        if clips.is_empty() {
            return Err(AnonymousVideoValidateError::NoClips(
                video_brief.get_video_id().clone(),
            ));
        }

        // クリップの指定時間が重複していないか
        let mut overlap_clips = Vec::new();

        for w in clips.windows(2) {
            if w[0].get_end_time() > w[1].get_start_time() {
                overlap_clips.push(w[0].clone());
                overlap_clips.push(w[1].clone());
            }
        }
        if overlap_clips.is_empty() {
            Ok(())
        } else {
            Err(AnonymousVideoValidateError::ClipsOverlap {
                id: video_brief.get_video_id().clone(),
                clips_titles: overlap_clips
                    .iter()
                    .map(|c| c.get_song_title().to_string())
                    .collect(),
            })
        }
    }

    fn to_sorted_clips(&self) -> Vec<crate::model::AnonymousClip> {
        let mut vec = self.clips.clone();
        vec.sort_by_key(|clip| clip.get_start_time().as_secs());
        vec
    }
}

// `Self`の存在条件を検証するためのカスタムデシリアライザ
impl<'de> serde::Deserialize<'de> for AnonymousVideos {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        struct RawAnonymousVideos(Vec<AnonymousVideo>);

        let raw = RawAnonymousVideos::deserialize(deserializer)?;
        AnonymousVideos::try_from_vec(raw.0).map_err(serde::de::Error::custom)
    }
}

impl serde::Serialize for AnonymousVideos {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_sorted_vec().serialize(serializer)
    }
}

impl AnonymousVideos {
    /// 空の`AnonymousVideos`を生成
    pub(crate) fn new() -> Self {
        Self {
            inner: std::collections::HashMap::new(),
        }
    }

    pub(crate) fn into_inner(
        self,
    ) -> std::collections::HashMap<crate::model::VideoId, AnonymousVideo> {
        self.inner
    }

    /// `AnonymousVideo`のリストを`AnonymousVideos`に変換
    ///
    /// Err: 動画のvideo_idが重複している場合
    pub(crate) fn try_from_vec(
        videos: Vec<AnonymousVideo>,
    ) -> Result<Self, AnonymousVideoValidateError> {
        use std::collections::{HashMap, HashSet};

        let mut inner = HashMap::with_capacity(videos.len());
        let mut duplicated_ids = HashSet::new();

        for video in videos {
            if let Some(prev_video) = inner.insert(video.get_video_id().clone(), video)
            {
                // 重複の有無のみ検出したく, すでに重複しているか(3回,同じ動画IDが来たとき)どうかは
                // 気にしないのでinsertの結果は無視
                let _res = duplicated_ids.insert(prev_video.get_video_id().clone());
            }
        }

        if duplicated_ids.is_empty() {
            Ok(Self { inner })
        } else {
            Err(AnonymousVideoValidateError::DuplicateVideoId(
                duplicated_ids.into_iter().collect(),
            ))
        }
    }

    /// 動画をextend
    ///
    /// 動画idが重複していれば上書き
    ///
    /// - Returns: 重複していた動画IDのリスト
    pub(crate) fn extend(
        &mut self,
        videos: AnonymousVideos,
    ) -> Option<crate::model::VideoIds> {
        let mut duplicated_ids = Vec::new();

        for video in videos.into_inner() {
            if let Some(stale_video) = self.inner.insert(video.0, video.1) {
                duplicated_ids.push(stale_video.get_video_id().clone());
            }
        }

        if duplicated_ids.is_empty() {
            None
        } else {
            Some(duplicated_ids.into_iter().collect())
        }
    }

    pub(crate) fn to_video_ids(&self) -> crate::model::VideoIds {
        self.inner.keys().cloned().collect()
    }

    fn to_sorted_vec(&self) -> Vec<AnonymousVideo> {
        let mut vec: Vec<_> = self.inner.values().cloned().collect();
        vec.sort_by_key(|video| video.get_video_id().clone());
        vec
    }
}

// MARK: Tests

#[cfg(test)]
mod tests {
    use super::*;

    fn make_brief_json() -> serde_json::Value {
        serde_json::json!({
            "videoId": crate::model::VideoId::test_id_1().to_string(),
            "videoTags": ["Test Video Tag1"]
        })
    }

    fn make_clip_json() -> serde_json::Value {
        serde_json::json!(crate::model::AnonymousClip::self_a_1())
    }

    #[test]
    fn test_deserialize_anonymous_video() {
        let mut v = make_brief_json();
        v["clips"] = serde_json::json!([make_clip_json()]);
        let av: AnonymousVideo = serde_json::from_value(v).unwrap();
        assert_eq!(av.get_video_id(), &crate::model::VideoId::test_id_1());
        assert_eq!(av.clips.len(), 1);
    }

    #[test]
    fn test_deserialize_anonymous_videos() {
        let mut v = make_brief_json();
        v["clips"] = serde_json::json!([make_clip_json()]);
        let arr = serde_json::json!([v]);
        let avs: AnonymousVideos = serde_json::from_value(arr).unwrap();
        assert_eq!(avs.inner.len(), 1);
        assert!(avs.inner.contains_key(&crate::model::VideoId::test_id_1()));
    }

    #[test]
    fn test_deserialize_anonymous_video_error_no_clips() {
        let mut v = make_brief_json();
        v["clips"] = serde_json::json!([]);
        let res: Result<AnonymousVideo, _> = serde_json::from_value(v);
        assert!(res.is_err());
    }

    #[test]
    fn test_deserialize_anonymous_videos_error_duplicate_id() {
        let mut v = make_brief_json();
        v["clips"] = serde_json::json!([make_clip_json()]);
        let arr = serde_json::json!([v.clone(), v]);
        let res: Result<AnonymousVideos, _> = serde_json::from_value(arr);
        assert!(res.is_err());
    }

    #[test]
    fn test_anonymous_video_new() {
        let brief = crate::model::LocalVideoInfo::new(
            crate::model::VideoId::test_id_1(),
            None,
            crate::model::VideoTags::self_1(),
        );
        let clips = vec![
            // ソートされてない
            crate::model::AnonymousClip::self_a_3(),
            crate::model::AnonymousClip::self_a_1(),
            crate::model::AnonymousClip::self_a_2(),
        ];
        let video =
            AnonymousVideo::new(brief, clips).expect("should create successfully");

        assert_eq!(video.get_video_id(), &crate::model::VideoId::test_id_1());
        assert_eq!(video.clips.len(), 3);
        // ソートを確認
        assert_eq!(video.clips[0], crate::model::AnonymousClip::self_a_1());
        assert_eq!(video.clips[1], crate::model::AnonymousClip::self_a_2());
        assert_eq!(video.clips[2], crate::model::AnonymousClip::self_a_3());
    }

    #[test]
    fn test_anonymous_video_new_with_overlapping_clips() {
        let brief = crate::model::LocalVideoInfo::new(
            crate::model::VideoId::test_id_1(),
            None,
            crate::model::VideoTags::self_1(),
        );
        let clips = vec![
            crate::model::AnonymousClip::self_a_1(),
            crate::model::AnonymousClip::self_a_2(),
            // 重複しているクリップ
            crate::model::AnonymousClip::self_a_1(),
        ];
        let result = AnonymousVideo::new(brief, clips);
        assert!(matches!(
            result,
            Err(AnonymousVideoValidateError::ClipsOverlap { .. })
        ));
    }

    #[test]
    fn test_anonymous_video_new_with_empty_clips() {
        let brief = crate::model::LocalVideoInfo::new(
            crate::model::VideoId::test_id_1(),
            None,
            crate::model::VideoTags::self_1(),
        );
        let clips = Vec::new();
        let result = AnonymousVideo::new(brief, clips);
        assert!(matches!(
            result,
            Err(AnonymousVideoValidateError::NoClips(_))
        ));
    }

    #[test]
    fn test_anonymous_videos_try_from_vec() {
        let video1 = AnonymousVideo::new(
            crate::model::LocalVideoInfo::new(
                crate::model::VideoId::test_id_1(),
                None,
                crate::model::VideoTags::self_1(),
            ),
            vec![crate::model::AnonymousClip::self_a_1()],
        )
        .expect("should create successfully");
        let video2 = AnonymousVideo::new(
            crate::model::LocalVideoInfo::new(
                crate::model::VideoId::test_id_2(),
                None,
                crate::model::VideoTags::self_2(),
            ),
            vec![crate::model::AnonymousClip::self_b_1()],
        )
        .expect("should create successfully");

        let videos = vec![video1, video2];
        let anonymous_videos =
            AnonymousVideos::try_from_vec(videos).expect("should create successfully");
        assert_eq!(anonymous_videos.inner.len(), 2);
        assert!(
            anonymous_videos
                .inner
                .contains_key(&crate::model::VideoId::test_id_1())
        );
        assert!(
            anonymous_videos
                .inner
                .contains_key(&crate::model::VideoId::test_id_2())
        );
    }

    #[test]
    fn test_anonymous_videos_try_from_vec_with_duplicates() {
        let video1 = AnonymousVideo::new(
            crate::model::LocalVideoInfo::new(
                crate::model::VideoId::test_id_1(),
                None,
                crate::model::VideoTags::self_1(),
            ),
            vec![crate::model::AnonymousClip::self_a_1()],
        )
        .expect("should create successfully");
        let video2 = AnonymousVideo::new(
            crate::model::LocalVideoInfo::new(
                crate::model::VideoId::test_id_1(), // 重複するID
                None,
                crate::model::VideoTags::self_2(),
            ),
            vec![crate::model::AnonymousClip::self_b_1()],
        )
        .expect("should create successfully");

        let videos = vec![video1, video2];
        let result = AnonymousVideos::try_from_vec(videos);
        assert!(matches!(
            result,
            Err(AnonymousVideoValidateError::DuplicateVideoId(_))
        ));
    }

    #[test]
    fn test_anonymous_videos_extend() {
        // Prepare initial videos
        let video1 = AnonymousVideo::new(
            crate::model::LocalVideoInfo::new(
                crate::model::VideoId::test_id_1(),
                None,
                crate::model::VideoTags::self_1(),
            ),
            vec![crate::model::AnonymousClip::self_a_1()],
        )
        .expect("should create successfully");
        let video2 = AnonymousVideo::new(
            crate::model::LocalVideoInfo::new(
                crate::model::VideoId::test_id_2(),
                None,
                crate::model::VideoTags::self_2(),
            ),
            vec![crate::model::AnonymousClip::self_b_1()],
        )
        .expect("should create successfully");
        let mut videos =
            AnonymousVideos::try_from_vec(vec![video1.clone(), video2.clone()])
                .expect("should create successfully");

        // Prepare videos to extend (one duplicate, one new)
        let video2_updated = AnonymousVideo::new(
            crate::model::LocalVideoInfo::new(
                crate::model::VideoId::test_id_2(),
                None,
                crate::model::VideoTags::self_2(),
            ),
            vec![
                crate::model::AnonymousClip::self_b_1(),
                crate::model::AnonymousClip::self_b_2(),
            ],
        )
        .expect("should create successfully");
        let video3 = AnonymousVideo::new(
            crate::model::LocalVideoInfo::new(
                crate::model::VideoId::test_id_3(),
                None,
                crate::model::VideoTags::self_3(),
            ),
            vec![crate::model::AnonymousClip::self_a_1()],
        )
        .expect("should create successfully");
        let extend_videos =
            AnonymousVideos::try_from_vec(vec![video2_updated.clone(), video3.clone()])
                .expect("should create successfully");

        // Extend and check result
        let result = videos.extend(extend_videos);
        assert!(result.is_some());
        let dupes = result.unwrap();
        assert!(
            dupes
                .iter()
                .any(|id| id == &crate::model::VideoId::test_id_2())
        );
        assert_eq!(dupes.len(), 1);

        // The new video should be added
        assert!(
            videos
                .inner
                .contains_key(&crate::model::VideoId::test_id_3())
        );
        // The duplicate video should be updated
        assert_eq!(
            videos
                .inner
                .get(&crate::model::VideoId::test_id_2())
                .unwrap()
                .clips
                .len(),
            2
        );
    }

    #[test]
    fn test_anonymous_video_serialize_sorted_clips() {
        let brief = crate::model::LocalVideoInfo::new(
            crate::model::VideoId::test_id_1(),
            Some(crate::model::UploaderName::test_uploader_name_1()),
            crate::model::VideoTags::self_1(),
        );
        // clips: ソートされていない順で作成
        let clips = vec![
            crate::model::AnonymousClip::self_a_3(),
            crate::model::AnonymousClip::self_a_1(),
            crate::model::AnonymousClip::self_a_2(),
        ];
        let video =
            AnonymousVideo::new(brief, clips).expect("should create successfully");
        let serialized =
            serde_json::to_string(&video).expect("should serialize successfully");

        let brief = crate::model::LocalVideoInfo::new(
            crate::model::VideoId::test_id_1(),
            Some(crate::model::UploaderName::test_uploader_name_1()),
            crate::model::VideoTags::self_1(),
        );
        let clips = vec![
            crate::model::AnonymousClip::self_a_1(),
            crate::model::AnonymousClip::self_a_2(),
            crate::model::AnonymousClip::self_a_3(),
        ];
        let expected_video =
            AnonymousVideo::new(brief, clips).expect("should create successfully");

        assert_eq!(
            serde_json::from_str::<AnonymousVideo>(&serialized).unwrap(),
            expected_video
        );
    }
}
