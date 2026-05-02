pub(crate) fn build_dictionaries(
    data: &crate::build::load::LoadedData,
) -> index_core::schema::Dictionaries {
    use index_core::util::BiMap;
    use std::collections::BTreeSet;

    let mut clip_keys = BTreeSet::new();
    let mut video_keys = BTreeSet::new();
    let mut channel_keys = BTreeSet::new();
    let mut artist_keys = BTreeSet::new();
    let mut tag_keys = BTreeSet::new();

    for artist in &data.artists {
        artist_keys.insert(artist.artist_id.clone());
        channel_keys.insert(artist.channel_id.clone());
    }
    for channel in &data.official_channels {
        channel_keys.insert(channel.channel_id.clone());
    }
    for tag_id in &data.tag_ids {
        tag_keys.insert(tag_id.clone());
    }
    for clip in &data.clips {
        clip_keys.insert(clip.clip_uuid.clone());
        video_keys.insert(clip.video_id.clone());
    }

    index_core::schema::Dictionaries {
        clips: BiMap::build(clip_keys),
        videos: BiMap::build(video_keys),
        channels: BiMap::build(channel_keys),
        artists: BiMap::build(artist_keys),
        tags: BiMap::build(tag_keys),
    }
}
