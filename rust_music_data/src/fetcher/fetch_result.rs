#[derive(Debug, Clone)]
/// YouTube APIから動画情報を取得した結果
///
/// レスポンスが正常に取得できた場合(status code: 200)を前提にしている
pub enum FetchResult {
    /// Clipのfinalizeに失敗
    FinalizationError(crate::model::VideoFinalizationError),
    /// 動画が存在しなかった
    NotExistVideo(crate::model::VideoId),
    /// 動画情報の取得に成功
    Ok(crate::model::FinalizedVideo),
}
