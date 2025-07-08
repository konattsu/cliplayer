#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct VideoBrief {
    /// 動画ID
    video_id: crate::model::VideoId,
    /// チャンネル名, 箱外で行われた配信/動画の時に付与
    uploader_name: Option<crate::model::UploaderName>,
    /// 動画のタグ
    tags: crate::model::VideoTags,
}

impl VideoBrief {
    pub fn get_video_id(&self) -> &crate::model::VideoId {
        &self.video_id
    }
    pub fn get_uploader_name(&self) -> Option<&crate::model::UploaderName> {
        self.uploader_name.as_ref()
    }
    pub fn get_tags(&self) -> &crate::model::VideoTags {
        &self.tags
    }
}
