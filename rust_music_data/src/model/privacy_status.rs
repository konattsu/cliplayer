/// 動画の公開状況
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum PrivacyStatus {
    /// 公開
    Public,
    /// 限定公開
    Unlisted,
    /// 非公開
    Private,
}
