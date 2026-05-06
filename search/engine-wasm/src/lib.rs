mod api;
mod cursor;
mod error;

#[derive(Debug, Clone)]
#[wasm_bindgen::prelude::wasm_bindgen]
pub struct WasmSearchEngine {
    inner: engine::SearchEngine,
}

#[wasm_bindgen::prelude::wasm_bindgen]
impl WasmSearchEngine {
    /// Loads a binary search index into a WASM-friendly search engine.
    #[wasm_bindgen::prelude::wasm_bindgen(constructor)]
    pub fn new(
        index_bytes: Vec<u8>,
    ) -> Result<WasmSearchEngine, wasm_bindgen::JsValue> {
        Self::load(index_bytes).map_err(error::SearchError::into_js_value)
    }

    /// Evaluates a structured search request and returns a structured response.
    pub fn search(
        &self,
        request: wasm_bindgen::JsValue,
    ) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
        let request: api::SearchRequest = serde_wasm_bindgen::from_value(request)
            .map_err(|error| {
                error::SearchError::invalid_request(format!(
                    "failed to decode request: {error}",
                ))
                .into_js_value()
            })?;
        let response = self
            .search_request(request)
            .map_err(error::SearchError::into_js_value)?;

        serde_wasm_bindgen::to_value(&response).map_err(|error| {
            error::SearchError::internal(format!("failed to encode response: {error}",))
                .into_js_value()
        })
    }
}

impl WasmSearchEngine {
    fn load(index_bytes: Vec<u8>) -> Result<Self, error::SearchError> {
        let inner =
            engine::SearchEngine::load(std::sync::Arc::<[u8]>::from(index_bytes))
                .map_err(error::SearchError::from_engine)?;
        Ok(Self { inner })
    }

    fn search_request(
        &self,
        request: api::SearchRequest,
    ) -> Result<api::SearchResponse, error::SearchError> {
        let request = request.into_engine()?;
        let response = self
            .inner
            .search(&request)
            .map_err(error::SearchError::from_engine)?;
        api::SearchResponse::from_engine(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_engine() -> WasmSearchEngine {
        let clips = index_core::util::BiMap::from_ordered_strings(vec![
            "clip-a".to_string(),
            "clip-b".to_string(),
            "clip-c".to_string(),
            "clip-d".to_string(),
            "clip-e".to_string(),
        ])
        .unwrap();
        let videos = index_core::util::BiMap::from_ordered_strings(vec![
            "video-a".to_string(),
            "video-b".to_string(),
            "video-c".to_string(),
            "video-d".to_string(),
            "video-e".to_string(),
        ])
        .unwrap();
        let channels = index_core::util::BiMap::from_ordered_strings(vec![
            "channel-a".to_string(),
            "channel-b".to_string(),
        ])
        .unwrap();
        let artists = index_core::util::BiMap::from_ordered_strings(vec![
            "artist-a".to_string(),
            "artist-b".to_string(),
        ])
        .unwrap();
        let tags = index_core::util::BiMap::from_ordered_strings(vec![
            "tag-a".to_string(),
            "tag-b".to_string(),
        ])
        .unwrap();

        let index = index_core::schema::SearchIndex {
            meta: index_core::schema::IndexMetadata {
                index_format_version: 1,
                index_build_id: 99,
                builder_version: "engine-wasm-test-builder".to_string(),
                record_count: 5,
            },
            dictionaries: index_core::schema::Dictionaries {
                clips,
                videos,
                channels,
                artists,
                tags,
            },
            columns: index_core::schema::ColumnStore {
                clip_ids: vec![0, 1, 2, 3, 4],
                video_ids: vec![0, 1, 2, 3, 4],
                published_ats: vec![
                    10u32.into(),
                    20u32.into(),
                    20u32.into(),
                    30u32.into(),
                    40u32.into(),
                ],
                channel_ids: vec![0, 0, 1, 1, 0],
                is_unlisteds: vec![false, false, false, true, false],
                embeddables: vec![true, true, true, false, true],
                artist_id_lists: index_core::util::U32ListColumn::build(&[
                    vec![0],
                    vec![1],
                    vec![1],
                    vec![0, 1],
                    vec![],
                ]),
                tag_id_lists: index_core::util::U32ListColumn::build(&[
                    vec![0],
                    vec![0],
                    vec![1],
                    vec![],
                    vec![1],
                ]),
            },
            exact_indexes: index_core::schema::ExactIndexes {
                artist_docs: std::collections::HashMap::from([
                    (0, vec![0, 3]),
                    (1, vec![1, 2, 3]),
                ]),
                tag_docs: std::collections::HashMap::from([
                    (0, vec![0, 1]),
                    (1, vec![2, 4]),
                ]),
                channel_docs: std::collections::HashMap::from([
                    (0, vec![0, 1, 4]),
                    (1, vec![2, 3]),
                ]),
                is_unlisted_docs: [vec![0, 1, 2, 4], vec![3]],
                embeddable_docs: [vec![3], vec![0, 1, 2, 4]],
            },
            sort_indexes: index_core::schema::SortIndexes {
                published_at: index_core::schema::SortIndex::new(vec![0, 1, 2, 3, 4]),
            },
        };

        let bytes = index_core::binary::serialize_search_index(&index).unwrap();
        WasmSearchEngine::load(bytes).unwrap()
    }

    fn sample_request() -> api::SearchRequest {
        serde_json::from_value(serde_json::json!({
            "api_version": 1,
            "query": {
                "type": "not",
                "child": {
                    "type": "term",
                    "term": {
                        "type": "artist_any_in",
                        "values": ["missing-artist"],
                    },
                },
            },
            "sort": {
                "field": "published_at",
                "order": "desc",
            },
            "page": {
                "limit": 2,
                "cursor": null,
            },
            "total_mode": "none",
        }))
        .unwrap()
    }

    #[test]
    fn test_request_shape_rejects_unknown_fields() {
        let request = serde_json::from_value::<api::SearchRequest>(serde_json::json!({
            "query": null,
            "sort": {
                "field": "published_at",
                "order": "desc",
                "unexpected": true,
            },
            "page": {
                "limit": 2,
                "cursor": null,
            },
            "total_mode": "none",
        }));

        assert!(request.is_err());
    }

    #[test]
    fn test_search_pages_with_opaque_cursor() {
        let engine = sample_engine();
        let request = sample_request();

        let first = engine.search_request(request.clone()).unwrap();
        assert_eq!(
            first.clip_uuids,
            vec!["clip-e".to_string(), "clip-d".to_string()]
        );
        assert!(first.has_more);
        assert_eq!(
            first.warnings,
            vec![api::QueryWarning::UnknownArtistId {
                value: "missing-artist".to_string(),
            }],
        );

        let cursor = first.next_cursor.clone().unwrap();
        assert!(
            cursor
                .chars()
                .all(|character| character.is_ascii_alphanumeric()
                    || character == '-'
                    || character == '_')
        );

        let second = engine
            .search_request(api::SearchRequest {
                page: api::PageSpec {
                    limit: 2,
                    cursor: Some(cursor),
                },
                ..request.clone()
            })
            .unwrap();
        assert_eq!(
            second.clip_uuids,
            vec!["clip-c".to_string(), "clip-b".to_string()]
        );
        assert!(second.has_more);

        let third = engine
            .search_request(api::SearchRequest {
                page: api::PageSpec {
                    limit: 2,
                    cursor: second.next_cursor.clone(),
                },
                ..request
            })
            .unwrap();
        assert_eq!(third.clip_uuids, vec!["clip-a".to_string()]);
        assert!(!third.has_more);
        assert!(third.next_cursor.is_none());
    }

    #[test]
    fn test_invalid_cursor_returns_structured_error() {
        let engine = sample_engine();
        let error = engine
            .search_request(api::SearchRequest {
                page: api::PageSpec {
                    limit: 2,
                    cursor: Some("not-a-valid-token".to_string()),
                },
                ..sample_request()
            })
            .unwrap_err();

        assert_eq!(error.code, error::SearchErrorCode::InvalidCursor);
    }

    #[test]
    fn test_response_shape_serializes_warning_and_cursor() {
        let engine = sample_engine();
        let response = engine.search_request(sample_request()).unwrap();
        let json = serde_json::to_value(&response).unwrap();

        assert_eq!(json["clip_uuids"], serde_json::json!(["clip-e", "clip-d"]));
        assert_eq!(json["has_more"], serde_json::json!(true));
        assert_eq!(
            json["warnings"][0]["type"],
            serde_json::json!("unknown_artist_id")
        );
        assert!(json["next_cursor"].is_string());
    }

    #[test]
    fn test_api_version_mismatch_is_invalid_request() {
        let engine = sample_engine();
        let error = engine
            .search_request(api::SearchRequest {
                api_version: Some(2),
                ..sample_request()
            })
            .unwrap_err();

        assert_eq!(error.code, error::SearchErrorCode::InvalidRequest);
    }

    #[test]
    fn test_constructor_rejects_corrupt_index() {
        let error = WasmSearchEngine::load(vec![1, 2, 3]).unwrap_err();
        assert_eq!(error.code, error::SearchErrorCode::CorruptIndex);
    }
}
