#[derive(serde::Serialize, Debug, Clone)]
pub struct ArtistSearchIndex(Vec<ArtistSearchIndexInner>);

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ArtistSearchIndexInner {
    key: crate::artist::model::StringNonEmpty,
    artist_id: crate::artist::model::ArtistId,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_alias: Option<bool>,
}

impl ArtistSearchIndex {
    pub fn new(artists: crate::artist::model::Artists) -> Self {
        let mut index = Vec::new();

        for (artist_id, artist) in artists.0.into_iter() {
            Self::push_non_aliases(
                [artist.ja, artist.jah, artist.en],
                &mut index,
                &artist_id,
            );
            Self::push_aliases(artist.aliases, &mut index, &artist_id);
        }

        index.sort_by(|a, b| {
            a.key
                .cmp(&b.key)
                .then_with(|| a.artist_id.cmp(&b.artist_id))
        });

        Self(index)
    }

    fn push_non_aliases(
        string_non_empties: [crate::artist::model::StringNonEmpty; 3],
        index: &mut Vec<ArtistSearchIndexInner>,
        artist_id: &crate::artist::model::ArtistId,
    ) {
        for string_non_empty in string_non_empties {
            index.push(ArtistSearchIndexInner {
                key: string_non_empty.clone(),
                artist_id: artist_id.clone(),
                is_alias: None,
            });
        }
    }

    fn push_aliases(
        keys: Vec<crate::artist::model::StringNonEmpty>,
        index: &mut Vec<ArtistSearchIndexInner>,
        artist_id: &crate::artist::model::ArtistId,
    ) {
        for key in keys {
            index.push(ArtistSearchIndexInner {
                key,
                artist_id: artist_id.clone(),
                is_alias: Some(true),
            });
        }
    }

    pub fn output_json(&self, path: &std::path::Path) -> anyhow::Result<()> {
        use anyhow::Context;

        let file = std::fs::File::create(path).with_context(|| {
            format!("Failed to create/truncate file at {}", path.display())
        })?;
        serde_json::to_writer(file, self).with_context(|| {
            format!("Failed to write JSON to file: {}", path.display())
        })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_artist_search_index_push_non_aliases() {
        let mut index = Vec::new();
        let artist_id =
            crate::artist::model::ArtistId::new("test-artist".into()).unwrap();
        let string_non_empties = [
            crate::artist::model::StringNonEmpty::new("Test Artist".to_string())
                .unwrap(),
            crate::artist::model::StringNonEmpty::new("てすとあーてぃすと".to_string())
                .unwrap(),
            crate::artist::model::StringNonEmpty::new("Test Artist (JPN)".to_string())
                .unwrap(),
        ];

        ArtistSearchIndex::push_non_aliases(string_non_empties, &mut index, &artist_id);

        assert_eq!(index.len(), 3);
        for entry in &index {
            assert_eq!(entry.artist_id, artist_id);
            assert!(entry.is_alias.is_none());
        }
    }

    #[test]
    fn test_artist_search_index_push_aliases() {
        let mut index = Vec::new();
        let artist_id =
            crate::artist::model::ArtistId::new("test-artist".into()).unwrap();
        let aliases = vec![
            crate::artist::model::StringNonEmpty::new("Alias 1".to_string()).unwrap(),
            crate::artist::model::StringNonEmpty::new("Alias 2".to_string()).unwrap(),
        ];

        ArtistSearchIndex::push_aliases(aliases, &mut index, &artist_id);

        assert_eq!(index.len(), 2);
        for entry in &index {
            assert_eq!(entry.artist_id, artist_id);
            assert!(entry.is_alias.is_some());
        }
    }
}
