mod codec;
mod error;
mod format;
mod reader;
mod validate;
mod view;
mod writer;

pub use error::Error;
pub use format::SearchIndexHeader;
pub use reader::SearchIndexReader;
pub use view::{
    BoolPostingsView, BoolSliceView, DensePostingsView, I64SliceView, MetadataView,
    SortIndexView, StringDictionaryView, U32ListColumnView, U32SliceView,
};

pub fn serialize_search_index(
    index: &crate::schema::SearchIndex,
) -> Result<Vec<u8>, Error> {
    writer::serialize_search_index(index)
}

#[cfg(test)]
mod tests;
