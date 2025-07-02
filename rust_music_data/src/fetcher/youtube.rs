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
    requests: Vec<crate::fetcher::VideoDetailFetchRequest>,
    pending_ids: Vec<crate::model::VideoId>,
    detail_result: crate::fetcher::VideoDetailFetchResult,
}

// TODO tagsは受け取らない, VideoDetailWithoutTags的な構造体用意(fetcher/)してこれを返却
// 返却時は今と一緒のHashMapでいい
// ~WithoutTagsにinto_video_detail(tags)で目的の値を作成できるように

impl YouTubeApi {
    pub fn new(
        api_key: crate::fetcher::YouTubeApiKey,
        requests: Vec<crate::fetcher::VideoDetailFetchRequest>,
    ) -> Self {
        let pending_ids = requests
            .iter()
            .map(|req| req.get_video_id())
            .cloned()
            .collect();
        let detail_result: crate::fetcher::VideoDetailFetchResult = requests
            .iter()
            .map(|req| (req.get_video_id().clone(), None))
            .collect();
        Self {
            api_key,
            requests,
            pending_ids,
            detail_result,
        }
    }

    pub async fn run(
        mut self,
    ) -> Result<crate::fetcher::VideoDetailFetchResult, crate::fetcher::YouTubeApiError>
    {
        match self.fetch_process().await {
            Ok(_a) => {
                tracing::info!("YouTube API fetch completed successfully");
                Ok(self.detail_result)
            }
            Err(e) => {
                tracing::error!(error = ?e, "YouTube API fetch failed");
                return Err(e);
            }
        }
    }

    async fn fetch_process(&mut self) -> Result<(), crate::fetcher::YouTubeApiError> {
        const MAX_RETRY: u8 = 3;
        const REQUEST_DELAY: tokio::time::Duration =
            tokio::time::Duration::from_millis(125);
        const REQUEST_DELAY_RETRY: tokio::time::Duration =
            tokio::time::Duration::from_millis(500);

        // urlを作れなくなるまでループ
        loop {
            let url = match self.generate_url() {
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
                    Ok(_a) => {
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

    fn generate_url(&mut self) -> Option<String> {
        let batch_ids = self.next_video_id_batch();
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
    /// 動画idのバッチは最大で `yt_api::MAX_RESULTS`
    fn next_video_id_batch(&mut self) -> Vec<crate::model::VideoId> {
        let mut batch = Vec::new();
        self.pending_ids
            .drain(0..(yt_api_const::MAX_RESULTS as usize).min(self.pending_ids.len()))
            .for_each(|id| batch.push(id));
        batch
    }

    async fn fetch_and_parse(
        &mut self,
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

    fn map_response_to_detail_result(
        &mut self,
        response: super::response::YouTubeApiResponse,
    ) -> Result<(), crate::fetcher::YouTubeApiError> {
        for item in response.items {
            let video_id = item.id;
            if let Some(slot) = self.detail_result.0.get_mut(&video_id) {
                // let video_detail = item.into_video_details()

                *slot = Some(video_detail);
            }
        }
        Ok(())
    }
}
