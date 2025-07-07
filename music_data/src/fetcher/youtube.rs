/// youtube apiを呼び出すときに使う定数たち
mod yt_api_const {
    pub const ENDPOINT: &str = "https://www.googleapis.com/youtube/v3/videos";
    pub const PARTS: &str = "snippet,contentDetails,status";
    pub const MAX_RESULTS: u8 = 50;
}

// 複雑なライフタイム制約は書かない. メモリ速度, 処理速度は知らない子

#[derive(Debug)]
pub struct YouTubeApi {
    api_key: crate::fetcher::YouTubeApiKey,
}

impl YouTubeApi {
    pub fn new(api_key: crate::fetcher::YouTubeApiKey) -> Self {
        Self { api_key }
    }

    pub async fn run(
        self,
        video_ids: Vec<crate::model::VideoId>,
    ) -> Result<crate::fetcher::VideoDetailFetchResult, crate::fetcher::YouTubeApiError>
    {
        let mut detail_result: crate::fetcher::VideoDetailFetchResult =
            video_ids.into_iter().map(|id| (id, None)).collect();

        match self.fetch_process(&mut detail_result).await {
            Ok(..) => {
                tracing::info!("YouTube API fetch completed successfully");
                Ok(detail_result)
            }
            Err(e) => {
                tracing::error!(error = ?e, "YouTube API fetch failed");
                Err(e)
            }
        }
    }

    async fn fetch_process(
        &self,
        detail_result: &mut crate::fetcher::VideoDetailFetchResult,
    ) -> Result<(), crate::fetcher::YouTubeApiError> {
        const MAX_RETRY: u8 = 3;
        const REQUEST_DELAY: tokio::time::Duration =
            tokio::time::Duration::from_millis(125);
        const REQUEST_DELAY_RETRY: tokio::time::Duration =
            tokio::time::Duration::from_millis(500);

        let mut pending_ids: Vec<crate::model::VideoId> =
            detail_result.0.keys().cloned().collect();

        // urlを作れなくなるまでループ
        loop {
            let url = match self.generate_url(&mut pending_ids) {
                Some(url) => url,
                None => {
                    tracing::info!("no more video IDs to fetch");
                    break;
                }
            };

            let mut retry_count = 0;
            // リトライ用のループ, 正常なときはループせず抜ける
            loop {
                match self.fetch_and_parse(&url).await {
                    Ok(resp) => {
                        // とってきたレスポンスをdetail_resultのvideo_idと対応するように格納
                        resp.items.into_iter().for_each(|item| {
                            let video_id = &item.id;
                            if let Some(slot) = detail_result.0.get_mut(video_id) {
                                *slot = Some(item.into_fetched_video_detail());
                            } else {
                                tracing::warn!(
                                    "Received video ID {} not found in pending IDs",
                                    video_id
                                );
                            }
                        });
                        break;
                    }
                    Err(e) => {
                        retry_count += 1;
                        if retry_count >= MAX_RETRY {
                            tracing::error!(error = ?e, "YouTube API fetch error after retries");
                            return Err(e);
                        }
                        tracing::warn!(
                            error = ?e,
                            "YouTube API fetch error, retrying... (attempt {}/{})",
                            retry_count,
                            MAX_RETRY
                        );
                        tokio::time::sleep(REQUEST_DELAY_RETRY).await;
                    }
                }
            }
            // for rate limiting
            tokio::time::sleep(REQUEST_DELAY).await;
        }
        Ok(())
    }

    /// YouTubeApiのurlを生成
    fn generate_url(
        &self,
        pending_ids: &mut Vec<crate::model::VideoId>,
    ) -> Option<String> {
        let batch_ids = self.next_video_id_batch(pending_ids);
        let batch_ids_str = if batch_ids.is_empty() {
            return None;
        } else {
            batch_ids
                .iter()
                .map(|id| id.as_str())
                .collect::<Vec<&str>>()
                .join(",")
        };

        Some(format!(
            "{}?part={}&maxResults={}&id={}&key={}",
            yt_api_const::ENDPOINT,
            yt_api_const::PARTS,
            yt_api_const::MAX_RESULTS,
            batch_ids_str,
            self.api_key.as_str()
        ))
    }

    /// 次の動画idのバッチを取得
    ///
    /// - pending_idsから動画idを取り出す
    /// - 動画idのバッチは最大で `yt_api::MAX_RESULTS`
    fn next_video_id_batch(
        &self,
        pending_ids: &mut Vec<crate::model::VideoId>,
    ) -> Vec<crate::model::VideoId> {
        let mut batch = Vec::new();
        let drain_range =
            0..(yt_api_const::MAX_RESULTS as usize).min(pending_ids.len());
        pending_ids.drain(drain_range).for_each(|id| batch.push(id));
        batch
    }

    async fn fetch_and_parse(
        &self,
        url: &str,
    ) -> Result<super::response::YouTubeApiResponse, crate::fetcher::YouTubeApiError>
    {
        use crate::fetcher::YouTubeApiError;

        let resp = Self::send_request(url).await?;

        let get_error_msg = async |response: reqwest::Response| {
            response
                .text()
                .await
                .unwrap_or_else(|_| "(No error message)".to_string())
        };

        match resp.status() {
            reqwest::StatusCode::OK => Self::parse_response(resp).await,
            reqwest::StatusCode::FORBIDDEN => {
                let error_message = get_error_msg(resp).await;
                tracing::warn!("YouTube API returned Forbidden: {}", error_message);
                Err(YouTubeApiError::Forbidden(error_message))
            }
            _ => {
                let status = resp.status();
                let error_message = get_error_msg(resp).await;
                tracing::warn!("YouTube API error: {} - {}", status, error_message);
                Err(YouTubeApiError::OtherApiError {
                    status,
                    message: error_message,
                })
            }
        }
    }

    /// YouTubeApiにリクエストを投げる
    async fn send_request(
        url: &str,
    ) -> Result<reqwest::Response, crate::fetcher::YouTubeApiError> {
        reqwest::Client::new()
            .get(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .send()
            .await
            .map_err(|e| crate::fetcher::YouTubeApiError::NetworkError(e.to_string()))
    }

    // YouTubeApiのレスポンスをパース
    async fn parse_response(
        resp: reqwest::Response,
    ) -> Result<super::response::YouTubeApiResponse, crate::fetcher::YouTubeApiError>
    {
        resp.json::<super::response::YouTubeApiResponse>()
            .await
            .map_err(|e| {
                crate::fetcher::YouTubeApiError::ResponseParseError(e.to_string())
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_url_batch() {
        let api_key = crate::fetcher::YouTubeApiKey::dummy_api_key();
        let api = YouTubeApi::new(api_key);
        let mut ids = vec![
            crate::model::VideoId::test_id_1(),
            crate::model::VideoId::test_id_2(),
            crate::model::VideoId::test_id_3(),
        ];
        let url = api.generate_url(&mut ids).unwrap();
        assert!(url.contains("11111111111"));
        assert!(url.contains("22222222222"));
        assert!(url.contains("33333333333"));
        assert!(url.contains("key=dummy_api_key"));
    }

    #[test]
    fn test_next_video_id_batch() {
        let api_key = crate::fetcher::YouTubeApiKey::dummy_api_key();
        let api = YouTubeApi::new(api_key);
        let mut ids = vec![
            crate::model::VideoId::test_id_1(),
            crate::model::VideoId::test_id_2(),
            crate::model::VideoId::test_id_3(),
        ];
        let batch = api.next_video_id_batch(&mut ids);
        assert_eq!(batch.len(), 3);
        assert_eq!(ids.len(), 0);
    }

    #[test]
    fn test_next_video_id_batch_empty() {
        let api_key = crate::fetcher::YouTubeApiKey::dummy_api_key();
        let api = YouTubeApi::new(api_key);
        let mut ids: Vec<crate::model::VideoId> = Vec::new();
        let batch = api.next_video_id_batch(&mut ids);
        assert!(batch.is_empty());
        assert!(ids.is_empty());
    }

    #[test]
    fn test_next_video_id_batch_more_than_max_results() {
        let api_key = crate::fetcher::YouTubeApiKey::dummy_api_key();
        let api = YouTubeApi::new(api_key);
        let mut ids = Vec::new();
        for _ in 0..60 {
            ids.push(crate::model::VideoId::test_id_1());
        }
        let batch = api.next_video_id_batch(&mut ids);
        assert_eq!(batch.len(), 50);
        assert_eq!(ids.len(), 10); // 60 - 50 = 10
    }
}
