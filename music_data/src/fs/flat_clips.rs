// ! デシリアライザは付与しない, 整合性が綺麗に取れないため(動画情報が無くVerifiedClipにできないため)

#[derive(Debug, Clone)]
pub struct FlatClips {
    clips: std::collections::HashMap<crate::model::UuidVer7, FlatClip>,
}

#[derive(Debug, Clone, serde::Serialize)]
/// クリップ情報をフラット化した物
struct FlatClip {
    uuid: crate::model::UuidVer7,
    song_title: String,
    artists: crate::model::InternalArtists,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_artists: Option<crate::model::ExternalArtists>,
    #[serde(skip_serializing_if = "Option::is_none")]
    clips_tags: Option<crate::model::ClipTags>,
    start_time: crate::model::Duration,
    end_time: crate::model::Duration,
}

impl serde::Serialize for FlatClips {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut clips: Vec<&FlatClip> = self.clips.values().collect();
        clips.sort_by(|a, b| a.uuid.cmp(&b.uuid));
        clips.serialize(serializer)
    }
}

impl FlatClips {
    pub fn new(clips: Vec<crate::model::VerifiedClip>) -> Self {
        let clips = clips
            .into_iter()
            .map(|clip| (clip.get_uuid().clone(), FlatClip::from_verified_clip(clip)))
            .collect();

        Self { clips }
    }

    pub fn output_json(&self, path: &std::path::Path) -> anyhow::Result<()> {
        use anyhow::Context;

        let file = std::fs::File::create(path).with_context(|| {
            format!("Failed to create/truncate file at {}", path.display())
        })?;
        serde_json::to_writer_pretty(file, self).with_context(|| {
            format!(
                "Failed to serialize FlatClips to JSON at {}",
                path.display()
            )
        })?;
        Ok(())
    }
}

impl FlatClip {
    fn from_verified_clip(clip: crate::model::VerifiedClip) -> Self {
        let clip = clip.into_inner();

        Self {
            uuid: clip.uuid,
            song_title: clip.song_title,
            artists: clip.artists,
            external_artists: clip.external_artists,
            clips_tags: clip.clip_tags,
            start_time: clip.start_time,
            end_time: clip.end_time,
        }
    }
}
