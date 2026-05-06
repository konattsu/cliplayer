/// 1 つの sort field に対する順序 index。
///
/// 候補集合そのものは持たず、その field の昇順で `doc_id` を
/// 走査するための安定順序だけを保持する。
/// 降順はこの配列を逆順走査して実現する。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SortIndex {
    doc_ids_asc: Vec<crate::schema::ids::DocId>,
}

impl SortIndex {
    pub fn new(doc_ids_asc: Vec<crate::schema::ids::DocId>) -> Self {
        Self { doc_ids_asc }
    }

    pub fn doc_ids_asc(&self) -> &[crate::schema::ids::DocId] {
        &self.doc_ids_asc
    }
}

/// sort field ごとの index 群。
///
/// index 側は query の sort 条件型を知らず、
/// 利用可能な sort index の実体だけを保持する。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SortIndexes {
    pub published_at: SortIndex,
}
