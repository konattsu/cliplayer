/// 検索エンジンが読み取る構造化インデックス全体。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchIndex {
    pub meta: crate::schema::IndexMetadata,
    pub dictionaries: crate::schema::Dictionaries,
    pub columns: crate::schema::ColumnStore,
    pub exact_indexes: crate::schema::ExactIndexes,
    pub sort_indexes: crate::schema::SortIndexes,
}
