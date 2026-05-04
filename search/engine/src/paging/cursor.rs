pub(crate) fn validate_cursor(
    published_ats: &index_core::binary::I64SliceView<'_>,
    index_build_id: u64,
    query_fingerprint: u64,
    sort: &crate::api::query::input::SortSpec,
    cursor: &crate::api::pagination::Cursor,
) -> Result<(), crate::EngineError> {
    use crate::EngineError;

    if cursor.index_build_id != index_build_id {
        return Err(EngineError::InvalidCursor(
            "cursor index build id does not match",
        ));
    }
    if cursor.query_fingerprint != query_fingerprint {
        return Err(EngineError::InvalidCursor(
            "cursor query fingerprint does not match",
        ));
    }
    if cursor.sort_field != sort.field || cursor.sort_order != sort.order {
        return Err(EngineError::InvalidCursor(
            "cursor sort does not match request sort",
        ));
    }

    let actual = published_ats
        .get(cursor.last_doc_id as usize)
        .ok_or(EngineError::InvalidCursor("cursor doc id is out of bounds"))?;
    if index_core::schema::TimestampSecs::from(actual) != cursor.last_published_at {
        return Err(EngineError::InvalidCursor(
            "cursor seek key does not match index contents",
        ));
    }

    Ok(())
}

pub(crate) fn build_cursor(
    published_ats: &index_core::binary::I64SliceView<'_>,
    index_build_id: u64,
    query_fingerprint: u64,
    sort: &crate::api::query::input::SortSpec,
    doc_id: index_core::schema::ids::DocId,
) -> Result<crate::api::pagination::Cursor, crate::EngineError> {
    let published_at =
        published_ats
            .get(doc_id as usize)
            .ok_or(crate::EngineError::InternalIndex(
                "published_at column out of bounds",
            ))?;
    Ok(crate::api::pagination::Cursor {
        index_build_id,
        query_fingerprint,
        sort_field: sort.field,
        sort_order: sort.order,
        last_published_at: index_core::schema::TimestampSecs::from(published_at),
        last_doc_id: doc_id,
    })
}

pub(crate) fn find_cursor_position(
    published_ats: &index_core::binary::I64SliceView<'_>,
    published_at_sort: &index_core::binary::SortIndexView<'_>,
    cursor: &crate::api::pagination::Cursor,
) -> Result<usize, crate::EngineError> {
    published_at_sort
        .doc_ids_asc()
        .iter()
        .enumerate()
        .find_map(|(ordinal, doc_id)| {
            let published_at = published_ats.get(doc_id as usize)?;
            ((index_core::schema::TimestampSecs::from(published_at)
                == cursor.last_published_at)
                && doc_id == cursor.last_doc_id)
                .then_some(ordinal)
        })
        .ok_or(crate::EngineError::InvalidCursor(
            "cursor seek key not found in sort index",
        ))
}
