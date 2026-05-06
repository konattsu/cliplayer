pub(super) fn validate_record_count(
    record_count: usize,
    columns: &crate::schema::ColumnStore,
) -> Result<(), crate::binary::Error> {
    let counts = [
        columns.clip_ids.len(),
        columns.video_ids.len(),
        columns.published_ats.len(),
        columns.channel_ids.len(),
        columns.is_unlisteds.len(),
        columns.embeddables.len(),
    ];

    if counts.into_iter().all(|count| count == record_count)
        && columns.artist_id_lists.offsets().len() == record_count + 1
        && columns.tag_id_lists.offsets().len() == record_count + 1
    {
        Ok(())
    } else {
        Err(crate::binary::Error::InvalidFormat(
            "column lengths do not match record_count",
        ))
    }
}

pub(super) fn validate_dictionary_non_empty(
    dictionaries: &crate::schema::Dictionaries,
) -> Result<(), crate::binary::Error> {
    for strings in [
        dictionaries.clips.ordered_strings(),
        dictionaries.videos.ordered_strings(),
        dictionaries.channels.ordered_strings(),
        dictionaries.artists.ordered_strings(),
        dictionaries.tags.ordered_strings(),
    ] {
        if strings.iter().any(String::is_empty) {
            return Err(crate::binary::Error::InvalidFormat(
                "dictionary strings must be non-empty",
            ));
        }
    }

    Ok(())
}

pub(super) fn validate_columns_against_dictionaries(
    columns: &crate::schema::ColumnStore,
    dictionaries: &crate::schema::Dictionaries,
) -> Result<(), crate::binary::Error> {
    validate_ids_in_range(&columns.clip_ids, dictionaries.clips.len(), "clip_ids")?;
    validate_ids_in_range(&columns.video_ids, dictionaries.videos.len(), "video_ids")?;
    validate_ids_in_range(
        &columns.channel_ids,
        dictionaries.channels.len(),
        "channel_ids",
    )?;
    validate_u32_list_column(
        &columns.artist_id_lists,
        columns.clip_ids.len(),
        dictionaries.artists.len(),
        "artist_id_lists",
    )?;
    validate_u32_list_column(
        &columns.tag_id_lists,
        columns.clip_ids.len(),
        dictionaries.tags.len(),
        "tag_id_lists",
    )?;

    Ok(())
}

pub(super) fn validate_exact_indexes_against_columns(
    columns: &crate::schema::ColumnStore,
    exact_indexes: &crate::schema::ExactIndexes,
) -> Result<(), crate::binary::Error> {
    use std::collections::HashMap;

    let record_count = columns.clip_ids.len();
    let mut expected_artist_docs = HashMap::<u32, Vec<u32>>::new();
    let mut expected_tag_docs = HashMap::<u32, Vec<u32>>::new();
    let mut expected_channel_docs = HashMap::<u32, Vec<u32>>::new();
    let mut expected_is_unlisted_docs = [Vec::new(), Vec::new()];
    let mut expected_embeddable_docs = [Vec::new(), Vec::new()];

    for doc_id in 0..record_count {
        for &artist_id in columns.artist_id_lists.get(doc_id) {
            expected_artist_docs
                .entry(artist_id)
                .or_default()
                .push(doc_id as u32);
        }
        for &tag_id in columns.tag_id_lists.get(doc_id) {
            expected_tag_docs
                .entry(tag_id)
                .or_default()
                .push(doc_id as u32);
        }
        expected_channel_docs
            .entry(columns.channel_ids[doc_id])
            .or_default()
            .push(doc_id as u32);
        expected_is_unlisted_docs[usize::from(columns.is_unlisteds[doc_id])]
            .push(doc_id as u32);
        expected_embeddable_docs[usize::from(columns.embeddables[doc_id])]
            .push(doc_id as u32);
    }

    if exact_indexes.artist_docs != expected_artist_docs
        || exact_indexes.tag_docs != expected_tag_docs
        || exact_indexes.channel_docs != expected_channel_docs
        || exact_indexes.is_unlisted_docs != expected_is_unlisted_docs
        || exact_indexes.embeddable_docs != expected_embeddable_docs
    {
        return Err(crate::binary::Error::InvalidFormat(
            "exact indexes do not match column values",
        ));
    }

    Ok(())
}

pub(super) fn validate_sort_index(
    doc_ids_asc: &[u32],
    published_ats: &[crate::schema::TimestampSecs],
) -> Result<(), crate::binary::Error> {
    use crate::binary::Error;

    if doc_ids_asc.len() != published_ats.len() {
        return Err(Error::InvalidFormat(
            "sort index length does not match record_count",
        ));
    }

    let mut seen = vec![false; doc_ids_asc.len()];
    for &doc_id in doc_ids_asc {
        let doc_id = usize::try_from(doc_id)
            .map_err(|_| Error::InvalidFormat("doc_id does not fit usize"))?;
        if doc_id >= published_ats.len() {
            return Err(Error::InvalidFormat("sort index doc_id out of range"));
        }
        if seen[doc_id] {
            return Err(Error::InvalidFormat("sort index doc_ids must be unique"));
        }
        seen[doc_id] = true;
    }

    for window in doc_ids_asc.windows(2) {
        let left_doc = window[0] as usize;
        let right_doc = window[1] as usize;
        let left = (published_ats[left_doc], window[0]);
        let right = (published_ats[right_doc], window[1]);
        if left > right {
            return Err(Error::InvalidFormat(
                "sort index must be ordered by (published_at asc, doc_id asc)",
            ));
        }
    }

    Ok(())
}

pub(super) fn validate_ids_in_range(
    ids: &[u32],
    max_id: usize,
    name: &'static str,
) -> Result<(), crate::binary::Error> {
    if ids.iter().all(|&id| (id as usize) < max_id) {
        Ok(())
    } else {
        Err(crate::binary::Error::InvalidFormat(name))
    }
}

pub(super) fn validate_u32_list_column(
    column: &crate::util::U32ListColumn,
    record_count: usize,
    max_id: usize,
    name: &'static str,
) -> Result<(), crate::binary::Error> {
    use crate::binary::Error;

    let expected_offsets_len = record_count
        .checked_add(1)
        .ok_or(Error::TooLarge("record count"))?;
    let offsets = column.offsets();
    let values = column.values();

    if offsets.len() != expected_offsets_len {
        return Err(Error::InvalidFormat(
            "column lengths do not match record_count",
        ));
    }
    if offsets.first().copied() != Some(0) {
        return Err(Error::InvalidFormat(name));
    }
    if !offsets.windows(2).all(|window| window[0] <= window[1]) {
        return Err(Error::InvalidFormat(name));
    }

    let value_count = u32::try_from(values.len()).map_err(|_| Error::TooLarge(name))?;
    if offsets.last().copied() != Some(value_count) {
        return Err(Error::InvalidFormat(name));
    }

    validate_sorted_unique_ids_per_list(offsets, values, max_id, name)
}

pub(super) fn validate_sorted_unique_ids_per_list(
    offsets: &[u32],
    values: &[u32],
    max_id: usize,
    name: &'static str,
) -> Result<(), crate::binary::Error> {
    use crate::binary::Error;

    for index in 0..offsets.len() - 1 {
        let start = offsets[index] as usize;
        let end = offsets[index + 1] as usize;
        let list = &values[start..end];
        if !list.windows(2).all(|window| window[0] < window[1]) {
            return Err(Error::InvalidFormat(name));
        }
        if list.iter().any(|&id| (id as usize) >= max_id) {
            return Err(Error::InvalidFormat(name));
        }
    }

    Ok(())
}
