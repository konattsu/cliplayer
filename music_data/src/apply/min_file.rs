/// 全てのファイルを保存
pub(crate) fn save(
    music_lib: crate::music_file::MusicLibrary,
    min_clips_path: &crate::util::FilePath,
    min_videos_path: &crate::util::FilePath,
) -> Result<(), crate::music_file::MusicFileError> {
    // 月ファイル
    music_lib.save_month_files()?;
    // minファイル
    save_min_files(music_lib, min_clips_path, min_videos_path)
}

/// minファイルを保存
pub(super) fn save_min_files(
    music_lib: crate::music_file::MusicLibrary,
    min_clips_path: &crate::util::FilePath,
    min_videos_path: &crate::util::FilePath,
) -> Result<(), crate::music_file::MusicFileError> {
    tracing::debug!(
        "Saving min files to disk: `{}` and `{}`",
        min_clips_path,
        min_videos_path,
    );

    let videos = music_lib.into_videos()?;

    let flat_clips = FlatClips::from_verified_videos(&videos);
    let flat_videos = FlatVideos::from_verified_videos(&videos);

    crate::music_file::MusicLibrary::save_min_file(&flat_clips, min_clips_path)?;
    crate::music_file::MusicLibrary::save_min_file(&flat_videos, min_videos_path)?;

    Ok(())
}

// MARK: Clips

/// 出力用のクリップ一覧
///
/// ! カスタムシリアライザ実装. 下見て
#[derive(serde::Serialize)]
struct FlatClips<'a>(
    std::collections::HashMap<&'a crate::model::UuidVer4, FlatClipValue<'a>>,
);

struct FlatClipValue<'a> {
    /// 曲名
    song_title: &'a str,
    /// 内部アーティストの一覧
    artists: &'a crate::model::InternalArtists,
    /// 外部アーティストの一覧
    external_artists: Option<&'a crate::model::ExternalArtists>,
    /// 切り抜いた動画が存在した場合の動画id
    clipped_video_id: Option<&'a crate::model::VideoId>,
    /// 曲が始まる時間
    start_time: &'a crate::model::Duration,
    /// 曲が終わる時間
    end_time: &'a crate::model::Duration,
    /// タグ
    clip_tags: Option<&'a crate::model::ClipTags>,
    /// 音量の正規化時に設定すべき音量
    volume_percent: Option<&'a crate::model::VolumePercent>,
    /// このクリップと紐づく動画id
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
            artists: &'a crate::model::InternalArtists,
            #[serde(skip_serializing_if = "Option::is_none")]
            external_artists: Option<&'a crate::model::ExternalArtists>,
            #[serde(skip_serializing_if = "Option::is_none")]
            clipped_video_id: Option<&'a crate::model::VideoId>,
            start_time_secs: u32,
            end_time_secs: u32,
            #[serde(skip_serializing_if = "Option::is_none")]
            clip_tags: Option<&'a crate::model::ClipTags>,
            #[serde(skip_serializing_if = "Option::is_none")]
            volume_percent: Option<&'a crate::model::VolumePercent>,
            video_id: &'a crate::model::VideoId,
        }
        let value = RawFlatClipValue {
            song_title: self.song_title,
            artists: self.artists,
            external_artists: self.external_artists,
            clipped_video_id: self.clipped_video_id,
            start_time_secs: self.start_time.as_secs(),
            end_time_secs: self.end_time.as_secs(),
            clip_tags: self.clip_tags,
            volume_percent: self.volume_percent,
            video_id: self.video_id,
        };
        value.serialize(serializer)
    }
}

impl<'a> FlatClips<'a> {
    fn from_verified_videos(videos: &'a crate::model::VerifiedVideos) -> Self {
        let mut flat_clips = std::collections::HashMap::new();
        for video in videos.to_vec() {
            let video_id = video.get_video_id();
            for clip in video.to_clips() {
                flat_clips.insert(
                    clip.get_uuid(),
                    FlatClipValue {
                        song_title: clip.get_song_title(),
                        artists: clip.get_artists(),
                        external_artists: clip.get_external_artists(),
                        clipped_video_id: clip.get_clipped_video_id(),
                        start_time: clip.get_start_time(),
                        end_time: clip.get_end_time(),
                        clip_tags: clip.get_clip_tags(),
                        volume_percent: clip.get_volume_percent(),
                        video_id,
                    },
                );
            }
        }
        FlatClips(flat_clips)
    }
}

// MARK: Videos

/// 出力用の動画のメタデータ一覧
///
/// ! カスタムシリアライザ実装. 下見て
#[derive(serde::Serialize)]
struct FlatVideos<'a>(
    std::collections::HashMap<&'a crate::model::VideoId, FlatVideoValue<'a>>,
);

struct FlatVideoValue<'a> {
    /// この動画に紐づくクリップのuuidの一覧
    clip_uuids: Vec<&'a crate::model::UuidVer4>,

    // 少なくとも1回クリップに出演している内部アーティスト
    artists: crate::model::InternalArtists,

    // api
    /// 動画のタイトル
    title: &'a str,
    /// チャンネルID
    channel_id: &'a crate::model::ChannelId,
    /// 動画の公開日時
    published_at: &'a crate::model::VideoPublishedAt,
    /// この情報を取得した日時
    synced_at: &'a chrono::DateTime<chrono::Utc>,
    /// 動画の長さ
    duration: &'a crate::model::Duration,
    /// 動画のプライバシー設定
    privacy_status: &'a crate::model::PrivacyStatus,
    /// 動画が埋め込み可能かどうか
    embeddable: bool,

    // local
    /// チャンネル名, 箱外で行われた配信/動画の時に付与
    uploader_name: Option<&'a crate::model::UploaderName>,
    /// 動画のタグ
    video_tags: &'a crate::model::VideoTags,
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
            artists: &'a crate::model::InternalArtists,
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
            video_tags: &'a crate::model::VideoTags,
        }
        let value = RawFlatVideoValue {
            clip_uuids: &self.clip_uuids,
            artists: &self.artists,
            title: self.title,
            channel_id: self.channel_id,
            published_at: self.published_at,
            synced_at: self.synced_at,
            duration_secs: self.duration.as_secs(),
            privacy_status: self.privacy_status,
            embeddable: self.embeddable,
            uploader_name: self.uploader_name,
            video_tags: self.video_tags,
        };
        value.serialize(serializer)
    }
}

impl<'a> FlatVideos<'a> {
    pub(super) fn from_verified_videos(
        videos: &'a crate::model::VerifiedVideos,
    ) -> Self {
        let mut flat_videos = std::collections::HashMap::new();
        for video in videos.to_vec() {
            let video_id = video.get_video_id();
            let artists = Self::collect_artists(video);

            flat_videos.insert(
                video_id,
                FlatVideoValue {
                    clip_uuids: video.to_clips().iter().map(|c| c.get_uuid()).collect(),
                    artists,
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
            );
        }
        FlatVideos(flat_videos)
    }

    /// 動画に1回でも出演している内部アーティストの一覧を返却
    ///
    /// 重複はしない
    fn collect_artists(
        video: &'a crate::model::VerifiedVideo,
    ) -> crate::model::InternalArtists {
        let mut artists = std::collections::HashSet::new();
        for clip in video.to_clips() {
            artists.extend(clip.get_artists().to_vec());
        }
        crate::model::InternalArtists::new(artists.into_iter().collect())
            .expect("will not fail")
    }
}

// MARK: Tests

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, to_value};

    fn iso8601_from_datetime_serde(dt: &chrono::DateTime<chrono::Utc>) -> String {
        let mut buf = Vec::new();
        let mut ser = serde_json::Serializer::new(&mut buf);
        crate::util::datetime_serde::serialize(dt, &mut ser).unwrap();
        let json_str = String::from_utf8(buf).unwrap();
        serde_json::from_str::<String>(&json_str).unwrap()
    }

    #[test]
    fn test_flat_clip_value_serialize_some_1() {
        let artists = crate::model::InternalArtists::self_1();
        let external_artists = Some(crate::model::ExternalArtists::self_1());
        let clipped_video_id = Some(crate::model::VideoId::test_id_4());
        let clip_tags = Some(crate::model::ClipTags::self_1());
        let volume_percent = Some(crate::model::VolumePercent::new(1).unwrap());
        let video_id = crate::model::VideoId::test_id_1();
        let start_time = crate::model::Duration::self_1();
        let end_time = crate::model::Duration::self_2();

        let val = FlatClipValue {
            song_title: "dummy",
            artists: &artists,
            external_artists: external_artists.as_ref(),
            clipped_video_id: clipped_video_id.as_ref(),
            start_time: &start_time,
            end_time: &end_time,
            clip_tags: clip_tags.as_ref(),
            volume_percent: volume_percent.as_ref(),
            video_id: &video_id,
        };
        let ser = to_value(&val).unwrap();
        // 期待値も実装に合わせて修正
        let expected = json!({
            "songTitle": "dummy",
            "artists": artists,
            "externalArtists": external_artists,
            "clippedVideoId": clipped_video_id,
            "startTimeSecs": start_time.as_secs(),
            "endTimeSecs": end_time.as_secs(),
            "clipTags": clip_tags,
            "volumePercent": volume_percent,
            "videoId": video_id
        });
        assert_eq!(ser, expected);
    }

    #[test]
    fn test_flat_clip_value_serialize_some_2() {
        let artists = crate::model::InternalArtists::self_2();
        let external_artists = Some(crate::model::ExternalArtists::self_2());
        let clipped_video_id = Some(crate::model::VideoId::test_id_3());
        let clip_tags = Some(crate::model::ClipTags::self_2());
        let volume_percent = Some(crate::model::VolumePercent::new(99).unwrap());
        let video_id = crate::model::VideoId::test_id_2();
        let start_time = crate::model::Duration::self_2();
        let end_time = crate::model::Duration::self_3();

        let val = FlatClipValue {
            song_title: "another",
            artists: &artists,
            external_artists: external_artists.as_ref(),
            clipped_video_id: clipped_video_id.as_ref(),
            start_time: &start_time,
            end_time: &end_time,
            clip_tags: clip_tags.as_ref(),
            volume_percent: volume_percent.as_ref(),
            video_id: &video_id,
        };
        let ser = to_value(&val).unwrap();
        let expected = json!({
            "songTitle": "another",
            "artists": artists,
            "externalArtists": external_artists,
            "clippedVideoId": clipped_video_id,
            "startTimeSecs": start_time.as_secs(),
            "endTimeSecs": end_time.as_secs(),
            "clipTags": clip_tags,
            "volumePercent": volume_percent,
            "videoId": video_id
        });
        assert_eq!(ser, expected);
    }

    #[test]
    fn test_flat_clip_value_serialize_none_1() {
        let artists = crate::model::InternalArtists::self_1();
        let video_id = crate::model::VideoId::test_id_1();
        let start_time = crate::model::Duration::self_1();
        let end_time = crate::model::Duration::self_2();

        let val = FlatClipValue {
            song_title: "dummy",
            artists: &artists,
            external_artists: None,
            clipped_video_id: None,
            start_time: &start_time,
            end_time: &end_time,
            clip_tags: None,
            volume_percent: None,
            video_id: &video_id,
        };
        let ser = to_value(&val).unwrap();
        let expected = json!({
            "songTitle": "dummy",
            "artists": artists,
            "startTimeSecs": start_time.as_secs(),
            "endTimeSecs": end_time.as_secs(),
            "videoId": video_id
        });
        assert_eq!(ser, expected);
    }

    #[test]
    fn test_flat_clip_value_serialize_none_2() {
        let artists = crate::model::InternalArtists::self_2();
        let video_id = crate::model::VideoId::test_id_2();
        let start_time = crate::model::Duration::self_2();
        let end_time = crate::model::Duration::self_3();

        let val = FlatClipValue {
            song_title: "none_case",
            artists: &artists,
            external_artists: None,
            clipped_video_id: None,
            start_time: &start_time,
            end_time: &end_time,
            clip_tags: None,
            volume_percent: None,
            video_id: &video_id,
        };
        let ser = to_value(&val).unwrap();
        let expected = json!({
            "songTitle": "none_case",
            "artists": artists,
            "startTimeSecs": start_time.as_secs(),
            "endTimeSecs": end_time.as_secs(),
            "videoId": video_id
        });
        assert_eq!(ser, expected);
    }

    #[test]
    fn test_flat_video_value_serialize_some_1() {
        let uuid1 = crate::model::UuidVer4::self_1();
        let uuid2 = crate::model::UuidVer4::self_2();
        let channel_id = crate::model::ChannelId::test_id_2();
        let published_at = crate::model::VideoPublishedAt::self_1();
        let synced_at = chrono::Utc::now();
        let duration = crate::model::Duration::self_1();
        let privacy_status = crate::model::PrivacyStatus::Private;
        let uploader_name = Some(crate::model::UploaderName::test_uploader_name_1());
        let video_tags = crate::model::VideoTags::self_1();

        let artists = crate::model::InternalArtists::self_1();
        let val = FlatVideoValue {
            clip_uuids: vec![&uuid1, &uuid2],
            artists,
            title: "dummy",
            channel_id: &channel_id,
            published_at: &published_at,
            synced_at: &synced_at,
            duration: &duration,
            privacy_status: &privacy_status,
            embeddable: true,
            uploader_name: uploader_name.as_ref(),
            video_tags: &video_tags,
        };
        let ser = to_value(&val).unwrap();
        let expected = json!({
            "clipUuids": [uuid1, uuid2],
            "artists": val.artists.clone(),
            "title": "dummy",
            "channelId": channel_id,
            "publishedAt": published_at,
            "syncedAt": iso8601_from_datetime_serde(&synced_at),
            "durationSecs": duration.as_secs(),
            "privacyStatus": privacy_status,
            "embeddable": true,
            "uploaderName": uploader_name,
            "videoTags": video_tags
        });
        assert_eq!(ser, expected);
    }

    #[test]
    fn test_flat_video_value_serialize_some_2() {
        let uuid1 = crate::model::UuidVer4::self_2();
        let uuid2 = crate::model::UuidVer4::self_1();
        let channel_id = crate::model::ChannelId::test_id_2();
        let published_at = crate::model::VideoPublishedAt::self_2();
        let synced_at = chrono::Utc::now();
        let duration = crate::model::Duration::self_2();
        let privacy_status = crate::model::PrivacyStatus::Public;
        let uploader_name = Some(crate::model::UploaderName::test_uploader_name_3());
        let video_tags = crate::model::VideoTags::self_2();

        let artists = crate::model::InternalArtists::self_2();
        let val = FlatVideoValue {
            clip_uuids: vec![&uuid1, &uuid2],
            artists,
            title: "video2",
            channel_id: &channel_id,
            published_at: &published_at,
            synced_at: &synced_at,
            duration: &duration,
            privacy_status: &privacy_status,
            embeddable: false,
            uploader_name: uploader_name.as_ref(),
            video_tags: &video_tags,
        };
        let ser = to_value(&val).unwrap();
        let expected = json!({
            "clipUuids": [uuid1, uuid2],
            "artists": val.artists.clone(),
            "title": "video2",
            "channelId": channel_id,
            "publishedAt": published_at,
            "syncedAt": iso8601_from_datetime_serde(&synced_at),
            "durationSecs": duration.as_secs(),
            "privacyStatus": privacy_status,
            "embeddable": false,
            "uploaderName": uploader_name,
            "videoTags": video_tags
        });
        assert_eq!(ser, expected);
    }

    #[test]
    fn test_flat_video_value_serialize_none_1() {
        let uuid1 = crate::model::UuidVer4::self_1();
        let channel_id = crate::model::ChannelId::test_id_1();
        let published_at = crate::model::VideoPublishedAt::self_1();
        let synced_at = chrono::Utc::now();
        let duration = crate::model::Duration::self_1();
        let privacy_status = crate::model::PrivacyStatus::Unlisted;
        let video_tags = crate::model::VideoTags::self_1();

        let artists = crate::model::InternalArtists::self_1();
        let val = FlatVideoValue {
            clip_uuids: vec![&uuid1],
            artists,
            title: "dummy",
            channel_id: &channel_id,
            published_at: &published_at,
            synced_at: &synced_at,
            duration: &duration,
            privacy_status: &privacy_status,
            embeddable: false,
            uploader_name: None,
            video_tags: &video_tags,
        };
        let ser = to_value(&val).unwrap();
        let expected = json!({
            "clipUuids": [uuid1],
            "artists": val.artists.clone(),
            "title": "dummy",
            "channelId": channel_id,
            "publishedAt": published_at,
            "syncedAt": iso8601_from_datetime_serde(&synced_at),
            "durationSecs": duration.as_secs(),
            "privacyStatus": privacy_status,
            "embeddable": false,
            "videoTags": video_tags
        });
        assert_eq!(ser, expected);
    }

    #[test]
    fn test_flat_video_value_serialize_none_2() {
        let uuid1 = crate::model::UuidVer4::self_2();
        let channel_id = crate::model::ChannelId::test_id_2();
        let published_at = crate::model::VideoPublishedAt::self_2();
        let synced_at = chrono::Utc::now();
        let duration = crate::model::Duration::self_2();
        let privacy_status = crate::model::PrivacyStatus::Public;
        let video_tags = crate::model::VideoTags::self_2();

        let artists = crate::model::InternalArtists::self_2();
        let val = FlatVideoValue {
            clip_uuids: vec![&uuid1],
            artists,
            title: "none_case",
            channel_id: &channel_id,
            published_at: &published_at,
            synced_at: &synced_at,
            duration: &duration,
            privacy_status: &privacy_status,
            embeddable: true,
            uploader_name: None,
            video_tags: &video_tags,
        };
        let ser = to_value(&val).unwrap();
        let expected = json!({
            "clipUuids": [uuid1],
            "artists": val.artists.clone(),
            "title": "none_case",
            "channelId": channel_id,
            "publishedAt": published_at,
            "syncedAt": iso8601_from_datetime_serde(&synced_at),
            "durationSecs": duration.as_secs(),
            "privacyStatus": privacy_status,
            "embeddable": true,
            "videoTags": video_tags
        });
        assert_eq!(ser, expected);
    }
}
