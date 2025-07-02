/// VideoDetailを取得するためのリクエストの構造体
#[derive(Debug, Clone)]
pub struct VideoDetailFetchRequest {
    /// 動画ID
    video_id: crate::model::VideoId,
    /// 動画のタグ
    tags: Option<crate::model::TagList>,
}

/// VideoIdとそれに基づくVideoDetailのペアを格納する構造体
#[derive(Debug, Clone)]
pub struct VideoDetailFetchResult(
    pub  std::collections::HashMap<
        crate::model::VideoId,
        Option<crate::model::VideoDetail>,
    >,
);

impl VideoDetailFetchRequest {
    pub fn new(
        video_id: crate::model::VideoId,
        tags: Option<crate::model::TagList>,
    ) -> Self {
        Self { video_id, tags }
    }

    pub fn get_video_id(&self) -> &crate::model::VideoId {
        &self.video_id
    }

    pub fn get_tags(&self) -> Option<&crate::model::TagList> {
        self.tags.as_ref()
    }
}

impl FromIterator<(crate::model::VideoId, Option<crate::model::VideoDetail>)>
    for VideoDetailFetchResult
{
    fn from_iter<
        I: IntoIterator<Item = (crate::model::VideoId, Option<crate::model::VideoDetail>)>,
    >(
        iter: I,
    ) -> Self {
        let map = iter.into_iter().collect();
        Self(map)
    }
}
