/// 構造と型だけ適している動画情報
///
/// 内部のclipsが`stat_time`順でソートされていることを保証
#[derive(Debug, Clone)]
pub struct AnonymousVideo {
    /// 動画の情報
    video_brief: super::VideoBrief,
    /// クリップ
    clips: Vec<crate::model::AnonymousClip>,
}

/// `AnonymousVideo`のリスト
///
/// 内部の動画のvideo_idは重複しないことを保証

#[derive(Debug, Clone)]
pub struct AnonymousVideos {
    pub inner: std::collections::HashMap<crate::model::VideoId, AnonymousVideo>,
}

// deserialize時にclipsをソートするためのカスタムデシリアライザ
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
            video_brief: super::VideoBrief,
            clips: Vec<crate::model::AnonymousClip>,
        }

        let raw = RawAnonymousVideo::deserialize(deserializer)?;
        let mut clips = raw.clips;
        clips.sort_by_key(|clip| clip.get_start_time().as_secs());

        Ok(AnonymousVideo {
            video_brief: raw.video_brief,
            clips,
        })
    }
}

impl AnonymousVideo {
    pub fn get_video_brief(&self) -> &super::VideoBrief {
        &self.video_brief
    }

    pub fn get_video_id(&self) -> &crate::model::VideoId {
        self.video_brief.get_video_id()
    }

    pub fn into_inner(self) -> (super::VideoBrief, Vec<crate::model::AnonymousClip>) {
        (self.video_brief, self.clips)
    }
}

impl AnonymousVideos {
    /// `AnonymousVideo`のリストを`AnonymousVideos`に変換
    ///
    /// Err: 動画のvideo_idが重複している場合
    pub fn try_from_vec(
        videos: Vec<AnonymousVideo>,
    ) -> Result<Self, Vec<crate::model::VideoId>> {
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
            Err(duplicated_ids.into_iter().collect())
        }
    }

    pub fn to_video_ids(&self) -> Vec<crate::model::VideoId> {
        self.inner.keys().cloned().collect()
    }

    pub fn to_briefs(&self) -> crate::model::VideoBriefs {
        let briefs = self
            .inner
            .values()
            .map(|v| v.get_video_brief())
            .cloned()
            .collect();

        // `Self(AnonymousVideos)`も`VideoBriefs`もvideo_idの重複を許可しないので
        // `Self`は`VideoBriefs`に変換するための十分条件を満たしている
        crate::model::VideoBriefs::try_from_vec(briefs).expect("will not fail")
    }
}
