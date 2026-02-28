#[derive(serde::Serialize, Debug, Clone)]
pub(crate) struct ArtistSearchIndex(Vec<ArtistSearchIndexInner>);

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ArtistSearchIndexInner {
    key: String,
    liver_id: crate::model::LiverId,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_alias: Option<bool>,
}

impl ArtistSearchIndex {
    pub(crate) fn new(livers: crate::model::Livers) -> Self {
        let mut index = Vec::new();

        for (liver_id, liver) in livers.into_iter() {
            let liver = liver.into_inner();

            Self::push_non_aliases(
                [liver.ja, liver.jah, liver.en],
                &mut index,
                &liver_id,
            );
            Self::push_aliases(liver.aliases, &mut index, &liver_id);
        }

        index.sort_by(|a, b| {
            a.key.cmp(&b.key).then_with(|| a.liver_id.cmp(&b.liver_id))
        });

        Self(index)
    }

    fn push_non_aliases(
        key: [String; 3],
        index: &mut Vec<ArtistSearchIndexInner>,
        liver_id: &crate::model::LiverId,
    ) {
        for string_non_empty in key {
            index.push(ArtistSearchIndexInner {
                key: string_non_empty.clone(),
                liver_id: liver_id.clone(),
                is_alias: None,
            });
        }
    }

    fn push_aliases(
        keys: Vec<String>,
        index: &mut Vec<ArtistSearchIndexInner>,
        liver_id: &crate::model::LiverId,
    ) {
        for key in keys {
            index.push(ArtistSearchIndexInner {
                key,
                liver_id: liver_id.clone(),
                is_alias: Some(true),
            });
        }
    }

    pub(crate) fn output_as_json(&self, path: &std::path::Path) -> anyhow::Result<()> {
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
        let liver_id = crate::model::LiverId::self_1();
        let keys = [
            "Test Artist".to_string(),
            "てすとあーてぃすと".to_string(),
            "Test Artist (JPN)".to_string(),
        ];

        ArtistSearchIndex::push_non_aliases(keys, &mut index, &liver_id);

        assert_eq!(index.len(), 3);
        for entry in &index {
            assert_eq!(entry.liver_id, liver_id);
            assert!(entry.is_alias.is_none());
        }
    }

    #[test]
    fn test_artist_search_index_push_aliases() {
        let mut index = Vec::new();
        let liver_id = crate::model::LiverId::self_2();
        let aliases = vec!["Alias 1".to_string(), "Alias 2".to_string()];

        ArtistSearchIndex::push_aliases(aliases, &mut index, &liver_id);

        assert_eq!(index.len(), 2);
        for entry in &index {
            assert_eq!(entry.liver_id, liver_id);
            assert!(entry.is_alias.is_some());
        }
    }
}
