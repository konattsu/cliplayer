// なぜか相対パスしか使えない
// TODO public以外に配置することにした. また,シリアライズ時asc順に修正したのでなってるはず. cliに入れてあげる
const INPUT_ARTIST_DATA_FULL_PATH: &str = "data/artists_data.json";
const ARTIST_OUTPUT_DIR: &str = "../public/music_data/";
const ARTIST_SEARCH_INDEX_PATH: &str = "artist_search_index.min.json";
const CHANNELS_PATH: &str = "channels.min.json";
const ARTISTS_PATH: &str = "artists.min.json";

pub fn generate() -> anyhow::Result<()> {
    let artist_data: crate::artist::model::Artists =
        crate::artist::model::Artists::load(INPUT_ARTIST_DATA_FULL_PATH)?;

    generate_artist_search_index(artist_data.clone())?;
    generate_channels(&artist_data)?;
    generate_artists(artist_data)?;
    Ok(())
}

fn generate_artist_search_index(
    artist_data: crate::artist::model::Artists,
) -> anyhow::Result<()> {
    let output_artists = crate::artist::output::ArtistSearchIndex::new(artist_data);
    output_artists.output_json(std::path::Path::new(&format!(
        "{ARTIST_OUTPUT_DIR}{ARTIST_SEARCH_INDEX_PATH}",
    )))?;
    Ok(())
}

fn generate_channels(
    artist_data: &crate::artist::model::Artists,
) -> anyhow::Result<()> {
    let channels = crate::artist::output::Channels::new(artist_data);
    channels.output_json(std::path::Path::new(&format!(
        "{ARTIST_OUTPUT_DIR}{CHANNELS_PATH}",
    )))?;
    Ok(())
}

fn generate_artists(artist_data: crate::artist::model::Artists) -> anyhow::Result<()> {
    let output_artists = crate::artist::output::OutputArtists::new(artist_data);
    output_artists.output_json(std::path::Path::new(&format!(
        "{ARTIST_OUTPUT_DIR}{ARTISTS_PATH}",
    )))?;
    Ok(())
}
