const CURSOR_TOKEN_VERSION: u8 = 1;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct CursorToken {
    v: u8,
    index_build_id: String,
    query_fingerprint: String,
    sort_field: crate::api::SortField,
    sort_order: crate::api::SortOrder,
    last_published_at: i64,
    last_doc_id: u32,
}

pub(crate) fn encode(
    cursor: &engine::api::pagination::Cursor,
) -> Result<String, crate::error::SearchError> {
    use base64::Engine as _;

    let payload = CursorToken::from_engine(cursor);
    let bytes = serde_json::to_vec(&payload).map_err(|error| {
        crate::error::SearchError::internal(format!(
            "failed to encode cursor token payload: {error}",
        ))
    })?;

    Ok(base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes))
}

pub(crate) fn decode(
    token: &str,
) -> Result<engine::api::pagination::Cursor, crate::error::SearchError> {
    use base64::Engine as _;

    let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(token)
        .map_err(|_| {
            crate::error::SearchError::invalid_cursor(
                "cursor token is not valid base64url",
            )
        })?;

    let payload: CursorToken = serde_json::from_slice(&bytes).map_err(|_| {
        crate::error::SearchError::invalid_cursor(
            "cursor token payload is not valid JSON",
        )
    })?;

    payload.into_engine()
}

impl CursorToken {
    fn from_engine(cursor: &engine::api::pagination::Cursor) -> Self {
        Self {
            v: CURSOR_TOKEN_VERSION,
            index_build_id: cursor.index_build_id.to_string(),
            query_fingerprint: cursor.query_fingerprint.to_string(),
            sort_field: crate::api::SortField::from_engine(cursor.sort_field),
            sort_order: crate::api::SortOrder::from_engine(cursor.sort_order),
            last_published_at: i64::from(cursor.last_published_at),
            last_doc_id: cursor.last_doc_id,
        }
    }

    fn into_engine(
        self,
    ) -> Result<engine::api::pagination::Cursor, crate::error::SearchError> {
        if self.v != CURSOR_TOKEN_VERSION {
            return Err(crate::error::SearchError::invalid_cursor(
                "unsupported cursor token version",
            ));
        }

        Ok(engine::api::pagination::Cursor {
            index_build_id: parse_u64("index_build_id", &self.index_build_id)?,
            query_fingerprint: parse_u64("query_fingerprint", &self.query_fingerprint)?,
            sort_field: self.sort_field.into_engine(),
            sort_order: self.sort_order.into_engine(),
            last_published_at: self.last_published_at.into(),
            last_doc_id: self.last_doc_id,
        })
    }
}

fn parse_u64(field: &str, value: &str) -> Result<u64, crate::error::SearchError> {
    value.parse::<u64>().map_err(|_| {
        crate::error::SearchError::invalid_cursor(format!(
            "cursor token field `{field}` is invalid",
        ))
    })
}
