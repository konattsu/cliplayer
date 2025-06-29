/// youtube apiを呼び出すときに使う定数たち
mod yt_api_const {
    pub const ENDPOINT: &str = "https://www.googleapis.com/youtube/v3/videos";
    pub const PARTS: &str = "snippet,contentDetails,status";
    pub const MAX_RESULTS: u8 = 50;
}

#[derive(Debug, Clone)]
/// YouTube APIから動画情報を取得した結果
pub enum FetchResult {
    /// Clipのfinalizeに失敗
    FinalizationError(crate::model::VideoFinalizationError),
    /// 動画が存在しなかった
    NotExistVideo(crate::model::VideoId),
    /// 動画情報の取得に成功
    Ok(crate::model::FinalizedVideo),
}

/// YouTube APIを利用するための構造体
#[derive(Debug)]
pub struct YouTubeApi {
    /// api key
    api_key: crate::model::YouTubeApiKey,
    /// fetch待ちの動画ID
    pending_ids: Vec<crate::model::VideoId>,
    /// クリップ付き動画情報とfetchした動画情報のペア
    drafts_with_fetch: Vec<DraftVideoWithFetch>,
}

impl YouTubeApi {
    /// YouTube APIを利用するための構造体を生成
    pub fn new(api_key: crate::model::YouTubeApiKey) -> Self {
        Self {
            api_key,
            pending_ids: Vec::new(),
            drafts_with_fetch: Vec::new(),
        }
    }

    /// 取得してきた動画情報を最終的な結果に変換
    ///
    /// - Error: まだ動画情報を取得していない動画IDが存在する場合
    /// - Ok: すべての動画情報を取得している場合
    fn finalize_results(self) -> anyhow::Result<Vec<FetchResult>> {
        if !self.pending_ids.is_empty() {
            tracing::error!(pending_ids = ?self.pending_ids, "There are still pending video IDs");
            return Err(anyhow::anyhow!(format!(
                "There are still pending video IDs: {:?}",
                self.pending_ids
            )));
        }

        let mut res: Vec<FetchResult> =
            Vec::with_capacity(self.drafts_with_fetch.len());

        self.drafts_with_fetch
            .into_iter()
            .for_each(|draft_with_fetch| {
                res.push(draft_with_fetch.finalize_result());
            });
        Ok(res)
    }

    /// 次の動画idのバッチを取得
    ///
    /// 動画idのバッチは最大で `yt_api::MAX_RESULTS`
    #[tracing::instrument(level = tracing::Level::TRACE, skip(self), ret)]
    async fn next_video_id_batch(&mut self) -> Vec<crate::model::VideoId> {
        let mut batch = Vec::new();

        let batch_size = yt_api_const::MAX_RESULTS;
        for _ in 0..batch_size {
            if self.pending_ids.is_empty() {
                break;
            }
            batch.push(self.pending_ids.remove(0));
        }
        batch
    }

    /// 動画情報を取得するためのurlを生成
    ///
    /// urlには`yt_api_const::MAX_RESULTS`個の動画idが含まれる
    ///
    /// - Some: まだ動画情報を取得してない動画が存在するとき
    /// - None: 取得する動画が存在しないとき
    #[tracing::instrument(level = tracing::Level::TRACE, skip(self))]
    async fn generate_url(&mut self) -> Option<String> {
        let batch_video_ids = self.next_video_id_batch().await;
        let batch_video_ids_str = if batch_video_ids.is_empty() {
            return None;
        } else {
            batch_video_ids
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
            batch_video_ids_str,
            self.api_key.as_str()
        ))
    }

    /// 取得してきた動画情報をdraftと紐づける
    ///
    /// draftで存在しない動画idを指定していれば, そのidには何も紐づけられない
    async fn map_fetched_to_draft(
        &mut self,
        response: super::youtube_api_response::YouTubeApiResponse,
    ) {
        for item in response.items {
            if let Some(draft) = self
                .drafts_with_fetch
                .iter_mut()
                .find(|df| df.draft.get_video_id() == &item.id)
            {
                draft.fetched = Some(item);
            }
        }
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
    ) -> Result<
        super::youtube_api_response::YouTubeApiResponse,
        crate::fetcher::YouTubeApiError,
    > {
        res.json::<super::youtube_api_response::YouTubeApiResponse>()
            .await
            .map_err(|e| {
                let e = e.to_string();
                tracing::error!("Failed to parse YouTube API response: {}", e);
                crate::fetcher::YouTubeApiError::ResponseParseError(e)
            })
    }

    /// 与えられたurlからapiを呼び出して動画情報を取得する
    #[tracing::instrument(level = tracing::Level::TRACE, skip(self, url), ret)]
    async fn fetch_and_parse(
        &mut self,
        url: &str,
    ) -> Result<(), crate::fetcher::YouTubeApiError> {
        use crate::fetcher::YouTubeApiError;

        let res = Self::send_request(url).await?;
        // - 存在しない動画idを指定しても200が返ってくる
        // - 複数指定の場合は, その存在しない動画idはitemsに含まれない

        // 最大でMAX_RESULTSまでの動画idしか含んでいないため,
        // next_page_tokenを使用して再度リクエストを送信する必要はない

        match res.status() {
            reqwest::StatusCode::OK => {
                let response = match Self::parse_response(res).await {
                    Ok(resp) => resp,
                    Err(e) => {
                        let e = e.to_string();
                        // ここでparseできないことは無いと思っているのでerror出す
                        tracing::error!("Failed to parse YouTube API response: {}", e);
                        return Err(YouTubeApiError::ResponseParseError(e));
                    }
                };
                self.map_fetched_to_draft(response).await;
                Ok(())
            }
            reqwest::StatusCode::FORBIDDEN => {
                let error_message = res
                    .text()
                    .await
                    .unwrap_or_else(|_| "(No error message)".to_string());
                tracing::warn!("YouTube API returned Forbidden: {}", error_message);
                Err(YouTubeApiError::Forbidden(error_message))
            }
            _ => {
                let status = res.status();
                let error_message = res
                    .text()
                    .await
                    .unwrap_or_else(|_| "(No error message)".to_string());
                tracing::warn!("YouTube API error: {} - {}", status, error_message);
                Err(YouTubeApiError::OtherApiError {
                    status,
                    message: error_message,
                })
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
            let url = match self.generate_url().await {
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
                    Ok(_) => {
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

    pub async fn run(
        mut self,
        drafts: Vec<crate::model::DraftVideo>,
    ) -> Result<Vec<FetchResult>, crate::fetcher::YouTubeApiError> {
        self.pending_ids = drafts.iter().map(|d| d.get_video_id().clone()).collect();
        self.drafts_with_fetch =
            drafts.into_iter().map(DraftVideoWithFetch::new).collect();

        match self.fetch_process().await {
            Ok(_) => {
                tracing::info!("Successfully fetched video details.");
            }
            Err(e) => {
                tracing::error!(error = ?e, "Failed to fetch video details.");
                return Err(e);
            }
        };

        // TODO ネットワークエラーとかで全ての動画に対してfetch出来ていないときがある
        // 全ての動画に対してfetchしているのでunwrapする
        Ok(self.finalize_results().unwrap())
    }
}

/// クリップ付き動画情報とfetchした動画情報のペア
#[derive(Debug, Clone)]
struct DraftVideoWithFetch {
    draft: crate::model::DraftVideo,
    fetched: Option<super::youtube_api_response::YouTubeApiItem>,
}

impl DraftVideoWithFetch {
    fn new(draft: crate::model::DraftVideo) -> Self {
        Self {
            draft,
            fetched: None,
        }
    }

    /// 最終的な結果にfinalizeする
    fn finalize_result(self) -> FetchResult {
        let fetched = match self.fetched {
            Some(f) => f,
            None => {
                tracing::error!(
                    "video_id = %self.draft.get_video_id(), video not found",
                );
                return FetchResult::NotExistVideo(self.draft.into_video_id());
            }
        };

        let video_details =
            fetched.into_video_details(Some(self.draft.get_tags().clone()));

        match crate::model::FinalizedVideo::finalize_from_unidentified_clips(
            video_details,
            self.draft.into_unidentified(),
        ) {
            Ok(f) => FetchResult::Ok(f),
            Err(e) => {
                tracing::error!("failed to finalize video: {}", e);
                FetchResult::FinalizationError(e)
            }
        }
    }
}
