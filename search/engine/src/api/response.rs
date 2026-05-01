/// 公開 API の検索結果。
///
/// engine は内部の `doc_id` で評価し、返却直前に `clip_uuid` へ戻す。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchResponse {
    pub clip_uuids: Vec<index::schema::ids::ClipUuid>,
    pub next_cursor: Option<crate::api::pagination::Cursor>,
    pub total_mode: crate::api::query::types::TotalMode,
    pub total: Option<u32>,
    pub has_more: bool,
}

/// engine 内部で使う検索結果。
///
/// paging や total 計算の中間表現として `doc_id` の並びを保持する。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InternalSearchResponse {
    pub doc_ids: Vec<index::schema::ids::DocId>,
    pub next_cursor: Option<crate::api::pagination::Cursor>,
    pub total_mode: crate::api::query::types::TotalMode,
    pub total: Option<u32>,
    pub has_more: bool,
}
