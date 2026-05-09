fn sample_index() -> index_core::schema::SearchIndex {
    let clips = index_core::util::BiMap::from_ordered_strings(vec![
        "clip-a".to_string(),
        "clip-b".to_string(),
        "clip-c".to_string(),
        "clip-d".to_string(),
    ])
    .unwrap();
    let videos = index_core::util::BiMap::from_ordered_strings(vec![
        "video-a".to_string(),
        "video-b".to_string(),
        "video-c".to_string(),
        "video-d".to_string(),
    ])
    .unwrap();
    let channels = index_core::util::BiMap::from_ordered_strings(vec![
        "channel-a".to_string(),
        "channel-b".to_string(),
        "channel-c".to_string(),
    ])
    .unwrap();
    let artists = index_core::util::BiMap::from_ordered_strings(vec![
        "artist-a".to_string(),
        "artist-b".to_string(),
        "artist-c".to_string(),
    ])
    .unwrap();
    let tags = index_core::util::BiMap::from_ordered_strings(vec![
        "tag-a".to_string(),
        "tag-b".to_string(),
        "tag-c".to_string(),
        "tag-d".to_string(),
    ])
    .unwrap();

    index_core::schema::SearchIndex {
        meta: index_core::schema::IndexMetadata {
            index_format_version: 1,
            dataset_build_id:
                "dataset-build-07abcdef0123456789abcdef0123456789abcdef0123456789"
                    .to_string(),
            builder_version: "integration-test-builder".to_string(),
            record_count: 4,
        },
        dictionaries: index_core::schema::Dictionaries {
            clips,
            videos,
            channels,
            artists,
            tags,
        },
        columns: index_core::schema::ColumnStore {
            clip_ids: vec![0, 1, 2, 3],
            video_ids: vec![3, 2, 1, 0],
            published_ats: vec![5u32.into(), 5u32.into(), 30u32.into(), 10u32.into()],
            channel_ids: vec![0, 2, 0, 1],
            is_unlisteds: vec![false, false, true, true],
            embeddables: vec![true, true, false, true],
            artist_id_lists: index_core::util::U32ListColumn::build(&[
                vec![0, 2],
                vec![],
                vec![1],
                vec![1, 2],
            ]),
            tag_id_lists: index_core::util::U32ListColumn::build(&[
                vec![],
                vec![0, 3],
                vec![1],
                vec![2, 3],
            ]),
        },
        exact_indexes: index_core::schema::ExactIndexes {
            artist_docs: std::collections::HashMap::from([
                (0, vec![0]),
                (1, vec![2, 3]),
                (2, vec![0, 3]),
            ]),
            tag_docs: std::collections::HashMap::from([
                (0, vec![1]),
                (1, vec![2]),
                (2, vec![3]),
                (3, vec![1, 3]),
            ]),
            channel_docs: std::collections::HashMap::from([
                (0, vec![0, 2]),
                (1, vec![3]),
                (2, vec![1]),
            ]),
            is_unlisted_docs: [vec![0, 1], vec![2, 3]],
            embeddable_docs: [vec![2], vec![0, 1, 3]],
        },
        sort_indexes: index_core::schema::SortIndexes {
            published_at: index_core::schema::SortIndex::new(vec![0, 1, 3, 2]),
        },
    }
}

#[test]
fn test_public_binary_roundtrip_with_sparse_and_tied_values() {
    let index = sample_index();

    let bytes = index_core::binary::serialize_search_index(&index).unwrap();
    let reader = index_core::binary::SearchIndexReader::new(&bytes).unwrap();

    assert_eq!(
        reader.metadata_view().unwrap().builder_version(),
        "integration-test-builder",
    );
    assert_eq!(
        reader.metadata_view().unwrap().dataset_build_id(),
        "dataset-build-07abcdef0123456789abcdef0123456789abcdef0123456789",
    );
    assert_eq!(reader.clip_ids().unwrap().to_vec(), vec![0, 1, 2, 3]);
    assert_eq!(
        reader.tag_docs().unwrap().get(3).unwrap().unwrap().to_vec(),
        vec![1, 3],
    );
    assert_eq!(
        reader.published_at_sort().unwrap().doc_ids_asc().to_vec(),
        vec![0, 1, 3, 2],
    );
}

#[test]
fn test_public_reader_exposes_header_and_metadata_before_full_decode() {
    let index = sample_index();
    let bytes = index_core::binary::serialize_search_index(&index).unwrap();

    let reader = index_core::binary::SearchIndexReader::new(&bytes).unwrap();

    assert_eq!(reader.header().format_version, 1);
    assert_eq!(reader.header().record_count, 4);
    assert_eq!(
        reader.metadata_view().unwrap().builder_version(),
        "integration-test-builder",
    );
    assert_eq!(
        reader.metadata_view().unwrap().dataset_build_id(),
        "dataset-build-07abcdef0123456789abcdef0123456789abcdef0123456789",
    );
    assert_eq!(
        reader.clips_dictionary().unwrap().get(2).unwrap(),
        Some("clip-c"),
    );
    assert_eq!(reader.clip_ids().unwrap().to_vec(), vec![0, 1, 2, 3]);
    assert_eq!(
        reader
            .artist_docs()
            .unwrap()
            .get(1)
            .unwrap()
            .unwrap()
            .to_vec(),
        vec![2, 3],
    );
    assert_eq!(
        reader.published_at_sort().unwrap().doc_ids_asc().to_vec(),
        vec![0, 1, 3, 2],
    );
}

#[test]
fn test_public_deserialize_rejects_corrupted_header_version() {
    let index = sample_index();
    let mut bytes = index_core::binary::serialize_search_index(&index).unwrap();
    bytes[8..12].copy_from_slice(&2u32.to_le_bytes());

    let err = index_core::binary::SearchIndexReader::new(&bytes).unwrap_err();
    assert_eq!(err, index_core::binary::Error::UnsupportedVersion(2));
}
