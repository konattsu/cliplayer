// 各enumに構造体生やしてそれぞれの構造体にimplしてみたが
// 処理内容が少ない, 逆に煩雑, どっちみちApiStateでunwrap必要だったのでやめた

// 状態遷移は純粋関数であるべきらしい

#[derive(Debug, Clone)]
pub(super) enum ApiState {
    Init,
    Fetching(StateFetching),
    Fetched {
        /// 取得した動画情報とdraftのペア
        draft_video_with_fetched: Vec<super::draft::DraftVideoWithFetched>,
    },
}

#[derive(Debug, thiserror::Error)]
pub(super) enum ApiStateError {
    #[error(
        "Invalid transition, expected from {expected_from} to {to}, but was from {from:?}"
    )]
    InvalidTransition {
        expected_from: &'static str,
        to: &'static str,
        from: ApiState,
    },
    #[error("pending_ids is not empty when transitioning to Fetched")]
    PendingIdsNotEmpty,
    #[error(
        "Called in an invalid state: expected {expected_from}, but state was {from:?}"
    )]
    CalledInInvalidState {
        expected_from: &'static str,
        from: ApiState,
    },
}

#[derive(Debug, Clone)]
pub(super) struct StateFetching {
    /// まだ取得していない動画IDのリスト
    pub(super) pending_ids: Vec<crate::model::VideoId>,
    /// 動画IDと取得した動画情報のペア
    pub(super) draft_video_with_fetched: std::collections::HashMap<
        crate::model::VideoId,
        Option<super::response::YouTubeApiItem>,
    >,
}

impl ApiState {
    /// 初期化状態から, fetchingの状態に遷移する
    ///
    /// `State`: `Init` -> `Fetching`
    ///
    /// - Error: `State` is **not** `Init`
    pub(super) fn transition_to_fetching(
        &self,
        drafts: Vec<crate::model::DraftVideo>,
    ) -> Result<Self, ApiStateError> {
        if !matches!(self, ApiState::Init) {
            return Err(ApiStateError::InvalidTransition {
                expected_from: "Init",
                to: "Fetching",
                from: self.clone(),
            });
        }

        let pending_ids = drafts.iter().map(|d| d.get_video_id()).cloned().collect();
        let draft_video_with_fetched = drafts
            .into_iter()
            .map(super::draft::DraftVideoWithFetched::init)
            .collect();
        Ok(Self::Fetching(StateFetching {
            pending_ids,
            draft_video_with_fetched,
        }))
    }

    /// fetchingの状態から、fetch済みの状態に遷移する
    ///
    /// `State`: `Fetching` -> `Fetched`
    ///
    /// Error:
    /// - `State` is **not** `Fetching`
    /// - `pending_ids` is **not** empty
    pub(super) fn transition_to_fetched(self) -> Result<Self, ApiStateError> {
        let fetching = match self {
            ApiState::Fetching(state) => state,
            _ => {
                return Err(ApiStateError::InvalidTransition {
                    expected_from: "Fetching",
                    to: "Fetched",
                    from: self,
                });
            }
        };

        if fetching.pending_ids.is_empty() {
            return Err(ApiStateError::PendingIdsNotEmpty);
        };

        Ok(Self::Fetched {
            draft_video_with_fetched: fetching.draft_video_with_fetched,
        })
    }

    /// fetch済みの状態から、最終的な結果に遷移する
    ///
    /// `State`: `Fetched` -> `(なし ∵全ての処理終了)`
    ///
    /// - Error: `State` is **not** `Fetched`
    pub(super) fn into_finalized(
        self,
    ) -> Result<Vec<crate::fetcher::FetchResult>, ApiStateError> {
        let fetched = match self {
            ApiState::Fetched {
                draft_video_with_fetched,
            } => draft_video_with_fetched,
            _ => {
                return Err(ApiStateError::InvalidTransition {
                    expected_from: "Fetched",
                    to: "Finalized",
                    from: self,
                });
            }
        };

        Ok(fetched
            .into_iter()
            .map(super::draft::DraftVideoWithFetched::finalize_result)
            .collect())
    }

    /// `ApiState`が`Fetching`状態であれば, 可変参照を返す
    ///
    /// - Error: `State` is **not** `Fetching`
    pub(super) fn expect_fetching_mut(
        &mut self,
    ) -> Result<&mut StateFetching, ApiStateError> {
        match self {
            ApiState::Fetching(state) => Ok(state),
            _ => Err(ApiStateError::CalledInInvalidState {
                expected_from: "Fetching",
                from: self.clone(),
            }),
        }
    }
}
