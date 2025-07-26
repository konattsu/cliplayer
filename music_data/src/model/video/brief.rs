/// 動画の概要情報
#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct VideoBrief {
    /// 動画ID
    video_id: crate::model::VideoId,
    /// チャンネル名, 箱外で行われた配信/動画の時に付与
    uploader_name: Option<crate::model::UploaderName>,
    /// 動画のタグ
    #[serde(default)]
    video_tags: crate::model::VideoTags,
}

/// `VideoBrief`のリスト
///
/// 各`VideoBrief`に含まれる動画idが一意であることを保証
#[derive(Debug, Clone)]
pub(crate) struct VideoBriefs {
    pub(crate) inner: std::collections::HashMap<crate::model::VideoId, VideoBrief>,
}

impl VideoBrief {
    pub(crate) fn get_video_id(&self) -> &crate::model::VideoId {
        &self.video_id
    }
    pub(crate) fn get_uploader_name(&self) -> Option<&crate::model::UploaderName> {
        self.uploader_name.as_ref()
    }
    pub(crate) fn get_tags(&self) -> &crate::model::VideoTags {
        &self.video_tags
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

impl VideoBriefs {
    /// `VideoBrief`のリストを`VideoBriefs`に変換
    ///
    /// Err: 動画idが重複しているとき
    pub(crate) fn try_from_vec(
        briefs: Vec<VideoBrief>,
    ) -> Result<Self, Vec<crate::model::VideoId>> {
        use std::collections::{HashMap, HashSet};

        let mut inner = HashMap::with_capacity(briefs.len());
        let mut duplicated_ids = HashSet::new();

        for brief in briefs {
            if let Some(prev_brief) = inner.insert(brief.get_video_id().clone(), brief)
            {
                // 重複の有無のみ検出したく, すでに重複しているか(3回,同じ動画IDが来たとき)どうかは
                // 気にしないのでinsertの結果は無視
                let _res = duplicated_ids.insert(prev_brief.get_video_id().clone());
            }
        }

        if duplicated_ids.is_empty() {
            Ok(Self { inner })
        } else {
            Err(duplicated_ids.into_iter().collect())
        }
    }
}

// MARK: For Tests

#[cfg(test)]
impl VideoBrief {
    pub(crate) fn self_a() -> Self {
        VideoBrief::new(
            crate::model::VideoId::test_id_1(),
            Some(crate::model::UploaderName::test_uploader_name_1()),
            crate::model::VideoTags::self_1(),
        )
    }
    pub(crate) fn self_b() -> Self {
        VideoBrief::new(
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

    fn make_video_brief(id: crate::model::VideoId) -> VideoBrief {
        VideoBrief {
            video_id: id,
            uploader_name: None,
            video_tags: crate::model::VideoTags::new(vec![]).expect("will be valid"),
        }
    }

    #[test]
    fn test_video_briefs_try_into_video_briefs_unique() {
        let briefs = vec![
            make_video_brief(crate::model::VideoId::test_id_1()),
            make_video_brief(crate::model::VideoId::test_id_2()),
            make_video_brief(crate::model::VideoId::test_id_3()),
        ];
        let result = VideoBriefs::try_from_vec(briefs);
        assert!(result.is_ok());
        let briefs = result.unwrap();
        let keys = briefs
            .inner
            .keys()
            .cloned()
            .collect::<std::collections::HashSet<_>>();
        assert_eq!(
            keys,
            [
                crate::model::VideoId::test_id_1(),
                crate::model::VideoId::test_id_2(),
                crate::model::VideoId::test_id_3()
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn test_video_briefs_try_into_video_briefs_duplicate() {
        let briefs = vec![
            make_video_brief(crate::model::VideoId::test_id_1()),
            make_video_brief(crate::model::VideoId::test_id_2()),
            make_video_brief(crate::model::VideoId::test_id_1()), // duplicate
            make_video_brief(crate::model::VideoId::test_id_3()),
            make_video_brief(crate::model::VideoId::test_id_3()), // duplicate
        ];
        let result = VideoBriefs::try_from_vec(briefs);
        assert!(result.is_err());
        let mut ids = result.err().unwrap();
        ids.sort();
        let mut expected: Vec<crate::model::VideoId> = vec![
            crate::model::VideoId::test_id_1(),
            crate::model::VideoId::test_id_3(),
        ];
        expected.sort();
        assert_eq!(ids, expected);
    }

    const VIDEO_BRIEF_VALID_JSON_1: &str = r#"{
        "videoId": "11111111111",
        "uploaderName": "Test Channel 1",
        "videoTags": ["Test Video Tag1"]
    }"#;

    const VIDEO_BRIEF_VALID_JSON_2: &str = r#"{
        "videoId": "22222222222"
    }"#;

    #[test]
    fn test_video_brief_deserialize_valid() {
        let brief: VideoBrief = serde_json::from_str(VIDEO_BRIEF_VALID_JSON_1)
            .expect("should deserialize successfully");
        assert_eq!(brief.video_id, crate::model::VideoId::test_id_1());
        assert_eq!(
            brief.uploader_name,
            Some(crate::model::UploaderName::test_uploader_name_1())
        );
        assert_eq!(brief.video_tags, crate::model::VideoTags::self_1());

        let brief: VideoBrief = serde_json::from_str(VIDEO_BRIEF_VALID_JSON_2)
            .expect("should deserialize successfully");
        assert_eq!(brief.video_id, crate::model::VideoId::test_id_2());
        assert!(brief.uploader_name.is_none());
        assert!(brief.video_tags.is_empty());
    }

    // unknown fieldsがある
    const VIDEO_BRIEF_INVALID_JSON_1: &str = r#"{
        "videoId": "33333333333",
        "videoTags": null,
        "editor": "vscode"
    }"#;

    // 必須フィールドの欠如
    const VIDEO_BRIEF_INVALID_JSON_2: &str = r#"{
        "uploaderName": "test_uploader",
        "videoTags": null
    }"#;

    #[test]
    fn test_video_brief_deserialize_invalid() {
        let result: Result<VideoBrief, _> =
            serde_json::from_str(VIDEO_BRIEF_INVALID_JSON_1);
        assert!(result.is_err());
        let result: Result<VideoBrief, _> =
            serde_json::from_str(VIDEO_BRIEF_INVALID_JSON_2);
        assert!(result.is_err());
    }
}
