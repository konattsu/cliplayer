/// `doc_id` から各フィールド値を引くための列ストア。
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ColumnStore {
    pub clip_ids: Vec<crate::schema::ids::ClipId>,
    pub video_ids: Vec<crate::schema::ids::VideoId>,
    pub published_ats: Vec<crate::schema::TimestampSecs>,
    pub channel_ids: Vec<crate::schema::ids::ChannelId>,
    pub is_unlisteds: Vec<bool>,
    pub embeddables: Vec<bool>,
    pub artist_id_lists: crate::util::U32ListColumn,
    pub tag_id_lists: crate::util::U32ListColumn,
}
