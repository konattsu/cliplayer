#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NormalizedClipRecord {
    pub(crate) doc_id: index_core::schema::ids::DocId,
    pub(crate) clip_id: index_core::schema::ids::ClipId,
    pub(crate) video_id: index_core::schema::ids::VideoId,
    pub(crate) published_at: index_core::schema::TimestampSecs,
    pub(crate) channel_id: index_core::schema::ids::ChannelId,
    pub(crate) is_unlisted: bool,
    pub(crate) embeddable: bool,
    pub(crate) artist_ids: Vec<index_core::schema::ids::ArtistId>,
    pub(crate) tag_ids: Vec<index_core::schema::ids::TagId>,
}

pub(crate) fn normalize_clip_records(
    data: &crate::build::load::LoadedData,
    dictionaries: &index_core::schema::Dictionaries,
) -> anyhow::Result<Vec<NormalizedClipRecord>> {
    let mut clips = data.clips.clone();
    clips.sort_by(|left, right| left.clip_uuid.cmp(&right.clip_uuid));

    clips
        .into_iter()
        .enumerate()
        .map(|(doc_id, clip)| normalize_clip_record(dictionaries, doc_id, clip))
        .collect()
}

fn normalize_clip_record(
    dictionaries: &index_core::schema::Dictionaries,
    doc_id: usize,
    clip: crate::build::load::LoadedClipRecord,
) -> anyhow::Result<NormalizedClipRecord> {
    let mut artist_ids = clip
        .artist_ids
        .iter()
        .map(|artist_id| {
            dictionaries.artists.get_by_str(artist_id).ok_or_else(|| {
                anyhow::anyhow!(
                    "clip {} refers to unknown artist_id {artist_id}",
                    clip.clip_uuid
                )
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    artist_ids.sort_unstable();
    artist_ids.dedup();

    let mut tag_ids = clip
        .tag_ids
        .iter()
        .map(|tag_id| {
            dictionaries.tags.get_by_str(tag_id).ok_or_else(|| {
                anyhow::anyhow!(
                    "clip {} refers to unknown tag_id {tag_id}",
                    clip.clip_uuid
                )
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    tag_ids.sort_unstable();
    tag_ids.dedup();

    Ok(NormalizedClipRecord {
        doc_id: u32::try_from(doc_id).expect("doc_id fits within u32"),
        clip_id: dictionaries
            .clips
            .get_by_str(&clip.clip_uuid)
            .ok_or_else(|| {
                anyhow::anyhow!("missing clip id mapping for {}", clip.clip_uuid)
            })?,
        video_id: dictionaries
            .videos
            .get_by_str(&clip.video_id)
            .ok_or_else(|| {
                anyhow::anyhow!("missing video id mapping for {}", clip.video_id)
            })?,
        published_at: index_core::schema::TimestampSecs::from(clip.published_at),
        channel_id: dictionaries
            .channels
            .get_by_str(&clip.channel_id)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "clip {} refers to unknown channel_id {}",
                    clip.clip_uuid,
                    clip.channel_id
                )
            })?,
        is_unlisted: clip.is_unlisted,
        embeddable: clip.embeddable,
        artist_ids,
        tag_ids,
    })
}
