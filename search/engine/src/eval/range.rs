pub(crate) fn evaluate_published_at_range(
    context: &super::context::EvalContext<'_>,
    range: &crate::api::query::types::DateRange,
) -> Result<crate::doc_set::DocSet, crate::EngineError> {
    let start = match range.lower {
        Some(lower) => lower_bound(context, lower.value, lower.inclusive)?,
        None => 0,
    };
    let end = match range.upper {
        Some(upper) => upper_bound(context, upper.value, upper.inclusive)?,
        None => context.published_at_sort.doc_ids_asc().len(),
    };

    if start >= end {
        return Ok(crate::doc_set::DocSet::Empty);
    }

    let hit_count = end - start;
    if should_materialize_as_sorted_doc_ids(context.record_count, hit_count) {
        build_sorted_doc_ids(context, start, end)
    } else {
        build_bitset(context, start, end)
    }
}

fn should_materialize_as_sorted_doc_ids(record_count: u32, hit_count: usize) -> bool {
    use index_core::schema::ids::DocId;

    let sorted_doc_ids_byte_len = hit_count * std::mem::size_of::<DocId>();
    let bitset_byte_len = crate::doc_set::bitset_byte_len(record_count);
    sorted_doc_ids_byte_len <= bitset_byte_len
}

fn build_sorted_doc_ids(
    context: &super::context::EvalContext<'_>,
    start: usize,
    end: usize,
) -> Result<crate::doc_set::DocSet, crate::EngineError> {
    let mut doc_ids = Vec::with_capacity(end - start);
    for ordinal in start..end {
        doc_ids.push(doc_id_at_ordinal(context, ordinal)?);
    }
    crate::doc_set::DocSet::from_unsorted_doc_ids(doc_ids, context.record_count)
}

fn build_bitset(
    context: &super::context::EvalContext<'_>,
    start: usize,
    end: usize,
) -> Result<crate::doc_set::DocSet, crate::EngineError> {
    use crate::doc_set;
    use crate::doc_set::DocSet;

    let mut bits = vec![0; doc_set::word_len(context.record_count)];
    for ordinal in start..end {
        doc_set::set_bit(&mut bits, doc_id_at_ordinal(context, ordinal)?);
    }
    Ok(DocSet::BitSet(bits))
}

fn lower_bound(
    context: &super::context::EvalContext<'_>,
    target: index_core::schema::TimestampSecs,
    inclusive: bool,
) -> Result<usize, crate::EngineError> {
    partition_point(context, |value| {
        if inclusive {
            value < target
        } else {
            value <= target
        }
    })
}

fn upper_bound(
    context: &super::context::EvalContext<'_>,
    target: index_core::schema::TimestampSecs,
    inclusive: bool,
) -> Result<usize, crate::EngineError> {
    partition_point(context, |value| {
        if inclusive {
            value <= target
        } else {
            value < target
        }
    })
}

fn partition_point(
    context: &super::context::EvalContext<'_>,
    predicate: impl Fn(index_core::schema::TimestampSecs) -> bool,
) -> Result<usize, crate::EngineError> {
    let mut left = 0usize;
    let mut right = context.published_at_sort.doc_ids_asc().len();
    while left < right {
        let mid = (left + right) / 2;
        if predicate(timestamp_at_ordinal(context, mid)?) {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    Ok(left)
}

fn timestamp_at_ordinal(
    context: &super::context::EvalContext<'_>,
    ordinal: usize,
) -> Result<index_core::schema::TimestampSecs, crate::EngineError> {
    let doc_id = doc_id_at_ordinal(context, ordinal)?;
    let value = context.published_ats.get(doc_id as usize).ok_or(
        crate::EngineError::InternalIndex("published_at column out of bounds"),
    )?;
    Ok(index_core::schema::TimestampSecs::from(value))
}

fn doc_id_at_ordinal(
    context: &super::context::EvalContext<'_>,
    ordinal: usize,
) -> Result<index_core::schema::ids::DocId, crate::EngineError> {
    context.published_at_sort.doc_ids_asc().get(ordinal).ok_or(
        crate::EngineError::InternalIndex("sort index ordinal out of bounds"),
    )
}
