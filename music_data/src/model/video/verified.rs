/// 内部のclipsの整合性が全て取れている動画
///
/// clipsの`start_time`順にソートされていることを保証
#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct VerifiedVideo {
    /// 動画の詳細情報
    #[serde(flatten)]
    video_detail: crate::model::VideoDetail,
    /// クリップ
    ///
    /// `start_time`順にソートされていることを保証
    clips: Vec<crate::model::VerifiedClip>,
}

/// `VerifiedVideo`のリスト
///
/// 内部の情報は`published_at`順にソートされていることを保証
#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct VerifiedVideos(Vec<VerifiedVideo>);

/// `VerifiedVideo`を作ろうとしたときのエラー
#[derive(Debug, Clone)]
pub enum VerifiedVideoError {
    /// クリップの情報が不正
    InvalidClip(Vec<crate::model::VerifiedClipError>),
    /// 動画IDが一致しない
    VideoIdMismatch {
        brief: crate::model::VideoId,
        fetched: crate::model::VideoId,
    },
}

// 以下をデシリアライズ時に行うためのカスタムデシリアライザ
// - video_detailの情報を基にVerifiedClipを作成する必要がある
// - clipsをソート
impl<'de> serde::Deserialize<'de> for VerifiedVideo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[serde(deny_unknown_fields)]
        struct RawVerifiedVideo {
            #[serde(flatten)]
            video_detail: crate::model::VideoDetail,
            clips: Vec<crate::model::UnverifiedClip>,
        }
        let raw = RawVerifiedVideo::deserialize(deserializer)?;
        let mut verified_clips = raw
            .clips
            .into_iter()
            .map(|clip| {
                clip.try_into_verified_clip(
                    raw.video_detail.get_published_at(),
                    raw.video_detail.get_duration(),
                )
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(serde::de::Error::custom)?;
        verified_clips.sort_by_key(|clip| clip.get_start_time().as_secs());
        Ok(VerifiedVideo {
            video_detail: raw.video_detail,
            clips: verified_clips,
        })
    }
}

// published_at順にソートする
impl<'de> serde::Deserialize<'de> for VerifiedVideos {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct RawVerifiedVideos(Vec<VerifiedVideo>);

        let mut raw = RawVerifiedVideos::deserialize(deserializer)?;
        raw.0
            .sort_by_key(|v| v.video_detail.get_published_at().as_secs());
        Ok(VerifiedVideos(raw.0))
    }
}

impl VerifiedVideo {
    pub fn new(
        video_detail: crate::model::VideoDetail,
        mut clips: Vec<crate::model::VerifiedClip>,
    ) -> Self {
        clips.sort_by_key(|clip| clip.get_start_time().as_secs());
        VerifiedVideo {
            video_detail,
            clips,
        }
    }

    pub fn get_year(&self) -> usize {
        self.video_detail.get_published_at().get_year()
    }
    pub fn get_month(&self) -> usize {
        self.video_detail.get_published_at().get_month()
    }
    pub fn get_video_id(&self) -> &crate::model::VideoId {
        self.video_detail.get_video_id()
    }
    pub fn get_published_at(&self) -> &crate::model::VideoPublishedAt {
        self.video_detail.get_published_at()
    }

    pub fn into_clips(self) -> Vec<crate::model::VerifiedClip> {
        self.clips
    }

    /// `AnonymousVideo`と`VideoDetail`から`VerifiedVideo`を作成
    ///
    /// Error:
    /// - `video_detail`の動画IDと`anonymous_video`の動画IDが一致しないとき
    /// - `anonymous_video`のクリップの情報が不正なとき
    pub fn from_anonymous_video(
        anonymous_video: crate::model::AnonymousVideo,
        video_detail: crate::model::VideoDetail,
    ) -> Result<Self, VerifiedVideoError> {
        VerifiedVideoError::ensure_video_id_match(
            anonymous_video.get_video_id(),
            video_detail.get_video_id(),
        )?;
        let (_brief, clips) = anonymous_video.into_inner();

        let (oks, errs): (Vec<_>, Vec<_>) = clips
            .into_iter()
            .map(|clip| {
                clip.try_into_verified_clip(
                    video_detail.get_published_at(),
                    video_detail.get_duration(),
                )
            })
            // ここでoks, errsに分割しているため後方の処理ではそれぞれunwrapを使用
            .partition(Result::is_ok);

        if !errs.is_empty() {
            return Err(VerifiedVideoError::InvalidClip(
                errs.into_iter().map(Result::unwrap_err).collect(),
            ));
        }

        Ok(VerifiedVideo {
            video_detail,
            clips: oks.into_iter().map(Result::unwrap).collect(),
        })
    }
}

impl VerifiedVideos {
    pub fn new(mut videos: Vec<VerifiedVideo>) -> Self {
        videos.sort_by_key(|v| v.video_detail.get_published_at().as_secs());
        VerifiedVideos(videos)
    }

    pub fn into_inner(self) -> Vec<VerifiedVideo> {
        self.0
    }

    pub fn has_video_id(&self, video_id: &crate::model::VideoId) -> bool {
        self.0
            .iter()
            .any(|video| video.video_detail.get_video_id() == video_id)
    }

    pub fn duplicates_videos(&self) -> Vec<&crate::model::VideoId> {
        let mut duplicates = Vec::new();
        let mut seen = std::collections::HashSet::new();
        for video in &self.0 {
            let video_id = video.video_detail.get_video_id();
            if !seen.insert(video_id) {
                duplicates.push(video_id);
            }
        }
        duplicates
    }

    /// 動画を追加
    ///
    /// ソートも一緒に行う
    pub fn push_video(&mut self, video: VerifiedVideo) {
        let pos = self
            .0
            .binary_search_by(|v| {
                v.video_detail
                    .get_published_at()
                    .cmp(video.video_detail.get_published_at())
            })
            .unwrap_or_else(|e| e);
        self.0.insert(pos, video);
    }
}

impl VerifiedVideoError {
    fn ensure_video_id_match(
        expected: &crate::model::VideoId,
        actual: &crate::model::VideoId,
    ) -> Result<(), Self> {
        if expected == actual {
            Ok(())
        } else {
            Err(VerifiedVideoError::VideoIdMismatch {
                brief: expected.clone(),
                fetched: actual.clone(),
            })
        }
    }

    /// 成形して表示する用の文字列をつくる
    ///
    /// 文字列の最後に`\n`が付与される
    pub fn to_pretty_string(&self) -> String {
        let mut msg = "Failed to create VerifiedVideo: ".to_string();
        match self {
            VerifiedVideoError::VideoIdMismatch { brief, fetched } => {
                msg.push_str(&format!(
                    "video_id mismatch: expected {brief}, got {fetched}\n",
                ));
            }
            VerifiedVideoError::InvalidClip(errors) => {
                let invalid_clip_err_msgs =
                    errors.iter().map(|e| e.to_string()).collect::<Vec<_>>();
                msg.push_str(&format!(
                    "Invalid clips found ({}):\n\t{}\n",
                    errors.len(),
                    invalid_clip_err_msgs.join("\n\t")
                ));
            }
        }
        msg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VERIFIED_VIDEO_JSON: &str = r#"
    {
        "videoId": "11111111111",
        "title": "Test Title",
        "channelId": "UC1111111111111111111111",
        "uploaderName": "Test Channel",
        "publishedAt": "2024-12-12T12:00:00Z",
        "modifiedAt": "2024-12-12T12:00:00Z",
        "duration": "PT20H1M5S",
        "privacyStatus": "public",
        "embeddable": true,
        "videoTags": ["Test Video Tag1"],
        "clips": [
            {
                "songTitle": "Test Song 1",
                "artists": ["Aimer Test"],
                "externalArtists": ["Apple Mike"],
                "isClipped": false,
                "startTime": "PT12H12M12S",
                "endTime": "PT12H12M20S",
                "clipTags": ["tag1", "tag2"],
                "uuid": "0193bac8-a560-7000-8000-000000000000"
            }
        ]
    }"#;

    // TODO ちょっとづつ書いてく, いっきにはしんどすぎる...

    #[test]
    fn test_verified_video_deserialization() {
        let _verified_video: VerifiedVideo = serde_json::from_str(VERIFIED_VIDEO_JSON)
            .expect("Failed to deserialize VerifiedVideo");
    }
}
