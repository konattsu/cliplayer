pub(super) const MAGIC: &[u8; 8] = b"CLIPIDX\0";
pub(super) const FORMAT_VERSION: u32 = 1;
pub(super) const FILE_HEADER_SIZE: usize = 48;
pub(super) const SECTION_TABLE_ENTRY_SIZE: usize = 32;
pub(super) const MAX_SECTION_COUNT: usize = 256;
pub(super) const PHYSICAL_ENCODING_RAW_LE: u32 = 1;

pub(super) const SECTION_METADATA: u32 = 0x0001;
pub(super) const SECTION_DICT_CLIPS: u32 = 0x1000;
pub(super) const SECTION_DICT_VIDEOS: u32 = 0x1001;
pub(super) const SECTION_DICT_CHANNELS: u32 = 0x1002;
pub(super) const SECTION_DICT_ARTISTS: u32 = 0x1003;
pub(super) const SECTION_DICT_TAGS: u32 = 0x1004;
pub(super) const SECTION_COLUMN_CLIP_IDS: u32 = 0x2000;
pub(super) const SECTION_COLUMN_VIDEO_IDS: u32 = 0x2001;
pub(super) const SECTION_COLUMN_PUBLISHED_ATS: u32 = 0x2002;
pub(super) const SECTION_COLUMN_CHANNEL_IDS: u32 = 0x2003;
pub(super) const SECTION_COLUMN_IS_UNLISTEDS: u32 = 0x2004;
pub(super) const SECTION_COLUMN_EMBEDDABLES: u32 = 0x2005;
pub(super) const SECTION_COLUMN_ARTIST_ID_LISTS: u32 = 0x2006;
pub(super) const SECTION_COLUMN_TAG_ID_LISTS: u32 = 0x2007;
pub(super) const SECTION_EXACT_ARTIST_DOCS: u32 = 0x3000;
pub(super) const SECTION_EXACT_TAG_DOCS: u32 = 0x3001;
pub(super) const SECTION_EXACT_CHANNEL_DOCS: u32 = 0x3002;
pub(super) const SECTION_EXACT_IS_UNLISTED_DOCS: u32 = 0x3003;
pub(super) const SECTION_EXACT_EMBEDDABLE_DOCS: u32 = 0x3004;
pub(super) const SECTION_SORT_PUBLISHED_AT: u32 = 0x4000;

pub(super) const REQUIRED_SECTION_IDS: &[u32] = &[
    SECTION_METADATA,
    SECTION_DICT_CLIPS,
    SECTION_DICT_VIDEOS,
    SECTION_DICT_CHANNELS,
    SECTION_DICT_ARTISTS,
    SECTION_DICT_TAGS,
    SECTION_COLUMN_CLIP_IDS,
    SECTION_COLUMN_VIDEO_IDS,
    SECTION_COLUMN_PUBLISHED_ATS,
    SECTION_COLUMN_CHANNEL_IDS,
    SECTION_COLUMN_IS_UNLISTEDS,
    SECTION_COLUMN_EMBEDDABLES,
    SECTION_COLUMN_ARTIST_ID_LISTS,
    SECTION_COLUMN_TAG_ID_LISTS,
    SECTION_EXACT_ARTIST_DOCS,
    SECTION_EXACT_TAG_DOCS,
    SECTION_EXACT_CHANNEL_DOCS,
    SECTION_EXACT_IS_UNLISTED_DOCS,
    SECTION_EXACT_EMBEDDABLE_DOCS,
    SECTION_SORT_PUBLISHED_AT,
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchIndexHeader {
    pub format_version: u32,
    pub section_count: u32,
    pub record_count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct SectionEntry {
    pub(super) section_id: u32,
    pub(super) offset: u64,
    pub(super) byte_len: u64,
    pub(super) item_count: u32,
    pub(super) physical_encoding: u32,
}

#[derive(Debug)]
pub(super) struct SectionToWrite {
    pub(super) section_id: u32,
    pub(super) item_count: u32,
    pub(super) data: Vec<u8>,
}
