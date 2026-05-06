#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoadedArtist {
    pub(crate) artist_id: String,
    pub(crate) channel_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoadedOfficialChannel {
    pub(crate) channel_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoadedClipRecord {
    pub(crate) clip_uuid: String,
    pub(crate) video_id: String,
    pub(crate) published_at: i64,
    pub(crate) channel_id: String,
    pub(crate) is_unlisted: bool,
    pub(crate) embeddable: bool,
    pub(crate) artist_ids: Vec<String>,
    pub(crate) tag_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoadedData {
    pub(crate) artists: Vec<LoadedArtist>,
    pub(crate) official_channels: Vec<LoadedOfficialChannel>,
    pub(crate) tag_ids: Vec<String>,
    pub(crate) clips: Vec<LoadedClipRecord>,
}

pub(crate) fn load_data(music_root: &std::path::Path) -> anyhow::Result<LoadedData> {
    let artists = artistctl::model::LOADED_LIVER_DATA
        .clone()
        .into_iter()
        .map(|(artist_id, artist)| {
            let artist = artist.into_inner();
            LoadedArtist {
                artist_id: artist_id.as_str().to_string(),
                channel_id: artist.channel_id.to_string(),
            }
        })
        .collect::<Vec<_>>();

    let official_channels = artistctl::model::LOADED_OFFICIAL_CHANNEL_DATA
        .clone()
        .into_iter()
        .map(|(_, channel)| {
            let channel = channel.into_inner();
            LoadedOfficialChannel {
                channel_id: channel.channel_id.to_string(),
            }
        })
        .collect::<Vec<_>>();

    let tag_ids = tagctl::model::LOADED_VIDEO_TAG_DATA
        .sorted_ids()
        .into_iter()
        .map(str::to_owned)
        .collect::<Vec<_>>();

    let library = musictl::music_file::MusicLibrary::load(music_root)?;
    let videos = library.into_videos()?;
    let mut clips = Vec::new();

    for video in videos.into_sorted_vec() {
        let mut video_tag_ids = video
            .video_tag_ids()
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>();
        video_tag_ids.sort_unstable();

        for clip in video.clips() {
            let mut tag_ids = video_tag_ids.clone();
            tag_ids.extend(clip.tag_ids().into_iter().map(str::to_owned));
            tag_ids.sort_unstable();
            tag_ids.dedup();

            clips.push(LoadedClipRecord {
                clip_uuid: clip.uuid_string(),
                video_id: video.video_id_string(),
                published_at: video.published_at_secs(),
                channel_id: video.channel_id_string(),
                is_unlisted: video.is_unlisted(),
                embeddable: video.embeddable(),
                artist_ids: clip
                    .artist_ids()
                    .into_iter()
                    .map(str::to_owned)
                    .collect::<Vec<_>>(),
                tag_ids,
            });
        }
    }

    Ok(LoadedData {
        artists,
        official_channels,
        tag_ids,
        clips,
    })
}
