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
#[derive(serde::Deserialize, Debug, Clone)]
pub struct AnonymousVideos {
    videos: Vec<AnonymousVideo>,
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
    pub fn new(videos: Vec<AnonymousVideo>) -> Self {
        AnonymousVideos { videos }
    }

    pub fn into_inner(self) -> Vec<AnonymousVideo> {
        self.videos
    }

    pub fn to_video_ids(&self) -> Vec<crate::model::VideoId> {
        self.videos
            .iter()
            .map(|v| v.get_video_id())
            .cloned()
            .collect()
    }

    pub fn to_briefs(&self) -> Vec<super::VideoBrief> {
        self.videos
            .iter()
            .map(|v| v.get_video_brief())
            .cloned()
            .collect()
    }
}
