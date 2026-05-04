pub fn build_search_index(
    music_root: &std::path::Path,
) -> anyhow::Result<index_core::schema::SearchIndex> {
    let data = crate::build::load::load_data(music_root)?;
    build_search_index_from_loaded_data(data)
}

pub fn build_search_index_binary(
    music_root: &std::path::Path,
) -> anyhow::Result<Vec<u8>> {
    let index = build_search_index(music_root)?;
    Ok(index_core::binary::serialize_search_index(&index)?)
}

pub(crate) fn build_search_index_from_loaded_data(
    data: crate::build::load::LoadedData,
) -> anyhow::Result<index_core::schema::SearchIndex> {
    let dictionaries = crate::build::dictionaries::build_dictionaries(&data);
    let normalized =
        crate::build::normalize::normalize_clip_records(&data, &dictionaries)?;
    let index_build_id = generate_index_build_id();

    Ok(index_core::schema::SearchIndex {
        meta: index_core::schema::IndexMetadata {
            index_format_version: 1,
            index_build_id,
            builder_version: env!("CARGO_PKG_VERSION").to_string(),
            record_count: u32::try_from(normalized.len())
                .expect("record count fits within u32"),
        },
        dictionaries,
        columns: build_columns(&normalized),
        exact_indexes: build_exact_indexes(&normalized),
        sort_indexes: build_sort_indexes(&normalized),
    })
}

fn build_columns(
    normalized: &[crate::build::normalize::NormalizedClipRecord],
) -> index_core::schema::ColumnStore {
    index_core::schema::ColumnStore {
        clip_ids: normalized.iter().map(|record| record.clip_id).collect(),
        video_ids: normalized.iter().map(|record| record.video_id).collect(),
        published_ats: normalized
            .iter()
            .map(|record| record.published_at)
            .collect(),
        channel_ids: normalized.iter().map(|record| record.channel_id).collect(),
        is_unlisteds: normalized.iter().map(|record| record.is_unlisted).collect(),
        embeddables: normalized.iter().map(|record| record.embeddable).collect(),
        artist_id_lists: index_core::util::U32ListColumn::build(
            &normalized
                .iter()
                .map(|record| record.artist_ids.clone())
                .collect::<Vec<_>>(),
        ),
        tag_id_lists: index_core::util::U32ListColumn::build(
            &normalized
                .iter()
                .map(|record| record.tag_ids.clone())
                .collect::<Vec<_>>(),
        ),
    }
}

fn generate_index_build_id() -> u64 {
    use std::hash::Hash;
    use std::hash::Hasher;

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
        .hash(&mut hasher);
    std::process::id().hash(&mut hasher);
    hasher.finish()
}

fn build_exact_indexes(
    normalized: &[crate::build::normalize::NormalizedClipRecord],
) -> index_core::schema::ExactIndexes {
    let mut exact_indexes = index_core::schema::ExactIndexes::default();

    for record in normalized {
        for artist_id in &record.artist_ids {
            exact_indexes
                .artist_docs
                .entry(*artist_id)
                .or_default()
                .push(record.doc_id);
        }
        for tag_id in &record.tag_ids {
            exact_indexes
                .tag_docs
                .entry(*tag_id)
                .or_default()
                .push(record.doc_id);
        }
        exact_indexes
            .channel_docs
            .entry(record.channel_id)
            .or_default()
            .push(record.doc_id);
        exact_indexes.is_unlisted_docs[usize::from(record.is_unlisted)]
            .push(record.doc_id);
        exact_indexes.embeddable_docs[usize::from(record.embeddable)]
            .push(record.doc_id);
    }

    exact_indexes
}

fn build_sort_indexes(
    normalized: &[crate::build::normalize::NormalizedClipRecord],
) -> index_core::schema::SortIndexes {
    let mut published_at = normalized
        .iter()
        .map(|record| (record.published_at, record.doc_id))
        .collect::<Vec<_>>();
    published_at.sort_by(|left, right| left.0.cmp(&right.0).then(left.1.cmp(&right.1)));

    index_core::schema::SortIndexes {
        published_at: index_core::schema::SortIndex::new(
            published_at
                .into_iter()
                .map(|(_, doc_id)| doc_id)
                .collect::<Vec<_>>(),
        ),
    }
}
