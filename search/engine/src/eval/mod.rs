mod boolean;
mod context;
mod range;
mod terms;

pub(crate) fn evaluate_query(
    reader: &index_core::binary::SearchIndexReader<'_>,
    record_count: u32,
    query: &crate::api::query::resolved::QueryNode,
) -> Result<crate::doc_set::DocSet, crate::error::EngineError> {
    let context = context::EvalContext::new(reader, record_count)?;
    boolean::evaluate_node(&context, query)
}
