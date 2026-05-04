#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchIndexReader<'a> {
    bytes: &'a [u8],
    header: crate::binary::format::SearchIndexHeader,
    sections: std::sync::Arc<
        std::collections::HashMap<u32, crate::binary::format::SectionEntry>,
    >,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedIndexLayout {
    header: crate::binary::format::SearchIndexHeader,
    sections: std::sync::Arc<
        std::collections::HashMap<u32, crate::binary::format::SectionEntry>,
    >,
    file_len: usize,
}

impl ValidatedIndexLayout {
    pub fn header(&self) -> &crate::binary::format::SearchIndexHeader {
        &self.header
    }
}

impl<'a> SearchIndexReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Result<Self, crate::binary::Error> {
        let layout = Self::validate_layout(bytes)?;
        Self::from_validated_layout(bytes, &layout)
    }

    pub fn validate_layout(
        bytes: &[u8],
    ) -> Result<ValidatedIndexLayout, crate::binary::Error> {
        use crate::binary::Error;
        use crate::binary::codec;
        use crate::binary::format::{
            FILE_HEADER_SIZE, FORMAT_VERSION, MAX_SECTION_COUNT,
            PHYSICAL_ENCODING_RAW_LE, REQUIRED_SECTION_IDS, SECTION_TABLE_ENTRY_SIZE,
        };

        if bytes.len() < FILE_HEADER_SIZE {
            return Err(Error::InvalidFormat("file too small for header"));
        }

        if &bytes[..crate::binary::format::MAGIC.len()] != crate::binary::format::MAGIC
        {
            return Err(Error::InvalidFormat("bad magic"));
        }

        let format_version = codec::read_u32_at(bytes, 8)?;
        if format_version != FORMAT_VERSION {
            return Err(Error::UnsupportedVersion(format_version));
        }

        let section_count = codec::read_u32_at(bytes, 12)?;
        if (section_count as usize) > MAX_SECTION_COUNT {
            return Err(Error::InvalidFormat("section count is too large"));
        }

        let record_count = codec::read_u32_at(bytes, 16)?;
        let section_table_offset = codec::read_u64_at(bytes, 24)?;
        let required_features = codec::read_u64_at(bytes, 32)?;
        let _optional_features = codec::read_u64_at(bytes, 40)?;

        if required_features != 0 {
            return Err(Error::UnsupportedRequiredFeatures(required_features));
        }

        let section_table_offset = usize::try_from(section_table_offset)
            .map_err(|_| Error::InvalidFormat("section table offset does not fit"))?;
        let section_table_len = codec::section_table_len(section_count)?;
        let section_table_end = section_table_offset
            .checked_add(section_table_len)
            .ok_or(Error::InvalidFormat("section table range overflow"))?;

        if section_table_offset < FILE_HEADER_SIZE {
            return Err(Error::InvalidFormat("section table overlaps header"));
        }
        if section_table_end > bytes.len() {
            return Err(Error::InvalidFormat("section table out of bounds"));
        }

        let mut sections =
            std::collections::HashMap::with_capacity(section_count as usize);
        let mut by_offset = Vec::with_capacity(section_count as usize);

        for index in 0..(section_count as usize) {
            let base = section_table_offset + index * SECTION_TABLE_ENTRY_SIZE;
            let section_id = codec::read_u32_at(bytes, base)?;
            let physical_encoding = codec::read_u32_at(bytes, base + 4)?;
            let item_count = codec::read_u32_at(bytes, base + 8)?;
            let offset = codec::read_u64_at(bytes, base + 16)?;
            let byte_len = codec::read_u64_at(bytes, base + 24)?;

            if physical_encoding != PHYSICAL_ENCODING_RAW_LE {
                return Err(Error::UnsupportedEncoding(physical_encoding));
            }
            if offset % 8 != 0 {
                return Err(Error::InvalidFormat(
                    "section offset is not 8-byte aligned",
                ));
            }

            let payload_end = offset
                .checked_add(byte_len)
                .ok_or(Error::InvalidFormat("section range overflow"))?;
            let offset_usize = usize::try_from(offset)
                .map_err(|_| Error::InvalidFormat("section offset does not fit"))?;
            let payload_end_usize = usize::try_from(payload_end)
                .map_err(|_| Error::InvalidFormat("section end does not fit"))?;

            if offset_usize < section_table_end {
                return Err(Error::InvalidFormat("section overlaps table"));
            }
            if payload_end_usize > bytes.len() {
                return Err(Error::InvalidFormat("section out of bounds"));
            }

            let entry = crate::binary::format::SectionEntry {
                section_id,
                offset,
                byte_len,
                item_count,
                physical_encoding,
            };

            if sections.insert(section_id, entry).is_some() {
                return Err(Error::DuplicateSection(section_id));
            }

            by_offset.push(entry);
        }

        by_offset.sort_by_key(|entry| entry.offset);
        for window in by_offset.windows(2) {
            let current = window[0];
            let next = window[1];
            let current_end = current
                .offset
                .checked_add(current.byte_len)
                .ok_or(Error::InvalidFormat("section range overflow"))?;
            if current_end > next.offset {
                return Err(Error::InvalidFormat("sections overlap"));
            }
        }

        for &section_id in REQUIRED_SECTION_IDS {
            if !sections.contains_key(&section_id) {
                return Err(Error::MissingSection(section_id));
            }
        }

        Ok(ValidatedIndexLayout {
            header: crate::binary::format::SearchIndexHeader {
                format_version,
                section_count,
                record_count,
            },
            sections: std::sync::Arc::new(sections),
            file_len: bytes.len(),
        })
    }

    pub fn from_validated_layout(
        bytes: &'a [u8],
        layout: &ValidatedIndexLayout,
    ) -> Result<Self, crate::binary::Error> {
        if bytes.len() != layout.file_len {
            return Err(crate::binary::Error::InvalidFormat(
                "validated layout does not match file length",
            ));
        }

        Ok(Self {
            bytes,
            header: layout.header.clone(),
            sections: layout.sections.clone(),
        })
    }

    pub fn header(&self) -> &crate::binary::format::SearchIndexHeader {
        &self.header
    }

    pub fn metadata_view(
        &self,
    ) -> Result<crate::binary::MetadataView<'a>, crate::binary::Error> {
        let section = self.required_section(crate::binary::format::SECTION_METADATA)?;
        crate::binary::MetadataView::new(
            self.section_bytes(section)?,
            section.item_count,
        )
    }

    pub fn clips_dictionary(
        &self,
    ) -> Result<crate::binary::StringDictionaryView<'a>, crate::binary::Error> {
        self.read_dictionary_view(crate::binary::format::SECTION_DICT_CLIPS)
    }

    pub fn videos_dictionary(
        &self,
    ) -> Result<crate::binary::StringDictionaryView<'a>, crate::binary::Error> {
        self.read_dictionary_view(crate::binary::format::SECTION_DICT_VIDEOS)
    }

    pub fn channels_dictionary(
        &self,
    ) -> Result<crate::binary::StringDictionaryView<'a>, crate::binary::Error> {
        self.read_dictionary_view(crate::binary::format::SECTION_DICT_CHANNELS)
    }

    pub fn artists_dictionary(
        &self,
    ) -> Result<crate::binary::StringDictionaryView<'a>, crate::binary::Error> {
        self.read_dictionary_view(crate::binary::format::SECTION_DICT_ARTISTS)
    }

    pub fn tags_dictionary(
        &self,
    ) -> Result<crate::binary::StringDictionaryView<'a>, crate::binary::Error> {
        self.read_dictionary_view(crate::binary::format::SECTION_DICT_TAGS)
    }

    pub fn clip_ids(
        &self,
    ) -> Result<crate::binary::U32SliceView<'a>, crate::binary::Error> {
        let view = self.read_u32_vector_view(
            crate::binary::format::SECTION_COLUMN_CLIP_IDS,
            self.header.record_count as usize,
        )?;
        validate_ids_in_range(&view, self.clips_dictionary()?.len(), "clip_ids")?;
        Ok(view)
    }

    pub fn video_ids(
        &self,
    ) -> Result<crate::binary::U32SliceView<'a>, crate::binary::Error> {
        let view = self.read_u32_vector_view(
            crate::binary::format::SECTION_COLUMN_VIDEO_IDS,
            self.header.record_count as usize,
        )?;
        validate_ids_in_range(&view, self.videos_dictionary()?.len(), "video_ids")?;
        Ok(view)
    }

    pub fn published_ats(
        &self,
    ) -> Result<crate::binary::I64SliceView<'a>, crate::binary::Error> {
        self.read_i64_vector_view(
            crate::binary::format::SECTION_COLUMN_PUBLISHED_ATS,
            self.header.record_count as usize,
        )
    }

    pub fn channel_ids(
        &self,
    ) -> Result<crate::binary::U32SliceView<'a>, crate::binary::Error> {
        let view = self.read_u32_vector_view(
            crate::binary::format::SECTION_COLUMN_CHANNEL_IDS,
            self.header.record_count as usize,
        )?;
        validate_ids_in_range(&view, self.channels_dictionary()?.len(), "channel_ids")?;
        Ok(view)
    }

    pub fn is_unlisteds(
        &self,
    ) -> Result<crate::binary::BoolSliceView<'a>, crate::binary::Error> {
        self.read_bool_vector_view(
            crate::binary::format::SECTION_COLUMN_IS_UNLISTEDS,
            self.header.record_count as usize,
        )
    }

    pub fn embeddables(
        &self,
    ) -> Result<crate::binary::BoolSliceView<'a>, crate::binary::Error> {
        self.read_bool_vector_view(
            crate::binary::format::SECTION_COLUMN_EMBEDDABLES,
            self.header.record_count as usize,
        )
    }

    pub fn artist_id_lists(
        &self,
    ) -> Result<crate::binary::U32ListColumnView<'a>, crate::binary::Error> {
        self.read_u32_list_column_view(
            crate::binary::format::SECTION_COLUMN_ARTIST_ID_LISTS,
            self.header.record_count as usize,
            self.artists_dictionary()?.len(),
            "artist_id_lists",
        )
    }

    pub fn tag_id_lists(
        &self,
    ) -> Result<crate::binary::U32ListColumnView<'a>, crate::binary::Error> {
        self.read_u32_list_column_view(
            crate::binary::format::SECTION_COLUMN_TAG_ID_LISTS,
            self.header.record_count as usize,
            self.tags_dictionary()?.len(),
            "tag_id_lists",
        )
    }

    pub fn artist_docs(
        &self,
    ) -> Result<crate::binary::DensePostingsView<'a>, crate::binary::Error> {
        self.read_dense_postings_view(
            crate::binary::format::SECTION_EXACT_ARTIST_DOCS,
            self.artists_dictionary()?.len(),
        )
    }

    pub fn tag_docs(
        &self,
    ) -> Result<crate::binary::DensePostingsView<'a>, crate::binary::Error> {
        self.read_dense_postings_view(
            crate::binary::format::SECTION_EXACT_TAG_DOCS,
            self.tags_dictionary()?.len(),
        )
    }

    pub fn channel_docs(
        &self,
    ) -> Result<crate::binary::DensePostingsView<'a>, crate::binary::Error> {
        self.read_dense_postings_view(
            crate::binary::format::SECTION_EXACT_CHANNEL_DOCS,
            self.channels_dictionary()?.len(),
        )
    }

    pub fn is_unlisted_docs(
        &self,
    ) -> Result<crate::binary::BoolPostingsView<'a>, crate::binary::Error> {
        let column = self.is_unlisteds()?;
        let view = self.read_bool_postings_view(
            crate::binary::format::SECTION_EXACT_IS_UNLISTED_DOCS,
        )?;
        view.validate_matches_column(column)?;
        Ok(view)
    }

    pub fn embeddable_docs(
        &self,
    ) -> Result<crate::binary::BoolPostingsView<'a>, crate::binary::Error> {
        let column = self.embeddables()?;
        let view = self.read_bool_postings_view(
            crate::binary::format::SECTION_EXACT_EMBEDDABLE_DOCS,
        )?;
        view.validate_matches_column(column)?;
        Ok(view)
    }

    pub fn published_at_sort(
        &self,
    ) -> Result<crate::binary::SortIndexView<'a>, crate::binary::Error> {
        let published_ats = self.published_ats()?;
        let doc_ids = self.read_u32_vector_view(
            crate::binary::format::SECTION_SORT_PUBLISHED_AT,
            published_ats.len(),
        )?;
        let view = crate::binary::SortIndexView::new(doc_ids);
        view.validate_against_timestamps(&published_ats)?;
        Ok(view)
    }
    fn required_section(
        &self,
        section_id: u32,
    ) -> Result<crate::binary::format::SectionEntry, crate::binary::Error> {
        self.sections
            .get(&section_id)
            .copied()
            .ok_or(crate::binary::Error::MissingSection(section_id))
    }

    fn section_bytes(
        &self,
        section: crate::binary::format::SectionEntry,
    ) -> Result<&'a [u8], crate::binary::Error> {
        use crate::binary::Error;

        let offset = usize::try_from(section.offset)
            .map_err(|_| Error::InvalidFormat("section offset does not fit"))?;
        let byte_len = usize::try_from(section.byte_len)
            .map_err(|_| Error::InvalidFormat("section length does not fit"))?;
        self.bytes
            .get(offset..offset + byte_len)
            .ok_or(Error::InvalidFormat("section slice out of bounds"))
    }

    fn read_dictionary_view(
        &self,
        section_id: u32,
    ) -> Result<crate::binary::StringDictionaryView<'a>, crate::binary::Error> {
        let section = self.required_section(section_id)?;
        crate::binary::StringDictionaryView::new(
            self.section_bytes(section)?,
            section.item_count,
        )
    }

    fn read_u32_vector_view(
        &self,
        section_id: u32,
        count: usize,
    ) -> Result<crate::binary::U32SliceView<'a>, crate::binary::Error> {
        use crate::binary::Error;

        let section = self.required_section(section_id)?;
        if section.item_count as usize != count {
            return Err(Error::InvalidFormat("u32 section item_count mismatch"));
        }

        let payload = self.section_bytes(section)?;
        if payload.len() != count * 4 {
            return Err(Error::InvalidFormat("u32 section byte length mismatch"));
        }

        crate::binary::U32SliceView::new(payload)
    }

    fn read_i64_vector_view(
        &self,
        section_id: u32,
        count: usize,
    ) -> Result<crate::binary::I64SliceView<'a>, crate::binary::Error> {
        use crate::binary::Error;

        let section = self.required_section(section_id)?;
        if section.item_count as usize != count {
            return Err(Error::InvalidFormat("i64 section item_count mismatch"));
        }

        let payload = self.section_bytes(section)?;
        if payload.len() != count * 8 {
            return Err(Error::InvalidFormat("i64 section byte length mismatch"));
        }

        crate::binary::I64SliceView::new(payload)
    }

    fn read_bool_vector_view(
        &self,
        section_id: u32,
        count: usize,
    ) -> Result<crate::binary::BoolSliceView<'a>, crate::binary::Error> {
        use crate::binary::Error;

        let section = self.required_section(section_id)?;
        if section.item_count as usize != count {
            return Err(Error::InvalidFormat("bool section item_count mismatch"));
        }

        let payload = self.section_bytes(section)?;
        if payload.len() != count {
            return Err(Error::InvalidFormat("bool section byte length mismatch"));
        }

        crate::binary::BoolSliceView::new(payload)
    }

    fn read_u32_list_column_view(
        &self,
        section_id: u32,
        record_count: usize,
        max_id: usize,
        name: &'static str,
    ) -> Result<crate::binary::U32ListColumnView<'a>, crate::binary::Error> {
        use crate::binary::Error;

        let section = self.required_section(section_id)?;
        if section.item_count as usize != record_count {
            return Err(Error::InvalidFormat("list column item_count mismatch"));
        }

        crate::binary::U32ListColumnView::new(
            self.section_bytes(section)?,
            record_count,
            max_id,
            name,
        )
    }

    fn read_dense_postings_view(
        &self,
        section_id: u32,
        term_count: usize,
    ) -> Result<crate::binary::DensePostingsView<'a>, crate::binary::Error> {
        use crate::binary::Error;

        let section = self.required_section(section_id)?;
        if section.item_count as usize != term_count {
            return Err(Error::InvalidFormat("postings item_count mismatch"));
        }

        crate::binary::DensePostingsView::new(
            self.section_bytes(section)?,
            term_count,
            self.header.record_count,
        )
    }

    fn read_bool_postings_view(
        &self,
        section_id: u32,
    ) -> Result<crate::binary::BoolPostingsView<'a>, crate::binary::Error> {
        let section = self.required_section(section_id)?;
        crate::binary::BoolPostingsView::new(
            self.section_bytes(section)?,
            section.item_count,
            self.header.record_count,
        )
    }
}

fn validate_ids_in_range(
    values: &crate::binary::U32SliceView<'_>,
    max_id: usize,
    name: &'static str,
) -> Result<(), crate::binary::Error> {
    if values.iter().all(|id| (id as usize) < max_id) {
        Ok(())
    } else {
        Err(crate::binary::Error::InvalidFormat(name))
    }
}
