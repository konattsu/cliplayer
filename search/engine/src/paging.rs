mod cursor;
mod scan;

#[allow(clippy::too_many_arguments)] // この関数はクレート内公開で, 呼び出しも少ないので警告は無視
pub(crate) fn paginate(
    reader: &index_core::binary::SearchIndexReader<'_>,
    record_count: u32,
    dataset_build_id: &str,
    query_fingerprint: u64,
    sort: &crate::api::query::input::SortSpec,
    cursor: Option<&crate::api::pagination::Cursor>,
    limit: u32,
    total_mode: crate::api::query::types::TotalMode,
    doc_set: &crate::doc_set::DocSet,
    warnings: Vec<crate::api::response::QueryWarning>,
) -> Result<crate::api::response::InternalSearchResponse, crate::EngineError> {
    use crate::api::query::types::TotalMode;

    let published_ats = reader.published_ats()?;
    let published_at_sort = reader.published_at_sort()?;
    let total = match total_mode {
        TotalMode::Exact => Some(doc_set.count(record_count)),
        TotalMode::None => None,
    };

    let cursor_position = match cursor {
        Some(cursor) => {
            cursor::validate_cursor(
                &published_ats,
                dataset_build_id,
                query_fingerprint,
                sort,
                cursor,
            )?;
            Some(cursor::find_cursor_position(
                &published_ats,
                &published_at_sort,
                cursor,
            )?)
        }
        None => None,
    };

    let doc_ids = scan::scan_page(
        &published_at_sort,
        doc_set,
        sort.order,
        cursor_position,
        limit,
    )?;

    let has_more = doc_ids.len() > limit as usize;
    let mut doc_ids = doc_ids;
    if has_more {
        doc_ids.pop();
    }

    let next_cursor = match (has_more, doc_ids.last().copied()) {
        (true, Some(doc_id)) => Some(cursor::build_cursor(
            &published_ats,
            dataset_build_id,
            query_fingerprint,
            sort,
            doc_id,
        )?),
        _ => None,
    };

    Ok(crate::api::response::InternalSearchResponse {
        doc_ids,
        next_cursor,
        total_mode,
        total,
        has_more,
        warnings,
    })
}
