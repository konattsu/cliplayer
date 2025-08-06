/// youtube apiを呼び出すときに使う定数たち
mod yt_api_const {
    pub(super) const ENDPOINT: &str = "https://www.googleapis.com/youtube/v3/videos";
    pub(super) const PARTS: &str = "snippet,contentDetails,status";
    pub(super) const MAX_RESULTS: u8 = 50;
}

/// youtube apiを呼び出すときに使うネットワーク関連の設定
mod network_cfg {
    pub(super) const MAX_RETRY: u8 = 3;
    pub(super) const REQUEST_DELAY: tokio::time::Duration =
        tokio::time::Duration::from_millis(125);
    pub(super) const REQUEST_DELAY_RETRY: tokio::time::Duration =
        tokio::time::Duration::from_millis(500);
}

#[derive(Debug)]
pub(crate) struct YouTubeApi {
    api_key: crate::fetcher::YouTubeApiKey,
}

impl YouTubeApi {
    pub(crate) fn new(api_key: crate::fetcher::YouTubeApiKey) -> Self {
        Self { api_key }
    }

    #[tracing::instrument(skip(self, video_ids), level = tracing::Level::DEBUG)]
    pub(crate) async fn run(
        &self,
        video_ids: crate::model::VideoIds,
    ) -> Result<crate::model::ApiVideoInfoList, crate::fetcher::YouTubeApiError> {
        tracing::trace!("YouTube API fetch started");
        match self.fetch_process(video_ids).await {
            Ok(api_info_list) => Ok(api_info_list),
            Err(e) => {
                tracing::error!(error = ?e, "YouTube API fetch failed");
                Err(e)
            }
        }
    }

    #[tracing::instrument(skip(self), level = tracing::Level::TRACE)]
    async fn fetch_process(
        &self,
        mut video_ids: crate::model::VideoIds,
    ) -> Result<crate::model::ApiVideoInfoList, crate::fetcher::YouTubeApiError> {
        let mut fetched_api_info: Vec<crate::model::ApiVideoInfo> =
            Vec::with_capacity(video_ids.len());

        // urlを作れなくなるまでループ
        loop {
            let url = match self.generate_url(&mut video_ids) {
                Some(url) => url,
                None => {
                    tracing::trace!("no more video IDs to fetch");
                    break;
                }
            };

            let mut retry_count = 0;
            // リトライ用のループ, 正常なときはループせず抜ける
            loop {
                match self.fetch_and_parse(&url).await {
                    Ok(resp) => {
                        fetched_api_info.extend(resp.into_api_video_info_vec());
                        break;
                    }
                    Err(e) => {
                        retry_count += 1;
                        if retry_count >= network_cfg::MAX_RETRY {
                            tracing::error!(error = ?e, "YouTube API fetch error after retries");
                            eprintln!(
                                "Failed to fetch YouTube API after {retry_count} retries: {e}"
                            );
                            return Err(e);
                        }
                        tracing::warn!(
                            error = ?e,
                            "YouTube API fetch error, retrying... (attempt {}/{})",
                            retry_count,
                            network_cfg::MAX_RETRY
                        );
                        eprintln!(
                            "YouTube API fetch error, retrying... (attempt {}/{})",
                            retry_count,
                            network_cfg::MAX_RETRY
                        );
                        tokio::time::sleep(network_cfg::REQUEST_DELAY_RETRY).await;
                    }
                }
            }
            // for rate limiting
            tokio::time::sleep(network_cfg::REQUEST_DELAY).await;
        }

        Ok(crate::model::ApiVideoInfoList::from_vec_ignore_duplicated(
            fetched_api_info,
        ))
    }

    /// YouTubeApiのurlを生成
    fn generate_url(&self, pending_ids: &mut crate::model::VideoIds) -> Option<String> {
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
        pending_ids: &mut crate::model::VideoIds,
    ) -> crate::model::VideoIds {
        let mut batch = Vec::new();
        let drain_range =
            0..(yt_api_const::MAX_RESULTS as usize).min(pending_ids.len());
        pending_ids.drain(drain_range).for_each(|id| batch.push(id));
        batch.into()
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
        // reqwest::headerはurl含むのでログ出さないように
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
        let mut ids: crate::model::VideoIds = vec![
            crate::model::VideoId::test_id_1(),
            crate::model::VideoId::test_id_2(),
            crate::model::VideoId::test_id_3(),
        ]
        .into();
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
        let mut ids: crate::model::VideoIds = vec![
            crate::model::VideoId::test_id_1(),
            crate::model::VideoId::test_id_2(),
            crate::model::VideoId::test_id_3(),
        ]
        .into();
        let batch = api.next_video_id_batch(&mut ids);
        assert_eq!(batch.len(), 3);
        assert_eq!(ids.len(), 0);
    }

    #[test]
    fn test_next_video_id_batch_empty() {
        let api_key = crate::fetcher::YouTubeApiKey::dummy_api_key();
        let api = YouTubeApi::new(api_key);
        let mut ids: crate::model::VideoIds = Vec::new().into();
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
        let mut ids: crate::model::VideoIds = ids.into();
        let batch = api.next_video_id_batch(&mut ids);
        assert_eq!(batch.len(), 50);
        assert_eq!(ids.len(), 10); // 60 - 50 = 10
    }
}
