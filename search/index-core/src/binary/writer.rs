pub fn serialize_search_index(
    index: &crate::schema::SearchIndex,
) -> Result<Vec<u8>, crate::binary::Error> {
    BinaryWriter::new(index).write_to_vec()
}

struct BinaryWriter<'a> {
    index: &'a crate::schema::SearchIndex,
}

impl<'a> BinaryWriter<'a> {
    fn new(index: &'a crate::schema::SearchIndex) -> Self {
        Self { index }
    }

    fn write_to_vec(&self) -> Result<Vec<u8>, crate::binary::Error> {
        use crate::binary::Error;
        use crate::binary::codec::{
            align_up, pad_to_offset, section_table_len, write_u32, write_u64,
        };
        use crate::binary::format::{FILE_HEADER_SIZE, PHYSICAL_ENCODING_RAW_LE};
        use crate::binary::validate::{
            validate_columns_against_dictionaries, validate_dictionary_non_empty,
            validate_exact_indexes_against_columns, validate_record_count,
            validate_sort_index,
        };

        if self.index.meta.index_format_version != crate::binary::format::FORMAT_VERSION
        {
            return Err(Error::InvalidFormat(
                "index meta format version does not match binary format version",
            ));
        }

        let record_count = self.index.meta.record_count as usize;
        validate_record_count(record_count, &self.index.columns)?;
        validate_dictionary_non_empty(&self.index.dictionaries)?;
        validate_columns_against_dictionaries(
            &self.index.columns,
            &self.index.dictionaries,
        )?;
        validate_exact_indexes_against_columns(
            &self.index.columns,
            &self.index.exact_indexes,
        )?;
        validate_sort_index(
            self.index.sort_indexes.published_at.doc_ids_asc(),
            &self.index.columns.published_ats,
        )?;

        let sections = self.build_sections()?;
        let section_count = sections.len();
        let section_table_offset = FILE_HEADER_SIZE as u64;
        let section_table_len = section_table_len(section_count as u32)? as u64;
        let mut next_offset = align_up(
            section_table_offset
                .checked_add(section_table_len)
                .ok_or(Error::Io("section table range overflow"))?,
        );
        let mut entries = Vec::with_capacity(section_count);

        for section in sections {
            let offset = next_offset;
            let byte_len = u64::try_from(section.data.len())
                .map_err(|_| Error::TooLarge("section byte length"))?;
            next_offset = align_up(
                offset
                    .checked_add(byte_len)
                    .ok_or(Error::Io("section range overflow"))?,
            );

            entries.push((
                crate::binary::format::SectionEntry {
                    section_id: section.section_id,
                    offset,
                    byte_len,
                    item_count: section.item_count,
                    physical_encoding: PHYSICAL_ENCODING_RAW_LE,
                },
                section.data,
            ));
        }

        let total_len = usize::try_from(next_offset)
            .map_err(|_| Error::TooLarge("encoded file length"))?;
        let mut out = Vec::with_capacity(total_len);
        self.write_header(
            &mut out,
            u32::try_from(section_count)
                .map_err(|_| Error::TooLarge("section count"))?,
            self.index.meta.record_count,
            section_table_offset,
        );

        for (entry, _) in &entries {
            write_u32(&mut out, entry.section_id);
            write_u32(&mut out, entry.physical_encoding);
            write_u32(&mut out, entry.item_count);
            write_u32(&mut out, 0);
            write_u64(&mut out, entry.offset);
            write_u64(&mut out, entry.byte_len);
        }

        for (entry, data) in entries {
            pad_to_offset(&mut out, entry.offset)?;
            out.extend_from_slice(&data);
        }

        Ok(out)
    }

    fn write_header(
        &self,
        out: &mut Vec<u8>,
        section_count: u32,
        record_count: u32,
        section_table_offset: u64,
    ) {
        use crate::binary::codec::{write_u32, write_u64};

        out.extend_from_slice(crate::binary::format::MAGIC);
        write_u32(out, crate::binary::format::FORMAT_VERSION);
        write_u32(out, section_count);
        write_u32(out, record_count);
        write_u32(out, 0);
        write_u64(out, section_table_offset);
        write_u64(out, 0);
        write_u64(out, 0);
    }

    fn build_sections(
        &self,
    ) -> Result<Vec<crate::binary::format::SectionToWrite>, crate::binary::Error> {
        use crate::binary::codec::{
            encode_bool_postings, encode_bool_slice, encode_dense_postings,
            encode_i64_slice, encode_metadata, encode_u32_list_column,
            encode_u32_slice,
        };
        use crate::binary::format::{
            SECTION_COLUMN_ARTIST_ID_LISTS, SECTION_COLUMN_CHANNEL_IDS,
            SECTION_COLUMN_CLIP_IDS, SECTION_COLUMN_EMBEDDABLES,
            SECTION_COLUMN_IS_UNLISTEDS, SECTION_COLUMN_PUBLISHED_ATS,
            SECTION_COLUMN_TAG_ID_LISTS, SECTION_COLUMN_VIDEO_IDS,
            SECTION_DICT_ARTISTS, SECTION_DICT_CHANNELS, SECTION_DICT_CLIPS,
            SECTION_DICT_TAGS, SECTION_DICT_VIDEOS, SECTION_EXACT_ARTIST_DOCS,
            SECTION_EXACT_CHANNEL_DOCS, SECTION_EXACT_EMBEDDABLE_DOCS,
            SECTION_EXACT_IS_UNLISTED_DOCS, SECTION_EXACT_TAG_DOCS, SECTION_METADATA,
            SECTION_SORT_PUBLISHED_AT,
        };

        let artist_term_count = self.index.dictionaries.artists.len();
        let tag_term_count = self.index.dictionaries.tags.len();
        let channel_term_count = self.index.dictionaries.channels.len();

        Ok(vec![
            self.section(
                SECTION_METADATA,
                1,
                encode_metadata(
                    &self.index.meta.dataset_build_id,
                    &self.index.meta.builder_version,
                )?,
            ),
            self.dictionary_section(
                SECTION_DICT_CLIPS,
                "clip dictionary length",
                &self.index.dictionaries.clips,
            )?,
            self.dictionary_section(
                SECTION_DICT_VIDEOS,
                "video dictionary length",
                &self.index.dictionaries.videos,
            )?,
            self.dictionary_section(
                SECTION_DICT_CHANNELS,
                "channel dictionary length",
                &self.index.dictionaries.channels,
            )?,
            self.dictionary_section(
                SECTION_DICT_ARTISTS,
                "artist dictionary length",
                &self.index.dictionaries.artists,
            )?,
            self.dictionary_section(
                SECTION_DICT_TAGS,
                "tag dictionary length",
                &self.index.dictionaries.tags,
            )?,
            self.record_section(
                SECTION_COLUMN_CLIP_IDS,
                encode_u32_slice(&self.index.columns.clip_ids),
            ),
            self.record_section(
                SECTION_COLUMN_VIDEO_IDS,
                encode_u32_slice(&self.index.columns.video_ids),
            ),
            self.record_section(
                SECTION_COLUMN_PUBLISHED_ATS,
                encode_i64_slice(&self.index.columns.published_ats),
            ),
            self.record_section(
                SECTION_COLUMN_CHANNEL_IDS,
                encode_u32_slice(&self.index.columns.channel_ids),
            ),
            self.record_section(
                SECTION_COLUMN_IS_UNLISTEDS,
                encode_bool_slice(&self.index.columns.is_unlisteds),
            ),
            self.record_section(
                SECTION_COLUMN_EMBEDDABLES,
                encode_bool_slice(&self.index.columns.embeddables),
            ),
            self.record_section(
                SECTION_COLUMN_ARTIST_ID_LISTS,
                encode_u32_list_column(&self.index.columns.artist_id_lists),
            ),
            self.record_section(
                SECTION_COLUMN_TAG_ID_LISTS,
                encode_u32_list_column(&self.index.columns.tag_id_lists),
            ),
            self.postings_section(
                SECTION_EXACT_ARTIST_DOCS,
                artist_term_count,
                "artist postings term count",
                encode_dense_postings(
                    artist_term_count,
                    &self.index.exact_indexes.artist_docs,
                )?,
            )?,
            self.postings_section(
                SECTION_EXACT_TAG_DOCS,
                tag_term_count,
                "tag postings term count",
                encode_dense_postings(
                    tag_term_count,
                    &self.index.exact_indexes.tag_docs,
                )?,
            )?,
            self.postings_section(
                SECTION_EXACT_CHANNEL_DOCS,
                channel_term_count,
                "channel postings term count",
                encode_dense_postings(
                    channel_term_count,
                    &self.index.exact_indexes.channel_docs,
                )?,
            )?,
            self.section(
                SECTION_EXACT_IS_UNLISTED_DOCS,
                2,
                encode_bool_postings(&self.index.exact_indexes.is_unlisted_docs)?,
            ),
            self.section(
                SECTION_EXACT_EMBEDDABLE_DOCS,
                2,
                encode_bool_postings(&self.index.exact_indexes.embeddable_docs)?,
            ),
            self.record_section(
                SECTION_SORT_PUBLISHED_AT,
                encode_u32_slice(self.index.sort_indexes.published_at.doc_ids_asc()),
            ),
        ])
    }

    fn section(
        &self,
        section_id: u32,
        item_count: u32,
        data: Vec<u8>,
    ) -> crate::binary::format::SectionToWrite {
        crate::binary::format::SectionToWrite {
            section_id,
            item_count,
            data,
        }
    }

    fn record_section(
        &self,
        section_id: u32,
        data: Vec<u8>,
    ) -> crate::binary::format::SectionToWrite {
        self.section(section_id, self.index.meta.record_count, data)
    }

    fn dictionary_section<Id>(
        &self,
        section_id: u32,
        len_label: &'static str,
        dictionary: &crate::util::BiMap<Id>,
    ) -> Result<crate::binary::format::SectionToWrite, crate::binary::Error>
    where
        Id: From<u32> + Copy + std::cmp::Eq + std::hash::Hash,
    {
        use crate::binary::codec::encode_dictionary;

        Ok(self.section(
            section_id,
            self.item_count(dictionary.len(), len_label)?,
            encode_dictionary(dictionary)?,
        ))
    }

    fn postings_section(
        &self,
        section_id: u32,
        term_count: usize,
        len_label: &'static str,
        data: Vec<u8>,
    ) -> Result<crate::binary::format::SectionToWrite, crate::binary::Error> {
        Ok(self.section(section_id, self.item_count(term_count, len_label)?, data))
    }

    fn item_count(
        &self,
        len: usize,
        len_label: &'static str,
    ) -> Result<u32, crate::binary::Error> {
        u32::try_from(len).map_err(|_| crate::binary::Error::TooLarge(len_label))
    }
}
