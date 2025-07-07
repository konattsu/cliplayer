/// 構造と型だけ適している動画情報
#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct VerifiedVideo {
    /// 動画の詳細情報
    #[serde(flatten)]
    video_detail: crate::model::VideoDetail,
    /// クリップ
    clips: Vec<crate::model::VerifiedClip>,
}

impl<'de> serde::Deserialize<'de> for VerifiedVideo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct RawVerifiedVideo {
            #[serde(flatten)]
            video_detail: crate::model::VideoDetail,
            clips: Vec<crate::model::UnverifiedClip>,
        }
        let raw = RawVerifiedVideo::deserialize(deserializer)?;
        raw.clips
            .into_iter()
            .map(|clip| {
                clip.try_into_verified_clip(
                    raw.video_detail.get_published_at(),
                    raw.video_detail.get_duration(),
                )
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|clips| VerifiedVideo {
                video_detail: raw.video_detail,
                clips,
            })
            .map_err(serde::de::Error::custom)
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
        "channelName": "Test Channel",
        "publishedAt": "2024-12-12T12:00:00Z",
        "modifiedAt": "2024-12-12T12:00:00Z",
        "duration": "PT20H1M5S",
        "privacyStatus": "public",
        "embeddable": true,
        "videoTags": ["Test Video Tag1"],
        "clips": [
            {
                "songTitle": "Test Song 1",
                "songTitleJah": "てすとそんぐいち",
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

    // ちょっとづつ書いてく, いっきにはしんどすぎる...

    #[test]
    fn test_verified_video_deserialization() {
        let _verified_video: VerifiedVideo = serde_json::from_str(VERIFIED_VIDEO_JSON)
            .expect("Failed to deserialize VerifiedVideo");
    }
}
