#[derive(Debug, Clone)]
pub struct SearchEngine {
    index: crate::index::LoadedIndex,
}

impl SearchEngine {
    pub fn load(bytes: std::sync::Arc<[u8]>) -> Result<Self, crate::EngineError> {
        Ok(Self {
            index: crate::index::LoadedIndex::load(bytes)?,
        })
    }

    pub fn search(
        &self,
        request: &crate::api::query::input::SearchRequest,
    ) -> Result<crate::api::response::SearchResponse, crate::EngineError> {
        let request = crate::normalize::validate_and_normalize_request(request)?;
        let reader = self.index.reader()?;
        let (resolved_query, warnings) = crate::resolve::resolve_query(
            request.query.as_ref(),
            &self.index.dictionaries,
        )?;
        let doc_set = crate::eval::evaluate_query(
            &reader,
            self.index.record_count,
            &resolved_query,
        )?;
        let internal = crate::paging::paginate(
            &reader,
            self.index.record_count,
            self.index.index_build_id,
            request.query_fingerprint,
            &request.sort,
            request.cursor.as_ref(),
            request.limit,
            request.total_mode,
            &doc_set,
            warnings,
        )?;

        to_public_response(&reader, internal)
    }
}

fn to_public_response(
    reader: &index_core::binary::SearchIndexReader<'_>,
    internal: crate::api::response::InternalSearchResponse,
) -> Result<crate::api::response::SearchResponse, crate::EngineError> {
    let clip_ids = reader.clip_ids()?;
    let clips_dictionary = reader.clips_dictionary()?;

    let mut clip_uuids = Vec::with_capacity(internal.doc_ids.len());
    for doc_id in &internal.doc_ids {
        let clip_id =
            clip_ids
                .get(*doc_id as usize)
                .ok_or(crate::EngineError::InternalIndex(
                    "clip_ids column out of bounds",
                ))?;
        let clip_uuid =
            clips_dictionary
                .get(clip_id)?
                .ok_or(crate::EngineError::InternalIndex(
                    "clip id missing from dictionary",
                ))?;
        clip_uuids.push(clip_uuid.to_string());
    }

    Ok(crate::api::response::SearchResponse {
        clip_uuids,
        next_cursor: internal.next_cursor,
        total_mode: internal.total_mode,
        total: internal.total,
        has_more: internal.has_more,
        warnings: internal.warnings,
    })
}
