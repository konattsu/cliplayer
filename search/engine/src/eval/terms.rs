pub(crate) fn evaluate_term(
    context: &super::context::EvalContext<'_>,
    term: &crate::api::query::resolved::TermNode,
) -> Result<crate::doc_set::DocSet, crate::EngineError> {
    use crate::api::query::resolved::TermNode;

    match term {
        TermNode::ArtistAnyIn { values } => {
            exact_any_in(context.record_count, &context.artist_docs, values)
        }
        TermNode::TagAnyIn { values } => {
            exact_any_in(context.record_count, &context.tag_docs, values)
        }
        TermNode::ChannelAnyIn { values } => {
            exact_any_in(context.record_count, &context.channel_docs, values)
        }
        TermNode::IsUnlistedEq { value } => {
            let docs = if *value {
                context.is_unlisted_docs.true_docs().to_vec()
            } else {
                context.is_unlisted_docs.false_docs().to_vec()
            };
            crate::doc_set::DocSet::from_sorted_doc_ids(docs, context.record_count)
        }
        TermNode::EmbeddableEq { value } => {
            let docs = if *value {
                context.embeddable_docs.true_docs().to_vec()
            } else {
                context.embeddable_docs.false_docs().to_vec()
            };
            crate::doc_set::DocSet::from_sorted_doc_ids(docs, context.record_count)
        }
        TermNode::PublishedAtRange(range) => {
            super::range::evaluate_published_at_range(context, range)
        }
    }
}

fn exact_any_in(
    record_count: u32,
    postings: &index_core::binary::DensePostingsView<'_>,
    values: &[u32],
) -> Result<crate::doc_set::DocSet, crate::EngineError> {
    let mut doc_ids = Vec::new();
    for &value in values {
        if let Some(posting_list) = postings.get(value as usize)? {
            doc_ids.extend(posting_list.iter());
        }
    }
    crate::doc_set::DocSet::from_unsorted_doc_ids(doc_ids, record_count)
}
