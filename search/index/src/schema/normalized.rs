/// build 中だけで使う正規化済みの中間表現。
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NormalizedClipRecord {
    pub(crate) doc_id: crate::schema::ids::DocId,
    pub(crate) clip_id: crate::schema::ids::ClipId,
    pub(crate) video_id: crate::schema::ids::VideoId,
    pub(crate) published_at: crate::schema::TimestampSecs,
    pub(crate) channel_id: crate::schema::ids::ChannelId,
    pub(crate) is_unlisted: bool,
    pub(crate) embeddable: bool,
    pub(crate) artist_ids: Vec<crate::schema::ids::ArtistId>,
    pub(crate) tag_ids: Vec<crate::schema::ids::TagId>,
}
