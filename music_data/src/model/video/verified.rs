/// 内部のclipsの整合性が全て取れている動画
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

// video_detailの情報を基にVerifiedClipを作成する必要があり
// これをデシリアライズに行うためのカスタムデシリアライザ
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

impl VerifiedVideos {
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
