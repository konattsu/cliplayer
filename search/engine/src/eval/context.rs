pub(crate) struct EvalContext<'a> {
    pub(crate) record_count: u32,
    pub(crate) published_ats: index_core::binary::I64SliceView<'a>,
    pub(crate) published_at_sort: index_core::binary::SortIndexView<'a>,
    pub(crate) artist_docs: index_core::binary::DensePostingsView<'a>,
    pub(crate) tag_docs: index_core::binary::DensePostingsView<'a>,
    pub(crate) channel_docs: index_core::binary::DensePostingsView<'a>,
    pub(crate) is_unlisted_docs: index_core::binary::BoolPostingsView<'a>,
    pub(crate) embeddable_docs: index_core::binary::BoolPostingsView<'a>,
}

impl<'a> EvalContext<'a> {
    pub(crate) fn new(
        reader: &index_core::binary::SearchIndexReader<'a>,
        record_count: u32,
    ) -> Result<Self, crate::error::EngineError> {
        Ok(Self {
            record_count,
            published_ats: reader.published_ats()?,
            published_at_sort: reader.published_at_sort()?,
            artist_docs: reader.artist_docs()?,
            tag_docs: reader.tag_docs()?,
            channel_docs: reader.channel_docs()?,
            is_unlisted_docs: reader.is_unlisted_docs()?,
            embeddable_docs: reader.embeddable_docs()?,
        })
    }
}
