// 付け忘れが怖いので rename_all は全構造体に適用
// ref: https://developers.google.com/youtube/v3/docs/videos

#[derive(Debug, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct YouTubeApiResponse {
    pub items: Vec<YouTubeApiItem>,
}

#[derive(Debug, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct YouTubeApiItem {
    pub id: crate::model::VideoId,
    pub snippet: YouTubeApiSnippet,
    pub content_details: YouTubeApiContentDetails,
    pub status: YouTubeApiStatus,
}

#[derive(Debug, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct YouTubeApiSnippet {
    pub published_at: crate::model::VideoPublishedAt,
    pub channel_id: crate::model::ChannelId,
    pub title: String,
}

#[derive(Debug, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct YouTubeApiContentDetails {
    pub duration: crate::model::Duration,
}

#[derive(Debug, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct YouTubeApiStatus {
    pub privacy_status: crate::model::PrivacyStatus,
    pub embeddable: bool,
}

impl YouTubeApiItem {
    pub(super) fn into_fetched_video_detail(
        self,
    ) -> crate::fetcher::FetchedVideoDetail {
        crate::fetcher::FetchedVideoDetailInitializer {
            video_id: self.id,
            title: self.snippet.title,
            channel_id: self.snippet.channel_id,
            published_at: self.snippet.published_at,
            modified_at: chrono::Utc::now(),
            duration: self.content_details.duration,
            privacy_status: self.status.privacy_status,
            embeddable: self.status.embeddable,
        }
        .init()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 実際のレスポンスから不要な情報は落としている
    const RESPONSE_1: &str = r#"{
    "items": [
        {
            "id": "gKkAif-7ZMM",
            "snippet": {
                "publishedAt": "2025-06-02T17:11:06Z",
                "channelId": "UC1111111111111111111111",
                "title": "Title 1"
            },
            "contentDetails": {
                "duration": "PT2H57M13S"
            },
            "status": {
                "privacyStatus": "public",
                "embeddable": true
            }
        }
    ]
}
"#;

    const RESPONSE_2: &str = r#"{
    "items": [
        {
            "id": "KR_Xjy0ZjuI",
            "snippet": {
                "publishedAt": "2025-06-28T17:02:38Z",
                "channelId": "UC2222222222222222222222",
                "title": "Title 2"
            },
            "contentDetails": {
                "duration": "PT3H49M47S"
            },
            "status": {
                "privacyStatus": "public",
                "embeddable": true
            }
        },
        {
            "id": "fmhYOo3Gy2a",
            "snippet": {
                "publishedAt": "2025-06-29T08:30:23Z",
                "channelId": "UC3333333333333333333333",
                "channelTitle": "Channel Name 3",
                "title": "Title 3"
            },
            "contentDetails": {
                "duration": "PT3M6S"
            },
            "status": {
                "privacyStatus": "public",
                "embeddable": true
            }
        }
    ]
}
"#;

    #[test]
    fn test_youtube_api_response_deserialize() {
        let _response_1 =
            serde_json::from_str::<YouTubeApiResponse>(RESPONSE_1).unwrap();
        let _response_2 =
            serde_json::from_str::<YouTubeApiResponse>(RESPONSE_2).unwrap();
    }
}
