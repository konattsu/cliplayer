// OtherApiError, Forbiddenに分けた理由:
//   Forbidden: 今まで正常だったのに急にapi制限引っかかる可能性があり, 検出しやすくするため
//   OtherApiErrorに他のエラーを統合: 403以外のエラーはほぼほぼ受け取らない
//     ref: https://developers.google.com/youtube/v3/docs/errors#videos_youtube.videos.list

// ResponseParseError:
// このエラーになることは無いと思っているが, 原因特定しやすくするために分割

/// YouTube API呼び出し時のエラー
#[derive(Debug, thiserror::Error, Clone)]
pub enum YouTubeApiError {
    /// apiが不正/制限
    #[error("forbidden: {0}")]
    Forbidden(String),
    /// レスポンスが受け取れない
    #[error("network error: {0}")]
    NetworkError(String),
    /// レスポンスのパースに失敗
    #[error("response parse error: {0}")]
    ResponseParseError(String),
    /// 他のAPIエラー
    #[error("other api error: {status} {message}")]
    OtherApiError {
        status: reqwest::StatusCode,
        message: String,
    },
}

impl YouTubeApiError {
    /// エラーメッセージを整形して返す
    ///
    /// 文字列の最後に`\n`が付与される
    pub fn to_pretty_string(&self) -> String {
        format!("Failed to call YouTube Api: {self}\n")
    }
}
