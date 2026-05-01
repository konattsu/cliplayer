#[derive(Debug, Clone)]
pub struct SearchSourceClipRecord {
    pub clip_uuid: String,
    pub video_id: String,
    pub published_at: i64,
    pub channel_id: String,
    pub is_unlisted: bool,
    pub embeddable: bool,
    pub artist_ids: Vec<String>,
    pub tag_ids: Vec<String>,
}

pub fn load_clip_records(
    music_root: &std::path::Path,
) -> anyhow::Result<Vec<SearchSourceClipRecord>> {
    let library = crate::music_file::MusicLibrary::load(music_root)
        .map_err(|err| anyhow::anyhow!(err.to_string()))?;
    let videos = library
        .into_videos()
        .map_err(|err| anyhow::anyhow!(err.to_string()))?;

    let mut records = Vec::<SearchSourceClipRecord>::new();
    for video in videos.into_sorted_vec() {
        let is_unlisted = match video.get_privacy_status() {
            crate::model::PrivacyStatus::Public => false,
            crate::model::PrivacyStatus::Unlisted => true,
            crate::model::PrivacyStatus::Private => continue,
        };

        let video_tag_ids = video
            .get_video_tags()
            .to_vec()
            .into_iter()
            .map(str::to_string)
            .collect::<Vec<_>>();

        for clip in video.to_clips() {
            let mut tag_ids = video_tag_ids.clone();
            if let Some(clip_tags) = clip.get_clip_tags() {
                tag_ids.extend(
                    clip_tags
                        .to_vec()
                        .into_iter()
                        .map(str::to_string)
                        .collect::<Vec<_>>(),
                );
            }
            tag_ids.sort_unstable();
            tag_ids.dedup();

            let mut artist_ids = clip
                .get_liver_ids()
                .to_vec()
                .into_iter()
                .map(str::to_string)
                .collect::<Vec<_>>();
            artist_ids.sort_unstable();
            artist_ids.dedup();

            records.push(SearchSourceClipRecord {
                clip_uuid: clip.get_uuid().to_string(),
                video_id: video.get_video_id().as_str().to_string(),
                published_at: video.get_published_at().as_secs() as i64,
                channel_id: video.get_channel_id().to_string(),
                is_unlisted,
                embeddable: video.is_embeddable(),
                artist_ids,
                tag_ids,
            });
        }
    }

    records.sort_by(|left, right| left.clip_uuid.cmp(&right.clip_uuid));
    Ok(records)
}
