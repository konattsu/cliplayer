// ! デシリアライザは付与しない, 整合性が綺麗に取れないため(動画情報が無くVerifiedClipにできないため)

/// クリップ情報をフラット化した物をまとめたもの
///
/// min化に必要な情報のみをまとめている
#[derive(Debug, Clone)]
pub(crate) struct FlatClips {
    clips: std::collections::HashMap<crate::model::UuidVer4, FlatClip>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
/// クリップ情報をフラット化した物
struct FlatClip {
    uuid: crate::model::UuidVer4,
    song_title: String,
    artists: crate::model::InternalArtists,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_artists: Option<crate::model::ExternalArtists>,
    #[serde(skip_serializing_if = "Option::is_none")]
    clips_tags: Option<crate::model::ClipTags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    clipped_video_id: Option<crate::model::VideoId>,
    start_time: crate::model::Duration,
    end_time: crate::model::Duration,
}

impl serde::Serialize for FlatClips {
    // ソートしてからシリアライズ. 一意にしたいだけで何の値をキーにしてもいい
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let clips = self.sorted_clips();
        clips.serialize(serializer)
    }
}

impl FlatClips {
    pub(crate) fn from_verified_videos(videos: crate::model::VerifiedVideos) -> Self {
        let clips = videos
            .into_sorted_vec()
            .into_iter()
            .flat_map(|video| (video.into_clips()))
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
            clipped_video_id: clip.clipped_video_id,
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
        // uuidが前
        let clip1 = FlatClip {
            uuid: crate::model::UuidVer4::self_1(),
            song_title: "Song A".to_string(),
            artists: crate::model::InternalArtists::self_1(),
            external_artists: None,
            clips_tags: None,
            clipped_video_id: None,
            start_time: crate::model::Duration::from_secs_u16(0),
            end_time: crate::model::Duration::from_secs_u16(10),
        };
        // uuidが後
        let clip2 = FlatClip {
            uuid: crate::model::UuidVer4::self_2(),
            song_title: "Song B".to_string(),
            artists: crate::model::InternalArtists::self_2(),
            external_artists: None,
            clips_tags: None,
            clipped_video_id: None,
            start_time: crate::model::Duration::from_secs_u16(5),
            end_time: crate::model::Duration::from_secs_u16(15),
        };

        let flat_clips = FlatClips {
            clips: vec![
                // 順番逆に
                (clip2.uuid.clone(), clip2.clone()),
                (clip1.uuid.clone(), clip1.clone()),
            ]
            .into_iter()
            .collect(),
        };

        let sorted_clips = flat_clips.sorted_clips();
        assert_eq!(sorted_clips.len(), 2);
        // ソート確認
        assert_eq!(sorted_clips[0].uuid, clip1.uuid);
        assert_eq!(sorted_clips[1].uuid, clip2.uuid);
    }
}
