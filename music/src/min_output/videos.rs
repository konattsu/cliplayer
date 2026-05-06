#[derive(serde::Serialize)]
pub(super) struct FlatVideos<'a>(
    std::collections::HashMap<&'a crate::model::VideoId, FlatVideoValue<'a>>,
);

struct FlatVideoValue<'a> {
    clip_uuids: Vec<&'a crate::model::UuidVer4>,
    title: &'a str,
    channel_id: &'a crate::model::ChannelId,
    published_at: &'a crate::model::VideoPublishedAt,
    synced_at: &'a chrono::DateTime<chrono::Utc>,
    duration: &'a crate::model::Duration,
    privacy_status: &'a crate::model::PrivacyStatus,
    embeddable: bool,
    uploader_name: Option<&'a crate::model::UploaderName>,
    video_tags: &'a crate::model::VideoTagIds,
}

impl serde::Serialize for FlatVideoValue<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct RawFlatVideoValue<'a> {
            clip_uuids: &'a Vec<&'a crate::model::UuidVer4>,
            title: &'a str,
            channel_id: &'a crate::model::ChannelId,
            published_at: &'a crate::model::VideoPublishedAt,
            #[serde(with = "crate::util::datetime_serde")]
            synced_at: &'a chrono::DateTime<chrono::Utc>,
            duration_secs: u32,
            privacy_status: &'a crate::model::PrivacyStatus,
            embeddable: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            uploader_name: Option<&'a crate::model::UploaderName>,
            #[serde(default)]
            video_tags: &'a crate::model::VideoTagIds,
        }

        RawFlatVideoValue {
            clip_uuids: &self.clip_uuids,
            title: self.title,
            channel_id: self.channel_id,
            published_at: self.published_at,
            synced_at: self.synced_at,
            duration_secs: self.duration.as_secs(),
            privacy_status: self.privacy_status,
            embeddable: self.embeddable,
            uploader_name: self.uploader_name,
            video_tags: self.video_tags,
        }
        .serialize(serializer)
    }
}

impl<'a> FlatVideos<'a> {
    pub(super) fn from_library(
        library: &'a crate::music_file::MusicLibrary,
    ) -> Result<Self, crate::music_file::MusicFileError> {
        let mut flat_videos = std::collections::HashMap::new();

        for file in library.iter_files() {
            for video in file.videos().iter() {
                let video_id = video.get_video_id();
                if flat_videos
                    .insert(
                        video_id,
                        FlatVideoValue {
                            clip_uuids: video
                                .clips()
                                .map(|clip| clip.get_uuid())
                                .collect(),
                            title: video.get_title(),
                            channel_id: video.get_channel_id(),
                            published_at: video.get_published_at(),
                            synced_at: video.get_synced_at(),
                            duration: video.get_duration(),
                            privacy_status: video.get_privacy_status(),
                            embeddable: video.is_embeddable(),
                            uploader_name: video.get_uploader_name(),
                            video_tags: video.get_video_tags(),
                        },
                    )
                    .is_some()
                {
                    return Err(
                        crate::music_file::MusicFileError::InvalidDatabaseContent {
                            msg: format!(
                                "duplicated video entry detected while building min videos output: {video_id}"
                            ),
                        },
                    );
                }
            }
        }

        Ok(Self(flat_videos))
    }
}
