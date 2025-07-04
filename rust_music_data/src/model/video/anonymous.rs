/// 構造と型だけ適している動画情報
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct AnonymousVideo {
    /// 動画id
    video_id: crate::model::VideoId,
    /// 動画のタグ
    tags: crate::model::TagList,
    /// クリップ
    clips: Vec<crate::model::AnonymousClip>,
}
