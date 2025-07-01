/// youtube apiを呼び出すときに使う定数たち
mod yt_api_const {
    pub const ENDPOINT: &str = "https://www.googleapis.com/youtube/v3/videos";
    pub const PARTS: &str = "snippet,contentDetails,status";
    pub const MAX_RESULTS: u8 = 50;
}

/// YouTube APIを利用するための構造体
#[derive(Debug)]
pub struct YouTubeApi {
    /// api key
    api_key: crate::model::YouTubeApiKey,
    /// 現在の状態
    state: super::state::ApiState,
}

impl YouTubeApi {
    /// YouTube APIを利用するための構造体を生成
    pub fn new(api_key: crate::model::YouTubeApiKey) -> Self {
        Self {
            api_key,
            state: super::state::ApiState::Init,
        }
    }

    /// 次の動画idのバッチを取得
    ///
    /// 動画idのバッチは最大で `yt_api_const::MAX_RESULTS`
    #[tracing::instrument(level = tracing::Level::TRACE, skip(self), ret)]
    fn drain_next_video_id_batch(
        &mut self,
    ) -> Result<Vec<crate::model::VideoId>, super::state::ApiStateError> {
        // `state`がFetching状態でないことはないのでunwrapで落とす
        // anyhow::Context使いたがったがselfの可変参照渡してしまっているので無理
        let state_fetching = self.state.expect_fetching_mut()?;

        let max_batch_size = yt_api_const::MAX_RESULTS as usize;
        Ok(state_fetching
            .pending_ids
            .drain(0..max_batch_size.min(state_fetching.pending_ids.len()))
            .collect())
    }

    /// 動画情報を取得するためのurlを生成
    ///
    /// urlには`yt_api_const::MAX_RESULTS`個の動画idが含まれる
    ///
    /// - Some: まだ動画情報を取得してない動画が存在するとき
    /// - None: 取得する動画が存在しないとき
    #[tracing::instrument(level = tracing::Level::TRACE, skip(self))]
    fn generate_url(&mut self) -> Result<Option<String>, super::state::ApiStateError> {
        let batch_video_ids = self.drain_next_video_id_batch()?;
        let batch_video_ids_str = if batch_video_ids.is_empty() {
            return Ok(None);
        } else {
            batch_video_ids
                .iter()
                .map(|id| id.as_str())
                .collect::<Vec<&str>>()
                .join(",")
        };

        Ok(Some(format!(
            "{}?part={}&maxResults={}&id={}&key={}",
            yt_api_const::ENDPOINT,
            yt_api_const::PARTS,
            yt_api_const::MAX_RESULTS,
            batch_video_ids_str,
            self.api_key.as_str()
        )))
    }

    /// 取得してきた動画情報をdraftと紐づける
    ///
    /// draftで存在しない動画idを指定していれば, そのidには何も紐づけられない
    async fn assign_fetched_to_drafts(
        &mut self,
        response: super::response::YouTubeApiResponse,
    ) -> Result<(), super::state::ApiStateError> {
        // ここのループは最大で`MAX_RESULTS`回
        for item in response.items {
            let draft = self
                .state
                .expect_fetching_mut()?
                .draft_video_with_fetched
                .iter_mut()
                // この探索時間はO(n)で, 一度に大量の動画をfetchすることはないので
                // hashmapを使った最適化などは考えない
                .find(|df| df.draft.get_video_id() == &item.id);
            match draft {
                Some(d) => d.fetched = Some(item),
                None => {
                    tracing::trace!("Draft not found for video ID: {}", item.id);
                }
            }
        }
        Ok(())
    }

    /// YouTube APIにリクエストを投げる
    #[tracing::instrument(level = tracing::Level::TRACE, skip(url))]
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

    /// YouTube APIのレスポンスをパースする
    async fn parse_response(
        res: reqwest::Response,
    ) -> Result<super::response::YouTubeApiResponse, crate::fetcher::YouTubeApiError>
    {
        res.json::<super::response::YouTubeApiResponse>()
            .await
            .map_err(|e| {
                crate::fetcher::YouTubeApiError::ResponseParseError(e.to_string())
            })
    }

    /// 与えられたurlからapiを呼び出して動画情報を取得する
    #[tracing::instrument(level = tracing::Level::TRACE, skip(self, url), ret)]
    async fn fetch_and_update_drafts(
        &mut self,
        url: &str,
    ) -> Result<Result<(), crate::fetcher::YouTubeApiError>, super::state::ApiStateError>
    {
        use crate::fetcher::YouTubeApiError;

        let res = match Self::send_request(url).await {
            Ok(r) => r,
            Err(e) => return Ok(Err(e)),
        };
        // - 存在しない動画idを指定しても200が返ってくる
        // - 複数指定の場合は, その存在しない動画idはitemsに含まれない

        // 最大でMAX_RESULTSまでの動画idしかurlに含んでいないため,
        // next_page_tokenを使用して再度リクエストを送信する必要はない

        match res.status() {
            reqwest::StatusCode::OK => {
                let response = match Self::parse_response(res).await {
                    Ok(resp) => resp,
                    Err(e) => {
                        tracing::warn!("Failed to parse YouTube API response");
                        return Ok(Err(e));
                    }
                };
                self.assign_fetched_to_drafts(response).await?;
                Ok(Ok(()))
            }
            reqwest::StatusCode::FORBIDDEN => {
                let error_message = res
                    .text()
                    .await
                    .unwrap_or_else(|_| "(No error message)".to_string());
                tracing::warn!("YouTube API returned Forbidden: {}", error_message);
                Ok(Err(YouTubeApiError::Forbidden(error_message)))
            }
            _ => {
                let status = res.status();
                let error_message = res
                    .text()
                    .await
                    .unwrap_or_else(|_| "(No error message)".to_string());
                tracing::warn!("YouTube API error: {} - {}", status, error_message);
                Ok(Err(YouTubeApiError::OtherApiError {
                    status,
                    message: error_message,
                }))
            }
        }
    }

    async fn fetch_process(
        &mut self,
    ) -> Result<Result<(), crate::fetcher::YouTubeApiError>, super::state::ApiStateError>
    {
        const MAX_RETRY: u8 = 3;
        // 8req/s まで抑える
        const REQUEST_DELAY: tokio::time::Duration =
            tokio::time::Duration::from_millis(125);
        const REQUEST_DELAY_RETRY: tokio::time::Duration =
            tokio::time::Duration::from_millis(500);

        // urlを作れなくなるまでループ
        loop {
            let url = match self.generate_url()? {
                Some(url) => url,
                None => {
                    tracing::info!("no more video IDs to fetch");
                    break;
                }
            };

            let mut retry_count = 0;
            // リトライ用のループ, 正常なときはループせず抜ける
            loop {
                match self.fetch_and_update_drafts(&url).await? {
                    Ok(..) => {
                        break;
                    }
                    Err(e) => {
                        retry_count += 1;
                        if retry_count >= MAX_RETRY {
                            tracing::error!(error = ?e, "YouTube API fetch error after retries");
                            return Ok(Err(e));
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
        Ok(Ok(()))
    }

    /// YouTube Apiを呼び出して動画情報を取得する
    ///
    /// # Arguments:
    /// - `drafts`動画情報を取得したい動画のリスト
    ///
    /// # Returns:
    /// - `Ok(Ok(...))`: api呼び出しが全て成功し, 期待している形式の動画情報を取得できたとき
    /// - `Ok(Err(...))`: なんらかの原因により, 途中でYouTube Api呼び出しをあきらめたとき
    /// - `Err(...)`: 内部的な状態遷移に失敗したとき
    pub async fn run(
        mut self,
        drafts: Vec<crate::model::DraftVideo>,
    ) -> anyhow::Result<
        Result<Vec<crate::fetcher::FetchResult>, crate::fetcher::YouTubeApiError>,
    > {
        use anyhow::Context;

        self.state = self
            .state
            .transition_to_fetching(drafts)
            .context("transition_to_fetching failed")?;

        match self.fetch_process().await.context("fetch_process failed")? {
            Ok(..) => {
                // api呼び出し, parse, draftへの紐づけが全て成功したとき
                tracing::info!("Successfully fetched video details.");
            }
            Err(e) => {
                // 途中でapi呼び出しをあきらめたとき
                tracing::error!(error = ?e, "Failed to fetch video details.");
                return Ok(Err(e));
            }
        };

        // `state`を`Fetched`に遷移させる
        self.state = self
            .state
            .transition_to_fetched()
            .context("transition_to_fetched failed")?;

        // `state`を`Finalized`に遷移させる
        Ok(Ok(self
            .state
            .into_finalized()
            .context(" transition_to_finalized failed")?))
    }
}
