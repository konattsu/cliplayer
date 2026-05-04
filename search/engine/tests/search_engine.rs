fn sample_engine() -> engine::SearchEngine {
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
            builder_version: "engine-test-builder".to_string(),
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
    engine::SearchEngine::load(std::sync::Arc::<[u8]>::from(bytes)).unwrap()
}

fn desc_sort() -> engine::api::query::input::SortSpec {
    engine::api::query::input::SortSpec {
        field: engine::api::query::types::SortField::PublishedAt,
        order: engine::api::query::types::SortOrder::Desc,
    }
}

#[test]
fn test_search_filters_artist_and_returns_exact_total() {
    let engine = sample_engine();
    let request = engine::api::query::input::SearchRequest {
        query: Some(engine::api::query::input::QueryNode::Term(
            engine::api::query::input::TermNode::ArtistAnyIn {
                values: vec!["artist-b".to_string()],
            },
        )),
        sort: vec![desc_sort()],
        page: engine::api::query::input::PageSpec {
            limit: 10,
            cursor: None,
        },
        total_mode: engine::api::query::types::TotalMode::Exact,
    };

    let response = engine.search(&request).unwrap();
    assert_eq!(
        response.clip_uuids,
        vec![
            "clip-d".to_string(),
            "clip-c".to_string(),
            "clip-b".to_string(),
        ],
    );
    assert_eq!(response.total, Some(3));
    assert!(!response.has_more);
    assert!(response.warnings.is_empty());
}

#[test]
fn test_search_pages_with_desc_tie_and_unknown_warning() {
    let engine = sample_engine();
    let request = engine::api::query::input::SearchRequest {
        query: Some(engine::api::query::input::QueryNode::Not {
            child: Box::new(engine::api::query::input::QueryNode::Term(
                engine::api::query::input::TermNode::ArtistAnyIn {
                    values: vec!["missing-artist".to_string()],
                },
            )),
        }),
        sort: vec![desc_sort()],
        page: engine::api::query::input::PageSpec {
            limit: 2,
            cursor: None,
        },
        total_mode: engine::api::query::types::TotalMode::None,
    };

    let first = engine.search(&request).unwrap();
    assert_eq!(
        first.clip_uuids,
        vec!["clip-e".to_string(), "clip-d".to_string()],
    );
    assert!(first.has_more);
    assert_eq!(
        first.warnings,
        vec![engine::api::response::QueryWarning::UnknownArtistId {
            value: "missing-artist".to_string(),
        }],
    );

    let second_request = engine::api::query::input::SearchRequest {
        page: engine::api::query::input::PageSpec {
            limit: 2,
            cursor: first.next_cursor.clone(),
        },
        ..request.clone()
    };
    let second = engine.search(&second_request).unwrap();
    assert_eq!(
        second.clip_uuids,
        vec!["clip-c".to_string(), "clip-b".to_string()],
    );
    assert!(second.has_more);

    let third_request = engine::api::query::input::SearchRequest {
        page: engine::api::query::input::PageSpec {
            limit: 2,
            cursor: second.next_cursor.clone(),
        },
        ..request
    };
    let third = engine.search(&third_request).unwrap();
    assert_eq!(third.clip_uuids, vec!["clip-a".to_string()]);
    assert!(!third.has_more);
}

#[test]
fn test_search_rejects_cursor_for_different_query() {
    let engine = sample_engine();
    let request = engine::api::query::input::SearchRequest {
        query: None,
        sort: vec![desc_sort()],
        page: engine::api::query::input::PageSpec {
            limit: 2,
            cursor: None,
        },
        total_mode: engine::api::query::types::TotalMode::None,
    };

    let first = engine.search(&request).unwrap();
    let mismatched_request = engine::api::query::input::SearchRequest {
        query: Some(engine::api::query::input::QueryNode::Term(
            engine::api::query::input::TermNode::ChannelAnyIn {
                values: vec!["channel-a".to_string()],
            },
        )),
        page: engine::api::query::input::PageSpec {
            limit: 2,
            cursor: first.next_cursor,
        },
        ..request
    };

    let err = engine.search(&mismatched_request).unwrap_err();
    assert_eq!(
        err,
        engine::EngineError::InvalidCursor("cursor query fingerprint does not match"),
    );
}
