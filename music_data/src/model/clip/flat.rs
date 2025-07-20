// ! デシリアライザは付与しない, 整合性が綺麗に取れないため(動画情報が無くVerifiedClipにできないため)

/// クリップ情報をフラット化した物をまとめたもの
///
/// min化に必要な情報のみをまとめている
#[derive(Debug, Clone)]
pub(crate) struct FlatClips {
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
    // ソートしてからシリアライズ
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let clips = self.sorted_clips();
        clips.serialize(serializer)
    }
}

impl FlatClips {
    pub(crate) fn new(clips: Vec<crate::model::VerifiedClip>) -> Self {
        let clips = clips
            .into_iter()
            .map(|clip| (clip.get_uuid().clone(), FlatClip::from_verified_clip(clip)))
            .collect();

        Self { clips }
    }

    fn sorted_clips(&self) -> Vec<&FlatClip> {
        let mut clips: Vec<&FlatClip> = self.clips.values().collect();
        clips.sort_by(|a, b| a.uuid.cmp(&b.uuid));
        clips
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

// MARK: Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flat_clips_sorted_clips() {
        // 新しい
        let clip1 = FlatClip {
            uuid: crate::model::UuidVer7::self_1(),
            song_title: "Song A".to_string(),
            artists: crate::model::InternalArtists::test_name_1(),
            external_artists: None,
            clips_tags: None,
            start_time: crate::model::Duration::from_secs(0),
            end_time: crate::model::Duration::from_secs(10),
        };
        // 古い
        let clip2 = FlatClip {
            uuid: crate::model::UuidVer7::self_2(),
            song_title: "Song B".to_string(),
            artists: crate::model::InternalArtists::test_name_2(),
            external_artists: None,
            clips_tags: None,
            start_time: crate::model::Duration::from_secs(5),
            end_time: crate::model::Duration::from_secs(15),
        };

        let flat_clips = FlatClips {
            clips: vec![
                (clip1.uuid.clone(), clip1.clone()),
                (clip2.uuid.clone(), clip2.clone()),
            ]
            .into_iter()
            .collect(),
        };

        let sorted_clips = flat_clips.sorted_clips();
        assert_eq!(sorted_clips.len(), 2);
        assert_eq!(sorted_clips[0].uuid, clip2.uuid);
        assert_eq!(sorted_clips[1].uuid, clip1.uuid);
    }
}
