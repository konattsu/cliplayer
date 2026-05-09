#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct IndexMetadata {
    pub index_format_version: u32,
    pub dataset_build_id: String,
    pub builder_version: String,
    pub record_count: u32,
}
