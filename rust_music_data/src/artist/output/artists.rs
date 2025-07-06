#[derive(serde::Serialize, Debug, Clone)]
pub struct OutputArtists(
    std::collections::HashMap<crate::artist::model::ArtistId, OutputArtist>,
);

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct OutputArtist {
    ja: crate::artist::model::StringNonEmpty,
    jah: crate::artist::model::StringNonEmpty,
    en: crate::artist::model::StringNonEmpty,
    color: crate::artist::model::Color,
    #[serde(skip_serializing_if = "is_false")]
    is_graduated: bool,
}

fn is_false(value: &bool) -> bool {
    !*value
}

impl OutputArtists {
    pub fn new(artists: crate::artist::model::Artists) -> Self {
        let mut map = std::collections::HashMap::new();
        for (artist_id, artist) in artists.0.into_iter() {
            let output_artist = OutputArtist {
                ja: artist.ja,
                jah: artist.jah,
                en: artist.en,
                color: artist.color,
                is_graduated: artist.is_graduated,
            };
            map.insert(artist_id, output_artist);
        }
        Self(map)
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
    use crate::artist::model::{ArtistId, Color, StringNonEmpty};
    use std::collections::HashMap;

    #[test]
    fn test_output_artists_new() {
        use std::str::FromStr;

        let mut artists_map = HashMap::new();
        let artist_id = ArtistId::new("test-artist".into()).unwrap();
        let artist = crate::artist::model::artist_data::Artist {
            ja: StringNonEmpty::new("テストアーティスト".to_string()).unwrap(),
            jah: StringNonEmpty::new("てすとあーてぃすと".to_string()).unwrap(),
            en: StringNonEmpty::new("Test Artist".to_string()).unwrap(),
            color: Color::from_str("FFFFFF").unwrap(),
            aliases: vec![],
            channel_id: crate::model::ChannelId::test_id_1(),
            is_graduated: false,
            artist_id: artist_id.clone(),
        };
        artists_map.insert(artist_id.clone(), artist);
        let artists = crate::artist::model::Artists(artists_map);
        let output_artists = OutputArtists::new(artists);
        assert!(output_artists.0.contains_key(&artist_id));
    }
}
