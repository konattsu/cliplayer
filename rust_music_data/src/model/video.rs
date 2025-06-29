/// 動画をfinalizeするときのエラー
#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum VideoFinalizationError {
    #[error(
        "the range of the clip is longer than the video duration: clips:({clips:?})"
    )]
    ClipLongerThanVideo {
        clips: Vec<crate::model::IdentifiedClip>,
    },
}

/// 動画の詳細情報とクリップの情報をまとめた構造体
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct FinalizedVideo {
    /// 動画の詳細情報
    #[serde(flatten)]
    video_details: VideoDetails,
    /// クリップの情報
    clips: Vec<crate::model::IdentifiedClip>,
}

impl FinalizedVideo {
    /// 動画の詳細情報とクリップの情報をまとめてfinalizeする
    ///
    /// - Error: `clips`の中に動画の長さを超えるクリップがある場合
    pub fn finalize_from_clips(
        video_details: VideoDetails,
        clips: Vec<crate::model::Clip>,
    ) -> Result<Self, VideoFinalizationError> {
        use crate::model::{Clip, IdentifiedClip};

        let identified_clips: Vec<IdentifiedClip> = clips
            .into_iter()
            .map(|clip| match clip {
                Clip::Identified(identified_clip) => identified_clip,
                Clip::Unidentified(unidentified_clip) => {
                    unidentified_clip.into_identified(video_details.get_published_at())
                }
            })
            .collect();

        Self::finalize_from_identified_clips(video_details, identified_clips)
    }

    pub fn finalize_from_unidentified_clips(
        video_details: VideoDetails,
        clips: Vec<crate::model::UnidentifiedClip>,
    ) -> Result<Self, VideoFinalizationError> {
        let identified_clips: Vec<crate::model::IdentifiedClip> = clips
            .into_iter()
            .map(|clip| clip.into_identified(video_details.get_published_at()))
            .collect();

        Self::finalize_from_identified_clips(video_details, identified_clips)
    }

    /// 動画の詳細情報と識別済みクリップの情報をまとめてfinalizeする
    pub fn finalize_from_identified_clips(
        video_details: VideoDetails,
        clips: Vec<crate::model::IdentifiedClip>,
    ) -> Result<Self, VideoFinalizationError> {
        let out_of_range_clips: Vec<crate::model::IdentifiedClip> = clips
            .iter()
            .filter(|clip| clip.get_end_time() > &video_details.duration)
            .cloned()
            .collect();

        if out_of_range_clips.is_empty() {
            Ok(Self {
                video_details,
                clips,
            })
        } else {
            Err(VideoFinalizationError::ClipLongerThanVideo {
                clips: out_of_range_clips,
            })
        }
    }
}

/// 動画の詳細情報
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct VideoDetails {
    /// 動画id
    video_id: crate::model::VideoId,
    /// 動画のタイトル
    title: String,
    /// 動画をアップロードしたチャンネルのid
    channel_id: crate::model::ChannelId,
    /// 動画の公開日時
    published_at: crate::model::VideoPublishedAt,
    /// クリップ情報が変更された日時
    #[serde(with = "crate::util::datetime_serde")]
    modified_at: chrono::DateTime<chrono::Utc>,
    /// 動画の長さ
    duration: crate::model::Duration,
    /// 動画の公開設定
    privacy_status: crate::model::PrivacyStatus,
    /// 埋め込み可能かどうか
    embeddable: bool,
    /// 動画全体に適用するタグ
    tags: crate::model::TagList,
}

impl VideoDetails {
    pub fn get_video_id(&self) -> &crate::model::VideoId {
        &self.video_id
    }
    pub fn get_title(&self) -> &String {
        &self.title
    }
    pub fn get_channel_id(&self) -> &crate::model::ChannelId {
        &self.channel_id
    }
    pub fn get_published_at(&self) -> &crate::model::VideoPublishedAt {
        &self.published_at
    }
    pub fn get_modified_at(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.modified_at
    }
    pub fn get_duration(&self) -> &crate::model::Duration {
        &self.duration
    }
    pub fn get_privacy_status(&self) -> &crate::model::PrivacyStatus {
        &self.privacy_status
    }
    pub fn is_embeddable(&self) -> bool {
        self.embeddable
    }
    pub fn get_tags(&self) -> &crate::model::TagList {
        &self.tags
    }
}

/// `VideoDetails`を初期化するための構造体
pub struct VideoDetailsInitializer {
    /// 動画id
    pub video_id: crate::model::VideoId,
    /// 動画のタイトル
    pub title: String,
    /// 動画をアップロードしたチャンネルのid
    pub channel_id: crate::model::ChannelId,
    /// 動画の公開日時
    pub published_at: crate::model::VideoPublishedAt,
    /// クリップ情報が変更された日時
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// 動画の長さ
    pub duration: crate::model::Duration,
    /// 動画の公開設定
    pub privacy_status: crate::model::PrivacyStatus,
    /// 埋め込み可能かどうか
    pub embeddable: bool,
    /// 動画全体に適用するタグ
    pub tags: crate::model::TagList,
}

impl VideoDetailsInitializer {
    pub fn init(self) -> VideoDetails {
        VideoDetails {
            video_id: self.video_id,
            title: self.title,
            channel_id: self.channel_id,
            published_at: self.published_at,
            modified_at: self.modified_at,
            duration: self.duration,
            privacy_status: self.privacy_status,
            embeddable: self.embeddable,
            tags: self.tags,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DraftVideo {
    /// 動画id
    video_id: crate::model::VideoId,
    /// 動画全体に適用するタグ
    tags: crate::model::TagList,
    /// クリップの情報
    clips: Vec<crate::model::UnidentifiedClip>,
}

impl DraftVideo {
    pub fn get_video_id(&self) -> &crate::model::VideoId {
        &self.video_id
    }
    pub fn get_tags(&self) -> &crate::model::TagList {
        &self.tags
    }
    pub fn get_clips(&self) -> &Vec<crate::model::UnidentifiedClip> {
        &self.clips
    }

    pub fn into_video_id(self) -> crate::model::VideoId {
        self.video_id
    }
    pub fn into_unidentified(self) -> Vec<crate::model::UnidentifiedClip> {
        self.clips
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_finalized_video_finalize() {
//         use chrono::TimeZone;

//         let video_details = VideoDetailsInitializer {
//             video_id: crate::model::VideoId::test_id_1(),
//             title: "Test Video".to_string(),
//             channel_id: crate::model::ChannelId::test_id_1(),
//             published_at: crate::model::VideoPublishedAt::new(
//                 chrono::Utc.with_ymd_and_hms(2024, 1, 1, 1, 1, 1).unwrap(),
//             )
//             .unwrap(),
//             modified_at: chrono::Utc.with_ymd_and_hms(2025, 1, 1, 1, 1, 1).unwrap(),
//             duration: crate::model::Duration::from_secs(300),
//             privacy_status: crate::model::PrivateStatus::Public,
//             embeddable: true,
//             tags: crate::model::TagList::from_vec_str(vec!["tag1", "tag2"]),
//         }
//         .init();
//     }
// }
