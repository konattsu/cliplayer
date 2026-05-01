/// ドキュメント ID のリスト。
///
/// exact-match inverted index で使われる posting list で、
/// 検索結果として一致したドキュメントを順序付きで保持する。
pub type PostingList = Vec<crate::schema::ids::DocId>;

/// exact-match inverted indexes。
///
/// exact-match 用なので、日付のような範囲情報は持たない。
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ExactIndexes {
    pub artist_docs:
        std::collections::HashMap<crate::schema::ids::ArtistId, PostingList>,
    pub tag_docs: std::collections::HashMap<crate::schema::ids::TagId, PostingList>,
    pub channel_docs:
        std::collections::HashMap<crate::schema::ids::ChannelId, PostingList>,
    pub is_unlisted_docs: [PostingList; 2],
    pub embeddable_docs: [PostingList; 2],
}
