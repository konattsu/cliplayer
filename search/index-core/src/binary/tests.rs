#[derive(Debug, Clone, Copy)]
struct SectionDescriptor {
    table_index: usize,
    section_id: u32,
    physical_encoding: u32,
    item_count: u32,
    offset: u64,
    byte_len: u64,
}

pub(crate) fn sample_index() -> crate::schema::SearchIndex {
    let clips = crate::util::BiMap::from_ordered_strings(vec![
        "clip-a".to_string(),
        "clip-b".to_string(),
        "clip-c".to_string(),
    ])
    .unwrap();
    let videos = crate::util::BiMap::from_ordered_strings(vec![
        "video-a".to_string(),
        "video-b".to_string(),
        "video-c".to_string(),
    ])
    .unwrap();
    let channels = crate::util::BiMap::from_ordered_strings(vec![
        "channel-a".to_string(),
        "channel-b".to_string(),
    ])
    .unwrap();
    let artists = crate::util::BiMap::from_ordered_strings(vec![
        "artist-a".to_string(),
        "artist-b".to_string(),
        "artist-c".to_string(),
    ])
    .unwrap();
    let tags = crate::util::BiMap::from_ordered_strings(vec![
        "tag-a".to_string(),
        "tag-b".to_string(),
        "tag-c".to_string(),
    ])
    .unwrap();

    crate::schema::SearchIndex {
        meta: crate::schema::IndexMetadata {
            index_format_version: super::format::FORMAT_VERSION,
            builder_version: "test-builder".to_string(),
            record_count: 3,
        },
        dictionaries: crate::schema::Dictionaries {
            clips,
            videos,
            channels,
            artists,
            tags,
        },
        columns: crate::schema::ColumnStore {
            clip_ids: vec![0, 1, 2],
            video_ids: vec![2, 0, 1],
            published_ats: vec![20u32.into(), 10u32.into(), 20u32.into()],
            channel_ids: vec![1, 0, 1],
            is_unlisteds: vec![false, true, false],
            embeddables: vec![true, false, true],
            artist_id_lists: crate::util::U32ListColumn::build(&[
                vec![0, 2],
                vec![],
                vec![1, 2],
            ]),
            tag_id_lists: crate::util::U32ListColumn::build(&[
                vec![1],
                vec![0, 2],
                vec![],
            ]),
        },
        exact_indexes: crate::schema::ExactIndexes {
            artist_docs: std::collections::HashMap::from([
                (0, vec![0]),
                (1, vec![2]),
                (2, vec![0, 2]),
            ]),
            tag_docs: std::collections::HashMap::from([
                (0, vec![1]),
                (1, vec![0]),
                (2, vec![1]),
            ]),
            channel_docs: std::collections::HashMap::from([
                (0, vec![1]),
                (1, vec![0, 2]),
            ]),
            is_unlisted_docs: [vec![0, 2], vec![1]],
            embeddable_docs: [vec![1], vec![0, 2]],
        },
        sort_indexes: crate::schema::SortIndexes {
            published_at: crate::schema::SortIndex::new(vec![1, 0, 2]),
        },
    }
}

fn serialize_index(index: &crate::schema::SearchIndex) -> Vec<u8> {
    super::serialize_search_index(index).unwrap()
}

fn write_u32_at(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn write_u64_at(bytes: &mut [u8], offset: usize, value: u64) {
    bytes[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}

fn section_count(bytes: &[u8]) -> u32 {
    super::codec::read_u32_at(bytes, 12).unwrap()
}

fn section_entry_offset(table_index: usize) -> usize {
    super::format::FILE_HEADER_SIZE
        + table_index * super::format::SECTION_TABLE_ENTRY_SIZE
}

fn read_section(bytes: &[u8], section_id: u32) -> SectionDescriptor {
    let section_count = section_count(bytes) as usize;
    for table_index in 0..section_count {
        let entry_offset = section_entry_offset(table_index);
        let current_section_id =
            super::codec::read_u32_at(bytes, entry_offset).unwrap();
        if current_section_id == section_id {
            return SectionDescriptor {
                table_index,
                section_id: current_section_id,
                physical_encoding: super::codec::read_u32_at(bytes, entry_offset + 4)
                    .unwrap(),
                item_count: super::codec::read_u32_at(bytes, entry_offset + 8).unwrap(),
                offset: super::codec::read_u64_at(bytes, entry_offset + 16).unwrap(),
                byte_len: super::codec::read_u64_at(bytes, entry_offset + 24).unwrap(),
            };
        }
    }

    panic!("section not found: 0x{section_id:04x}");
}

fn section_payload_range(section: SectionDescriptor) -> std::ops::Range<usize> {
    let start = usize::try_from(section.offset).unwrap();
    let len = usize::try_from(section.byte_len).unwrap();
    start..start + len
}

#[test]
fn test_roundtrip_search_index_binary() {
    let index = sample_index();
    let bytes = serialize_index(&index);
    let reader = super::SearchIndexReader::new(&bytes).unwrap();

    assert_eq!(
        reader.metadata_view().unwrap().builder_version(),
        "test-builder"
    );
    assert_eq!(
        reader.clips_dictionary().unwrap().get(0).unwrap(),
        Some("clip-a")
    );
    assert_eq!(reader.video_ids().unwrap().to_vec(), vec![2, 0, 1]);
    assert_eq!(
        reader
            .artist_docs()
            .unwrap()
            .get(2)
            .unwrap()
            .unwrap()
            .to_vec(),
        vec![0, 2],
    );
    assert_eq!(
        reader.published_at_sort().unwrap().doc_ids_asc().to_vec(),
        vec![1, 0, 2],
    );
}

#[test]
fn test_writer_emits_expected_header_and_monotonic_aligned_sections() {
    let index = sample_index();
    let bytes = serialize_index(&index);

    assert_eq!(&bytes[..8], super::format::MAGIC);
    assert_eq!(
        super::codec::read_u32_at(&bytes, 8).unwrap(),
        super::format::FORMAT_VERSION
    );
    assert_eq!(
        section_count(&bytes),
        super::format::REQUIRED_SECTION_IDS.len() as u32,
    );
    assert_eq!(
        super::codec::read_u32_at(&bytes, 16).unwrap(),
        index.meta.record_count
    );
    assert_eq!(
        super::codec::read_u64_at(&bytes, 24).unwrap(),
        super::format::FILE_HEADER_SIZE as u64,
    );

    let mut seen_ids = std::collections::HashSet::new();
    let mut previous_end = super::codec::read_u64_at(&bytes, 24).unwrap()
        + super::codec::section_table_len(section_count(&bytes)).unwrap() as u64;
    previous_end = super::codec::align_up(previous_end);

    for &required_section_id in super::format::REQUIRED_SECTION_IDS {
        let section = read_section(&bytes, required_section_id);
        assert!(seen_ids.insert(section.section_id));
        assert_eq!(
            section.physical_encoding,
            super::format::PHYSICAL_ENCODING_RAW_LE
        );
        assert!(section.item_count > 0);
        assert_eq!(section.offset % 8, 0);
        assert!(section.offset >= previous_end);
        previous_end = section.offset + section.byte_len;
        assert!(usize::try_from(previous_end).unwrap() <= bytes.len());
    }
}

#[test]
fn test_writer_rejects_index_format_version_mismatch() {
    let mut index = sample_index();
    index.meta.index_format_version = super::format::FORMAT_VERSION + 1;

    let err = super::serialize_search_index(&index).unwrap_err();
    assert_eq!(
        err,
        super::Error::InvalidFormat(
            "index meta format version does not match binary format version",
        ),
    );
}

#[test]
fn test_writer_rejects_column_length_mismatch() {
    let mut index = sample_index();
    index.columns.channel_ids.pop();

    let err = super::serialize_search_index(&index).unwrap_err();
    assert_eq!(
        err,
        super::Error::InvalidFormat("column lengths do not match record_count"),
    );
}

#[test]
fn test_writer_rejects_clip_id_out_of_range() {
    let mut index = sample_index();
    index.columns.clip_ids[0] = index.dictionaries.clips.len() as u32;

    let err = super::serialize_search_index(&index).unwrap_err();
    assert_eq!(err, super::Error::InvalidFormat("clip_ids"));
}

#[test]
fn test_writer_rejects_unsorted_artist_id_list() {
    let mut index = sample_index();
    index.columns.artist_id_lists =
        crate::util::U32ListColumn::from_parts(vec![0, 2, 2, 4], vec![2, 0, 1, 2]);

    let err = super::serialize_search_index(&index).unwrap_err();
    assert_eq!(err, super::Error::InvalidFormat("artist_id_lists"));
}

#[test]
fn test_writer_rejects_empty_dictionary_string() {
    let mut index = sample_index();
    index.dictionaries.tags = crate::util::BiMap::from_ordered_strings(vec![
        "tag-a".to_string(),
        "".to_string(),
        "tag-c".to_string(),
    ])
    .unwrap();

    let err = super::serialize_search_index(&index).unwrap_err();
    assert_eq!(
        err,
        super::Error::InvalidFormat("dictionary strings must be non-empty"),
    );
}

#[test]
fn test_writer_rejects_exact_index_mismatch() {
    let mut index = sample_index();
    index.exact_indexes.artist_docs.insert(2, vec![0, 1, 2]);

    let err = super::serialize_search_index(&index).unwrap_err();
    assert_eq!(
        err,
        super::Error::InvalidFormat("exact indexes do not match column values"),
    );
}

#[test]
fn test_writer_rejects_sort_index_mismatch() {
    let mut index = sample_index();
    index.sort_indexes.published_at = crate::schema::SortIndex::new(vec![0, 1, 2]);

    let err = super::serialize_search_index(&index).unwrap_err();
    assert_eq!(
        err,
        super::Error::InvalidFormat(
            "sort index must be ordered by (published_at asc, doc_id asc)",
        ),
    );
}

#[test]
fn test_search_index_reader_reads_metadata_before_full_decode() {
    let index = sample_index();
    let bytes = serialize_index(&index);
    let reader = super::SearchIndexReader::new(&bytes).unwrap();

    assert_eq!(reader.header().record_count, 3);
    assert_eq!(
        reader.metadata_view().unwrap().builder_version(),
        "test-builder",
    );
}

#[test]
fn test_search_index_reader_exposes_borrowed_section_views() {
    let index = sample_index();
    let bytes = serialize_index(&index);
    let reader = super::SearchIndexReader::new(&bytes).unwrap();

    let clips = reader.clips_dictionary().unwrap();
    assert_eq!(clips.len(), 3);
    assert_eq!(clips.get(1).unwrap(), Some("clip-b"));

    let clip_ids = reader.clip_ids().unwrap();
    assert_eq!(clip_ids.to_vec(), vec![0, 1, 2]);

    let artist_lists = reader.artist_id_lists().unwrap();
    assert_eq!(artist_lists.get(0).unwrap().unwrap().to_vec(), vec![0, 2]);
    assert_eq!(
        artist_lists.get(1).unwrap().unwrap().to_vec(),
        Vec::<u32>::new()
    );

    let artist_docs = reader.artist_docs().unwrap();
    assert_eq!(artist_docs.get(2).unwrap().unwrap().to_vec(), vec![0, 2]);

    let sort = reader.published_at_sort().unwrap();
    assert_eq!(sort.doc_ids_asc().to_vec(), vec![1, 0, 2]);
}

#[test]
fn test_search_index_reader_rejects_missing_required_section() {
    let index = sample_index();
    let mut bytes = serialize_index(&index);
    let count = section_count(&bytes);
    write_u32_at(&mut bytes, 12, count - 1);

    let err = super::SearchIndexReader::new(&bytes).unwrap_err();
    assert!(matches!(err, super::Error::MissingSection(_)));
}

#[test]
fn test_search_index_reader_rejects_bad_magic() {
    let index = sample_index();
    let mut bytes = serialize_index(&index);
    bytes[0] ^= 0xff;

    let err = super::SearchIndexReader::new(&bytes).unwrap_err();
    assert_eq!(err, super::Error::InvalidFormat("bad magic"));
}

#[test]
fn test_search_index_reader_rejects_unsupported_version() {
    let index = sample_index();
    let mut bytes = serialize_index(&index);
    write_u32_at(&mut bytes, 8, super::format::FORMAT_VERSION + 1);

    let err = super::SearchIndexReader::new(&bytes).unwrap_err();
    assert_eq!(
        err,
        super::Error::UnsupportedVersion(super::format::FORMAT_VERSION + 1),
    );
}

#[test]
fn test_search_index_reader_rejects_required_features() {
    let index = sample_index();
    let mut bytes = serialize_index(&index);
    write_u64_at(&mut bytes, 32, 1);

    let err = super::SearchIndexReader::new(&bytes).unwrap_err();
    assert_eq!(err, super::Error::UnsupportedRequiredFeatures(1));
}

#[test]
fn test_search_index_reader_rejects_duplicate_section_id() {
    let index = sample_index();
    let mut bytes = serialize_index(&index);
    let dict_clips = read_section(&bytes, super::format::SECTION_DICT_CLIPS);
    let metadata = read_section(&bytes, super::format::SECTION_METADATA);
    let entry_offset = section_entry_offset(dict_clips.table_index);
    write_u32_at(&mut bytes, entry_offset, metadata.section_id);

    let err = super::SearchIndexReader::new(&bytes).unwrap_err();
    assert_eq!(err, super::Error::DuplicateSection(metadata.section_id));
}

#[test]
fn test_search_index_reader_rejects_unsupported_encoding() {
    let index = sample_index();
    let mut bytes = serialize_index(&index);
    let metadata = read_section(&bytes, super::format::SECTION_METADATA);
    let entry_offset = section_entry_offset(metadata.table_index);
    write_u32_at(&mut bytes, entry_offset + 4, 99);

    let err = super::SearchIndexReader::new(&bytes).unwrap_err();
    assert_eq!(err, super::Error::UnsupportedEncoding(99));
}

#[test]
fn test_search_index_reader_rejects_overlapping_sections() {
    let index = sample_index();
    let mut bytes = serialize_index(&index);
    let metadata = read_section(&bytes, super::format::SECTION_METADATA);
    let clips = read_section(&bytes, super::format::SECTION_DICT_CLIPS);
    let entry_offset = section_entry_offset(clips.table_index);
    write_u64_at(&mut bytes, entry_offset + 16, metadata.offset);

    let err = super::SearchIndexReader::new(&bytes).unwrap_err();
    assert_eq!(err, super::Error::InvalidFormat("sections overlap"));
}

#[test]
fn test_search_index_reader_rejects_dictionary_with_empty_string() {
    let index = sample_index();
    let mut bytes = serialize_index(&index);
    let clips = read_section(&bytes, super::format::SECTION_DICT_CLIPS);
    let payload = section_payload_range(clips);
    write_u32_at(&mut bytes, payload.start + 8, 0);

    let err = super::SearchIndexReader::new(&bytes)
        .unwrap()
        .clips_dictionary()
        .unwrap_err();
    assert_eq!(
        err,
        super::Error::InvalidFormat("dictionary strings must be non-empty"),
    );
}

#[test]
fn test_search_index_reader_rejects_bool_postings_that_do_not_match_column() {
    let index = sample_index();
    let mut bytes = serialize_index(&index);
    let embeddables =
        read_section(&bytes, super::format::SECTION_EXACT_EMBEDDABLE_DOCS);
    let payload = section_payload_range(embeddables);
    write_u32_at(&mut bytes, payload.start + 16, 1);

    let err = super::SearchIndexReader::new(&bytes)
        .unwrap()
        .embeddable_docs()
        .unwrap_err();
    assert_eq!(
        err,
        super::Error::InvalidFormat(
            "bool postings must partition all docs and match column values",
        ),
    );
}

#[test]
fn test_search_index_reader_rejects_sort_index_with_wrong_order() {
    let index = sample_index();
    let mut bytes = serialize_index(&index);
    let sort = read_section(&bytes, super::format::SECTION_SORT_PUBLISHED_AT);
    let payload = section_payload_range(sort);
    write_u32_at(&mut bytes, payload.start, 0);
    write_u32_at(&mut bytes, payload.start + 4, 1);
    write_u32_at(&mut bytes, payload.start + 8, 2);

    let err = super::SearchIndexReader::new(&bytes)
        .unwrap()
        .published_at_sort()
        .unwrap_err();
    assert_eq!(
        err,
        super::Error::InvalidFormat(
            "sort index must be ordered by (published_at asc, doc_id asc)",
        ),
    );
}

#[test]
fn test_search_index_reader_rejects_list_column_with_unsorted_ids() {
    let index = sample_index();
    let mut bytes = serialize_index(&index);
    let artists = read_section(&bytes, super::format::SECTION_COLUMN_ARTIST_ID_LISTS);
    let payload = section_payload_range(artists);
    let values_start =
        payload.start + (usize::try_from(index.meta.record_count).unwrap() + 1) * 4;
    write_u32_at(&mut bytes, values_start, 2);
    write_u32_at(&mut bytes, values_start + 4, 0);

    let err = super::SearchIndexReader::new(&bytes)
        .unwrap()
        .artist_id_lists()
        .unwrap_err();
    assert_eq!(err, super::Error::InvalidFormat("artist_id_lists"));
}
