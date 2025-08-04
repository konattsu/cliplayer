/// 動画の概要情報
#[derive(serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct LocalVideoInfo {
    /// 動画ID
    pub(super) video_id: crate::model::VideoId,
    /// チャンネル名, 箱外で行われた配信/動画の時に付与
    pub(super) uploader_name: Option<crate::model::UploaderName>,
    /// 動画のタグ
    #[serde(default)]
    pub(super) video_tags: crate::model::VideoTags,
}

// TODO 本当に消すか確認
// /// `LocalVideoInfo`のリスト
// ///
// /// 各`LocalVideoInfo`に含まれる動画idが一意であることを保証
// #[derive(Debug, Clone)]
// pub(crate) struct LocalVideoInfoList {
//     pub(crate) inner: std::collections::HashMap<crate::model::VideoId, LocalVideoInfo>,
// }

impl LocalVideoInfo {
    pub(crate) fn get_video_id(&self) -> &crate::model::VideoId {
        &self.video_id
    }

    pub(crate) fn new(
        video_id: crate::model::VideoId,
        uploader_name: Option<crate::model::UploaderName>,
        tags: crate::model::VideoTags,
    ) -> Self {
        Self {
            video_id,
            uploader_name,
            video_tags: tags,
        }
    }
}

// TODO 本当に消すか確認
// impl LocalVideoInfoList {
// /// `LocalVideoInfo`のリストを`LocalVideoInfoList`に変換
// ///
// /// Err: 動画idが重複しているとき
// pub(crate) fn try_from_vec(
//     briefs: Vec<LocalVideoInfo>,
// ) -> Result<Self, Vec<crate::model::VideoId>> {
//     use std::collections::{HashMap, HashSet};

//     let mut inner = HashMap::with_capacity(briefs.len());
//     let mut duplicated_ids = HashSet::new();

//     for brief in briefs {
//         if let Some(prev_brief) = inner.insert(brief.get_video_id().clone(), brief)
//         {
//             // 重複の有無のみ検出したく, すでに重複しているか(3回,同じ動画IDが来たとき)どうかは
//             // 気にしないのでinsertの結果は無視
//             let _res = duplicated_ids.insert(prev_brief.get_video_id().clone());
//         }
//     }

//     if duplicated_ids.is_empty() {
//         Ok(Self { inner })
//     } else {
//         Err(duplicated_ids.into_iter().collect())
//     }
// }
// }

// MARK: For Tests

#[cfg(test)]
impl LocalVideoInfo {
    pub(crate) fn self_a() -> Self {
        LocalVideoInfo::new(
            crate::model::VideoId::test_id_1(),
            Some(crate::model::UploaderName::test_uploader_name_1()),
            crate::model::VideoTags::self_1(),
        )
    }
    pub(crate) fn self_b() -> Self {
        LocalVideoInfo::new(
            crate::model::VideoId::test_id_2(),
            None,
            crate::model::VideoTags::self_2(),
        )
    }
}

// MARK: Tests

#[cfg(test)]
mod tests {
    use super::*;

    const LOCAL_VIDEO_INFO_VALID_JSON_1: &str = r#"{
        "videoId": "11111111111",
        "uploaderName": "Test Channel 1",
        "videoTags": ["Test Video Tag1"]
    }"#;

    const LOCAL_VIDEO_INFO_VALID_JSON_2: &str = r#"{
        "videoId": "22222222222"
    }"#;

    #[test]
    fn test_local_video_info_deserialize_valid() {
        let local: LocalVideoInfo = serde_json::from_str(LOCAL_VIDEO_INFO_VALID_JSON_1)
            .expect("should deserialize successfully");
        assert_eq!(local.video_id, crate::model::VideoId::test_id_1());
        assert_eq!(
            local.uploader_name,
            Some(crate::model::UploaderName::test_uploader_name_1())
        );
        assert_eq!(local.video_tags, crate::model::VideoTags::self_1());

        let local: LocalVideoInfo = serde_json::from_str(LOCAL_VIDEO_INFO_VALID_JSON_2)
            .expect("should deserialize successfully");
        assert_eq!(local.video_id, crate::model::VideoId::test_id_2());
        assert!(local.uploader_name.is_none());
        assert!(local.video_tags.is_empty());
    }

    // unknown fieldsがある
    const LOCAL_VIDEO_INFO_INVALID_JSON_1: &str = r#"{
        "videoId": "33333333333",
        "videoTags": null,
        "editor": "vscode"
    }"#;

    // 必須フィールドの欠如
    const LOCAL_VIDEO_INFO_INVALID_JSON_2: &str = r#"{
        "uploaderName": "test_uploader",
        "videoTags": null
    }"#;

    #[test]
    fn test_local_video_info_deserialize_invalid() {
        let result: Result<LocalVideoInfo, _> =
            serde_json::from_str(LOCAL_VIDEO_INFO_INVALID_JSON_1);
        assert!(result.is_err());
        let result: Result<LocalVideoInfo, _> =
            serde_json::from_str(LOCAL_VIDEO_INFO_INVALID_JSON_2);
        assert!(result.is_err());
    }
}
