/// ページング再開位置を表す cursor。
///
/// cursor は同一 index build, 同一 query, 同一 sort 条件に対する
/// 再開位置だけを表す。
///
/// 将来、sort の種類ごとに再開位置に必要な payload が変わるなら、
/// `Cursor` を enum にして sort ごとの variant に `doc_id` 以外の値を
/// 持たせる設計も有用である。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cursor {
    pub dataset_build_id: String,
    pub query_fingerprint: u64,
    pub sort_field: crate::api::query::types::SortField,
    pub sort_order: crate::api::query::types::SortOrder,
    pub last_published_at: index_core::schema::TimestampSecs,
    pub last_doc_id: index_core::schema::ids::DocId,
}
