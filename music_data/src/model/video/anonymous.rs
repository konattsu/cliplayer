/// 構造と型だけ適している動画情報
#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct AnonymousVideo {
    /// 動画の情報
    #[serde(flatten)]
    video_brief: crate::model::VideoBrief,
    /// クリップ
    clips: Vec<crate::model::AnonymousClip>,
}
