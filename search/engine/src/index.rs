#[derive(Debug, Clone)]
pub(crate) struct DictionaryCaches {
    pub(crate) channels: std::collections::HashMap<
        std::sync::Arc<str>,
        index_core::schema::ids::ChannelId,
    >,
    pub(crate) artists: std::collections::HashMap<
        std::sync::Arc<str>,
        index_core::schema::ids::ArtistId,
    >,
    pub(crate) tags:
        std::collections::HashMap<std::sync::Arc<str>, index_core::schema::ids::TagId>,
}

#[derive(Debug, Clone)]
pub(crate) struct LoadedIndex {
    bytes: std::sync::Arc<[u8]>,
    layout: index_core::binary::ValidatedIndexLayout,
    pub(crate) record_count: u32,
    pub(crate) dataset_build_id: String,
    pub(crate) dictionaries: DictionaryCaches,
}

impl LoadedIndex {
    pub(crate) fn load(
        bytes: std::sync::Arc<[u8]>,
    ) -> Result<Self, crate::EngineError> {
        let layout = index_core::binary::SearchIndexReader::validate_layout(&bytes)?;
        let bytes_for_reader = bytes.clone();
        let reader = index_core::binary::SearchIndexReader::from_validated_layout(
            &bytes_for_reader,
            &layout,
        )?;
        let metadata = reader.metadata_view()?;

        Ok(Self {
            bytes,
            record_count: layout.header().record_count,
            dataset_build_id: metadata.dataset_build_id().to_string(),
            dictionaries: DictionaryCaches {
                channels: build_dictionary_cache(reader.channels_dictionary()?)?,
                artists: build_dictionary_cache(reader.artists_dictionary()?)?,
                tags: build_dictionary_cache(reader.tags_dictionary()?)?,
            },
            layout,
        })
    }

    pub(crate) fn reader(
        &self,
    ) -> Result<index_core::binary::SearchIndexReader<'_>, crate::EngineError> {
        Ok(
            index_core::binary::SearchIndexReader::from_validated_layout(
                &self.bytes,
                &self.layout,
            )?,
        )
    }
}

fn build_dictionary_cache<Id>(
    dictionary: index_core::binary::StringDictionaryView<'_>,
) -> Result<std::collections::HashMap<std::sync::Arc<str>, Id>, crate::EngineError>
where
    Id: From<u32> + Copy + std::cmp::Eq + std::hash::Hash,
{
    use std::collections::HashMap;
    use std::sync::Arc;

    let mut out = HashMap::with_capacity(dictionary.len());
    for (index, value) in dictionary.iter().enumerate() {
        let value = value?;
        out.insert(Arc::<str>::from(value), Id::from(index as u32));
    }
    Ok(out)
}
