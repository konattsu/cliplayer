/// 検索条件の評価過程で使う文書集合。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocSet {
    All,
    Empty,
    SortedDocIds(Vec<index_core::schema::ids::DocId>),
    BitSet(Vec<u64>),
}
