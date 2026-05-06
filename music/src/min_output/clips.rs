#[derive(serde::Serialize)]
pub(super) struct FlatClips<'a>(
    std::collections::HashMap<&'a crate::model::UuidVer4, FlatClipValue<'a>>,
);

struct FlatClipValue<'a> {
    song_title: &'a str,
    liver_ids: &'a artistctl::model::LiverIds,
    external_artists_name: Option<&'a artistctl::model::ExternalArtistsName>,
    clipped_video_id: Option<&'a crate::model::VideoId>,
    start_time: &'a crate::model::Duration,
    end_time: &'a crate::model::Duration,
    video_id: &'a crate::model::VideoId,
}

impl serde::Serialize for FlatClipValue<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct RawFlatClipValue<'a> {
            song_title: &'a str,
            liver_ids: &'a artistctl::model::LiverIds,
            #[serde(skip_serializing_if = "Option::is_none")]
            external_artists_name: Option<&'a artistctl::model::ExternalArtistsName>,
            #[serde(skip_serializing_if = "Option::is_none")]
            clipped_video_id: Option<&'a crate::model::VideoId>,
            start_time_secs: u32,
            end_time_secs: u32,
            video_id: &'a crate::model::VideoId,
        }

        RawFlatClipValue {
            song_title: self.song_title,
            liver_ids: self.liver_ids,
            external_artists_name: self.external_artists_name,
            clipped_video_id: self.clipped_video_id,
            start_time_secs: self.start_time.as_secs(),
            end_time_secs: self.end_time.as_secs(),
            video_id: self.video_id,
        }
        .serialize(serializer)
    }
}

impl<'a> FlatClips<'a> {
    pub(super) fn from_library(library: &'a crate::music_file::MusicLibrary) -> Self {
        let mut flat_clips = std::collections::HashMap::new();

        for file in library.iter_files() {
            for video in file.videos().iter() {
                let video_id = video.get_video_id();
                for clip in video.clips() {
                    flat_clips.insert(
                        clip.get_uuid(),
                        FlatClipValue {
                            song_title: clip.get_song_title(),
                            liver_ids: clip.get_liver_ids(),
                            external_artists_name: clip.get_external_artists_name(),
                            clipped_video_id: clip.get_clipped_video_id(),
                            start_time: clip.get_start_time(),
                            end_time: clip.get_end_time(),
                            video_id,
                        },
                    );
                }
            }
        }

        Self(flat_clips)
    }
}
