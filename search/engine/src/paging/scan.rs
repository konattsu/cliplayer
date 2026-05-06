pub(crate) fn scan_page(
    published_at_sort: &index_core::binary::SortIndexView<'_>,
    doc_set: &crate::doc_set::DocSet,
    order: crate::api::query::types::SortOrder,
    cursor_position: Option<usize>,
    limit: u32,
) -> Result<Vec<index_core::schema::ids::DocId>, crate::EngineError> {
    use crate::api::query::types::SortOrder;

    match order {
        SortOrder::Asc => scan_asc(published_at_sort, doc_set, cursor_position, limit),
        SortOrder::Desc => {
            scan_desc(published_at_sort, doc_set, cursor_position, limit)
        }
    }
}

fn scan_asc(
    published_at_sort: &index_core::binary::SortIndexView<'_>,
    doc_set: &crate::doc_set::DocSet,
    cursor_position: Option<usize>,
    limit: u32,
) -> Result<Vec<index_core::schema::ids::DocId>, crate::EngineError> {
    let start = cursor_position.map_or(0, |position| position + 1);
    let mut doc_ids = Vec::with_capacity(limit as usize + 1);

    for ordinal in start..published_at_sort.doc_ids_asc().len() {
        let doc_id = published_at_sort.doc_ids_asc().get(ordinal).ok_or(
            crate::EngineError::InternalIndex("sort ordinal out of bounds"),
        )?;
        if doc_set.contains(doc_id) {
            doc_ids.push(doc_id);
            if doc_ids.len() > limit as usize {
                break;
            }
        }
    }

    Ok(doc_ids)
}

fn scan_desc(
    published_at_sort: &index_core::binary::SortIndexView<'_>,
    doc_set: &crate::doc_set::DocSet,
    cursor_position: Option<usize>,
    limit: u32,
) -> Result<Vec<index_core::schema::ids::DocId>, crate::EngineError> {
    let end = cursor_position.unwrap_or(published_at_sort.doc_ids_asc().len());
    let mut doc_ids = Vec::with_capacity(limit as usize + 1);

    for ordinal in (0..end).rev() {
        let doc_id = published_at_sort.doc_ids_asc().get(ordinal).ok_or(
            crate::EngineError::InternalIndex("sort ordinal out of bounds"),
        )?;
        if doc_set.contains(doc_id) {
            doc_ids.push(doc_id);
            if doc_ids.len() > limit as usize {
                break;
            }
        }
    }

    Ok(doc_ids)
}
