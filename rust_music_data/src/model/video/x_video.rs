// /// 動画をfinalizeするときのエラー
// #[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
// pub enum VideoFinalizationError {
//     #[error(
//         "the range of the clip is longer than the video duration: clips:({clips:?})"
//     )]
//     ClipLongerThanVideo {
//         clips: Vec<crate::model::IdentifiedClip>,
//     },
// }

// // TODO modifiedAt本当に必要か検討, git管理なら社さんこれいらなくないですか?

// /// 動画の詳細情報とクリップの情報をまとめた構造体
// #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
// pub struct FinalizedVideo {
//     /// 動画の詳細情報
//     #[serde(flatten)]
//     video_details: VideoDetails,
//     /// クリップの情報
//     clips: Vec<crate::model::IdentifiedClip>,
// }

// impl FinalizedVideo {
//     /// 動画の詳細情報とクリップの情報をまとめてfinalizeする
//     ///
//     /// - Error: `clips`の中に動画の長さを超えるクリップがある場合
//     pub fn finalize_from_clips(
//         video_details: VideoDetails,
//         clips: Vec<crate::model::Clip>,
//     ) -> Result<Self, VideoFinalizationError> {
//         use crate::model::{Clip, IdentifiedClip};

//         let identified_clips: Vec<IdentifiedClip> = clips
//             .into_iter()
//             .map(|clip| match clip {
//                 Clip::Identified(identified_clip) => identified_clip,
//                 Clip::Unidentified(unidentified_clip) => {
//                     unidentified_clip.into_identified(video_details.get_published_at())
//                 }
//             })
//             .collect();

//         Self::finalize_from_identified_clips(video_details, identified_clips)
//     }

//     /// 動画の詳細情報と識別されていないクリップの情報をまとめてfinalizeする
//     ///
//     /// - Error: `clips`の中に動画の長さを超えるクリップがある場合
//     pub fn finalize_from_unidentified_clips(
//         video_details: VideoDetails,
//         clips: Vec<crate::model::UnidentifiedClip>,
//     ) -> Result<Self, VideoFinalizationError> {
//         let identified_clips: Vec<crate::model::IdentifiedClip> = clips
//             .into_iter()
//             .map(|clip| clip.into_identified(video_details.get_published_at()))
//             .collect();

//         Self::finalize_from_identified_clips(video_details, identified_clips)
//     }

//     /// 動画の詳細情報と識別済みクリップの情報をまとめてfinalizeする
//     ///
//     /// - Error: `clips`の中に動画の長さを超えるクリップがある場合
//     pub fn finalize_from_identified_clips(
//         video_details: VideoDetails,
//         clips: Vec<crate::model::IdentifiedClip>,
//     ) -> Result<Self, VideoFinalizationError> {
//         let out_of_range_clips: Vec<crate::model::IdentifiedClip> = clips
//             .iter()
//             .filter(|clip| clip.get_end_time() > &video_details.duration)
//             .cloned()
//             .collect();

//         if out_of_range_clips.is_empty() {
//             Ok(Self {
//                 video_details,
//                 clips,
//             })
//         } else {
//             Err(VideoFinalizationError::ClipLongerThanVideo {
//                 clips: out_of_range_clips,
//             })
//         }
//     }
// }

// /// 動画の詳細情報
// #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
// pub struct VideoDetails {
//     /// 動画id
//     video_id: crate::model::VideoId,
//     /// 動画のタイトル
//     title: String,
//     /// 動画をアップロードしたチャンネルのid
//     channel_id: crate::model::ChannelId,
//     /// 動画をアップロードしたチャンネルの名前
//     channel_name: crate::model::ChannelName,
//     /// 動画の公開日時
//     published_at: crate::model::VideoPublishedAt,
//     /// クリップ情報が変更された日時
//     #[serde(with = "crate::util::datetime_serde")]
//     modified_at: chrono::DateTime<chrono::Utc>,
//     /// 動画の長さ
//     duration: crate::model::Duration,
//     /// 動画の公開設定
//     privacy_status: crate::model::PrivacyStatus,
//     /// 埋め込み可能かどうか
//     embeddable: bool,
//     /// 動画のタグ
//     tags: crate::model::TagList,
// }

// impl VideoDetails {
//     pub fn get_video_id(&self) -> &crate::model::VideoId {
//         &self.video_id
//     }
//     pub fn get_title(&self) -> &String {
//         &self.title
//     }
//     pub fn get_channel_id(&self) -> &crate::model::ChannelId {
//         &self.channel_id
//     }
//     pub fn get_channel_name(&self) -> &crate::model::ChannelName {
//         &self.channel_name
//     }
//     pub fn get_published_at(&self) -> &crate::model::VideoPublishedAt {
//         &self.published_at
//     }
//     pub fn get_modified_at(&self) -> &chrono::DateTime<chrono::Utc> {
//         &self.modified_at
//     }
//     pub fn get_duration(&self) -> &crate::model::Duration {
//         &self.duration
//     }
//     pub fn get_privacy_status(&self) -> &crate::model::PrivacyStatus {
//         &self.privacy_status
//     }
//     pub fn is_embeddable(&self) -> bool {
//         self.embeddable
//     }
//     pub fn get_tags(&self) -> &crate::model::TagList {
//         &self.tags
//     }
// }

// /// `VideoDetails`を初期化するための構造体
// pub struct VideoDetailsInitializer {
//     /// 動画id
//     pub video_id: crate::model::VideoId,
//     /// 動画のタイトル
//     pub title: String,
//     /// 動画をアップロードしたチャンネルのid
//     pub channel_id: crate::model::ChannelId,
//     /// 動画をアップロードしたチャンネルの名前
//     pub channel_name: crate::model::ChannelName,
//     /// 動画の公開日時
//     pub published_at: crate::model::VideoPublishedAt,
//     /// クリップ情報が変更された日時
//     pub modified_at: chrono::DateTime<chrono::Utc>,
//     /// 動画の長さ
//     pub duration: crate::model::Duration,
//     /// 動画の公開設定
//     pub privacy_status: crate::model::PrivacyStatus,
//     /// 埋め込み可能かどうか
//     pub embeddable: bool,
//     /// 動画のタグ
//     pub tags: crate::model::TagList,
// }

// impl VideoDetailsInitializer {
//     pub fn init(self) -> VideoDetails {
//         VideoDetails {
//             video_id: self.video_id,
//             title: self.title,
//             channel_id: self.channel_id,
//             channel_name: self.channel_name,
//             published_at: self.published_at,
//             modified_at: self.modified_at,
//             duration: self.duration,
//             privacy_status: self.privacy_status,
//             embeddable: self.embeddable,
//             tags: self.tags,
//         }
//     }
// }

// /// ドラフト段階での動画の情報を保持する構造体
// #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
// pub struct DraftVideo {
//     /// 動画id
//     video_id: crate::model::VideoId,
//     /// 動画のタグ
//     tags: crate::model::TagList,
//     /// クリップの情報
//     clips: Vec<crate::model::UnidentifiedClip>,
// }

// impl DraftVideo {
//     pub fn get_video_id(&self) -> &crate::model::VideoId {
//         &self.video_id
//     }
//     pub fn get_tags(&self) -> &crate::model::TagList {
//         &self.tags
//     }
//     pub fn get_clips(&self) -> &Vec<crate::model::UnidentifiedClip> {
//         &self.clips
//     }

//     pub fn into_video_id(self) -> crate::model::VideoId {
//         self.video_id
//     }
//     pub fn into_unidentified(self) -> Vec<crate::model::UnidentifiedClip> {
//         self.clips
//     }
// }

// // MARK: Tests

// #[cfg(test)]
// mod tests {
//     use super::*;

//     // MARK: -- Deserialization

//     const FINALIZED_VIDEO_JSON: &str = r#"
//     {
//         "videoId": "11111111111",
//         "title": "Test Video 1",
//         "channelId": "UC111111111111111111111111",
//         "channelName": "Test Channel Name 1",
//         "publishedAt": "2023-10-01T00:00:00Z",
//         "modifiedAt": "2023-10-01T11:11:11Z",
//         "duration": "PT300S",
//         "privacyStatus": "public",
//         "embeddable": true,
//         "tags": ["tag1", "tag2"],
//         "clips": [
//             {
//                 "song_title": "Test Clip",
//                 "artists": ["Aimer Test"],
//                 "is_clipped": true,
//                 "start_time": "PT10S",
//                 "end_time": "PT20S",
//                 "tags": ["tag1", "tag2"],
//                 "uuid": "0193bac8-a560-7000-8000-000000000000",
//                 "volume_percent": 50
//             }
//         ]
//     }"#;

//     #[test]
//     fn test_finalized_video_deserialization() {
//         let _video: FinalizedVideo = serde_json::from_str(FINALIZED_VIDEO_JSON)
//             .expect("Failed to deserialize FinalizedVideo");
//     }

//     const VIDEO_DETAILS_JSON: &str = r#"
//     {
//         "videoId": "11111111111",
//         "title": "Test Video 1",
//         "channelId": "UC111111111111111111111111",
//         "channelName": "Test Channel Name 1",
//         "publishedAt": "2023-10-01T00:00:00Z",
//         "modifiedAt": "2023-10-01T11:11:11Z",
//         "duration": "PT300S",
//         "privacyStatus": "public",
//         "embeddable": true,
//         "tags": ["tag1", "tag2"]
//     }"#;

//     #[test]
//     fn test_video_details_deserialization() {
//         let _video_details: VideoDetails = serde_json::from_str(VIDEO_DETAILS_JSON)
//             .expect("Failed to deserialize VideoDetails");
//     }

//     const DRAFT_VIDEO_JSON: &str = r#"
//     {
//         "videoId": "11111111111",
//         "tags": ["tag1", "tag2"],
//         "clips": [
//             {
//                 "song_title": "Test Clip",
//                 "artists": ["Aimer Test"],
//                 "is_clipped": true,
//                 "start_time": "PT10S",
//                 "end_time": "PT20S",
//                 "tags": ["tag3"]
//             }
//         ]
//     }"#;

//     #[test]
//     fn test_draft_video_deserialization() {
//         let _draft_video: DraftVideo = serde_json::from_str(DRAFT_VIDEO_JSON)
//             .expect("Failed to deserialize DraftVideo");
//     }

//     // MARK: -- Methods

//     /// 動画の長さ(duration_secs)のみ指定
//     fn sample_video_details(duration_secs: u16) -> VideoDetails {
//         use chrono::TimeZone;
//         let modified_at = chrono::Utc.with_ymd_and_hms(2024, 10, 1, 1, 1, 1).unwrap();
//         VideoDetailsInitializer {
//             video_id: crate::model::VideoId::test_id_1(),
//             title: "Test Video".to_string(),
//             channel_id: crate::model::ChannelId::test_id_1(),
//             channel_name: crate::model::ChannelName::test_channel_name_1(),
//             published_at: crate::model::VideoPublishedAt::self_1(),
//             modified_at,
//             duration: crate::model::Duration::from_secs(duration_secs),
//             privacy_status: crate::model::PrivacyStatus::Public,
//             embeddable: true,
//             tags: crate::model::TagList::test_tag_list_1(),
//         }
//         .init()
//     }

//     /// - Error: `start_sec` > `end_sec`
//     fn sample_identified_clip(
//         start_sec: u64,
//         end_sec: u64,
//     ) -> crate::model::IdentifiedClip {
//         crate::model::IdentifiedClipInitializer {
//             song_title: "Song Title".to_string(),
//             artists: vec!["Artist Name".to_string()],
//             external_artists: None,
//             is_clipped: false,
//             start_time: crate::model::Duration::from_secs(start_sec),
//             end_time: crate::model::Duration::from_secs(end_sec),
//             tags: None,
//             uuid: crate::model::UuidVer7::generate(datetime),
//             volume_percent: None,
//         }
//         .init()
//         .unwrap()
//     }

//     fn sample_unidentified_clip(start: u64, end: u64) -> UnidentifiedClip {
//         UnidentifiedClip::new(
//             "Song".to_string(),
//             vec!["Artist".to_string()],
//             true,
//             crate::model::Duration::from_std(std::time::Duration::from_secs(start))
//                 .unwrap(),
//             crate::model::Duration::from_std(std::time::Duration::from_secs(end))
//                 .unwrap(),
//             vec!["tag".into()],
//         )
//     }

//     #[test]
//     fn test_finalize_from_identified_clips_success() {
//         let video_details = sample_video_details(100);
//         let clips = vec![sample_identified_clip(10, 20)];
//         let result = FinalizedVideo::finalize_from_identified_clips(
//             video_details.clone(),
//             clips.clone(),
//         );
//         assert!(result.is_ok());
//         let finalized = result.unwrap();
//         assert_eq!(
//             finalized.video_details.get_video_id(),
//             video_details.get_video_id()
//         );
//         assert_eq!(finalized.clips, clips);
//     }

//     #[test]
//     fn test_finalize_from_identified_clips_error() {
//         let video_details = sample_video_details(15);
//         let clips = vec![sample_identified_clip(10, 20)];
//         let result =
//             FinalizedVideo::finalize_from_identified_clips(video_details, clips);
//         assert!(matches!(
//             result,
//             Err(VideoFinalizationError::ClipLongerThanVideo { .. })
//         ));
//     }

//     #[test]
//     fn test_finalize_from_unidentified_clips_success() {
//         let video_details = sample_video_details(100);
//         let clips = vec![sample_unidentified_clip(10, 20)];
//         let result = FinalizedVideo::finalize_from_unidentified_clips(
//             video_details.clone(),
//             clips.clone(),
//         );
//         assert!(result.is_ok());
//         let finalized = result.unwrap();
//         assert_eq!(
//             finalized.video_details.get_video_id(),
//             video_details.get_video_id()
//         );
//         assert_eq!(finalized.clips.len(), 1);
//         assert_eq!(
//             finalized.clips[0].get_start_time(),
//             &crate::model::Duration::from_std(std::time::Duration::from_secs(10))
//                 .unwrap()
//         );
//         assert_eq!(
//             finalized.clips[0].get_end_time(),
//             &crate::model::Duration::from_std(std::time::Duration::from_secs(20))
//                 .unwrap()
//         );
//     }

//     #[test]
//     fn test_finalize_from_unidentified_clips_error() {
//         let video_details = sample_video_details(15);
//         let clips = vec![sample_unidentified_clip(10, 20)];
//         let result =
//             FinalizedVideo::finalize_from_unidentified_clips(video_details, clips);
//         assert!(matches!(
//             result,
//             Err(VideoFinalizationError::ClipLongerThanVideo { .. })
//         ));
//     }

//     #[test]
//     fn test_finalize_from_clips_success() {
//         let video_details = sample_video_details(100);
//         let clips = vec![
//             Clip::Unidentified(sample_unidentified_clip(10, 20)),
//             Clip::Identified(sample_identified_clip(30, 40)),
//         ];
//         let result = FinalizedVideo::finalize_from_clips(video_details.clone(), clips);
//         assert!(result.is_ok());
//         let finalized = result.unwrap();
//         assert_eq!(
//             finalized.video_details.get_video_id(),
//             video_details.get_video_id()
//         );
//         assert_eq!(finalized.clips.len(), 2);
//         assert_eq!(
//             finalized.clips[0].get_start_time(),
//             &crate::model::Duration::from_std(std::time::Duration::from_secs(10))
//                 .unwrap()
//         );
//         assert_eq!(
//             finalized.clips[1].get_end_time(),
//             &crate::model::Duration::from_std(std::time::Duration::from_secs(40))
//                 .unwrap()
//         );
//     }

//     #[test]
//     fn test_finalize_from_clips_error() {
//         let video_details = sample_video_details(15);
//         let clips = vec![
//             Clip::Unidentified(sample_unidentified_clip(10, 20)),
//             Clip::Identified(sample_identified_clip(5, 25)),
//         ];
//         let result = FinalizedVideo::finalize_from_clips(video_details, clips);
//         assert!(matches!(
//             result,
//             Err(VideoFinalizationError::ClipLongerThanVideo { .. })
//         ));
//     }
// }
