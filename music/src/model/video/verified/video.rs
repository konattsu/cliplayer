/// 内部のclipsの整合性が全て取れている動画
///
/// serialize時, clipsの`start_time`順にソートされていることを保証
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct VerifiedVideo {
    /// 動画の詳細情報
    record: crate::model::VideoRecord,
    /// クリップ
    clips: Vec<crate::model::VerifiedClip>,
}

// recordの情報を基にVerifiedClipを作成する必要があるため, カスタムデシリアライザ実装
impl<'de> serde::Deserialize<'de> for VerifiedVideo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[serde(deny_unknown_fields)]
        struct RawVerifiedVideo {
            #[serde(flatten)]
            record: crate::model::VideoRecord,
            clips: Vec<crate::model::UnverifiedClip>,
        }
        let raw = RawVerifiedVideo::deserialize(deserializer)?;
        // ここでrecordの情報を基にVerifiedClipを作成
        let verified_clips = Self::verify_clips_from_unverified(
            raw.clips,
            raw.record.get_api().get_duration(),
        )
        .map_err(|e| serde::de::Error::custom(e.to_pretty_string()))?;
        Self::new(raw.record, verified_clips)
            .map_err(|e| serde::de::Error::custom(e.to_pretty_string()))
    }
}

impl serde::Serialize for VerifiedVideo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // clipsのstart_time順にソートしてからserialize
        let mut sorted_clips = self.clips.clone();
        sorted_clips.sort_by_key(|clip| clip.get_start_time().as_secs());

        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        #[serde(deny_unknown_fields)]
        struct RawVerifiedVideo<'a> {
            #[serde(flatten)]
            record: &'a crate::model::VideoRecord,
            clips: Vec<crate::model::VerifiedClip>,
        }

        let raw = RawVerifiedVideo {
            record: &self.record,
            clips: sorted_clips,
        };
        raw.serialize(serializer)
    }
}

impl VerifiedVideo {
    pub(crate) fn get_video_id(&self) -> &crate::model::VideoId {
        self.record.get_video_id()
    }
    // local
    pub(crate) fn get_uploader_name(&self) -> Option<&crate::model::UploaderName> {
        self.record.get_local().get_uploader_name()
    }
    pub(crate) fn get_video_tags(&self) -> &crate::model::VideoTags {
        self.record.get_local().get_video_tags()
    }
    // api
    pub(crate) fn get_title(&self) -> &str {
        self.record.get_api().get_title()
    }
    pub(crate) fn get_channel_id(&self) -> &crate::model::ChannelId {
        self.record.get_api().get_channel_id()
    }
    pub(crate) fn get_published_at(&self) -> &crate::model::VideoPublishedAt {
        self.record.get_api().get_published_at()
    }
    pub(crate) fn get_synced_at(&self) -> &chrono::DateTime<chrono::Utc> {
        self.record.get_api().get_synced_at()
    }
    pub(crate) fn get_duration(&self) -> &crate::model::Duration {
        self.record.get_api().get_duration()
    }
    pub(crate) fn get_privacy_status(&self) -> &crate::model::PrivacyStatus {
        self.record.get_api().get_privacy_status()
    }
    pub(crate) fn is_embeddable(&self) -> bool {
        self.record.get_api().is_embeddable()
    }

    pub(crate) fn get_year(&self) -> usize {
        self.record.get_api().get_published_at().get_year()
    }
    pub(crate) fn get_month(&self) -> usize {
        self.record.get_api().get_published_at().get_month()
    }
    pub(crate) fn to_clips(&self) -> Vec<&crate::model::VerifiedClip> {
        self.clips.iter().collect()
    }

    /// `AnonymousVideo`と`ApiVideoInfo`から`VerifiedVideo`を作成
    ///
    /// Error:
    /// - `record`の動画IDと`anonymous_video`の動画IDが一致しないとき
    /// - `anonymous_video`のクリップの情報が不正なとき
    pub(crate) fn from_anonymous_video(
        anonymous_video: crate::model::AnonymousVideo,
        api_info: crate::model::ApiVideoInfo,
    ) -> Result<Self, super::VerifiedVideoError> {
        super::VerifiedVideoError::ensure_video_id_match(
            anonymous_video.get_video_id(),
            api_info.get_video_id(),
        )?;
        let (local_info, clips) = anonymous_video.into_inner();

        let (oks, errs): (Vec<_>, Vec<_>) = clips
            .into_iter()
            .map(|clip| clip.try_into_verified_clip(api_info.get_duration()))
            // ここでoks, errsに分割しているため後方の処理では
            // それぞれunwrapを使用. パニックしない.
            .partition(Result::is_ok);

        if !errs.is_empty() {
            Err(super::VerifiedVideoError::InvalidClip(
                errs.into_iter().map(Result::unwrap_err).collect(),
            ))
        } else {
            let clips = oks.into_iter().map(Result::unwrap).collect();
            // 上で同じvideo_idであることをすでに保証しているためunwrapで処理
            let record = crate::model::VideoRecord::new(local_info, api_info).unwrap();
            Self::new(record, clips)
        }
    }

    /// 既存の`VerifiedVideo`に新しいapiで手に入れた動画の詳細情報を適用する
    pub(super) fn with_new_api_info(
        mut self,
        api: crate::model::ApiVideoInfo,
    ) -> Result<Self, super::VerifiedVideoError> {
        let record = match self.record.with_new_api_info(api) {
            // 動画idが同じなとき(正常)
            Ok(is_not_modified) => {
                if is_not_modified {
                    // 変更がなかったとき
                    return Ok(self);
                } else {
                    // 変更があったとき
                    // 変更されたrecordを基にclips情報を再度verifyしたいのでrecord返す
                    self.record
                }
            }
            // 動画idが異なったとき(異常)
            Err(e) => {
                return Err(crate::model::VerifiedVideoError::VideoIdMismatch {
                    local: e.local,
                    fetched: e.api,
                });
            }
        };

        let unverified_clips: Vec<crate::model::UnverifiedClip> = self
            .clips
            .into_iter()
            .map(crate::model::UnverifiedClip::from_verified_clip)
            .collect();

        match Self::verify_clips_from_unverified(
            unverified_clips,
            record.get_api().get_duration(),
        ) {
            Ok(verified_clips) => Self::new(record, verified_clips),
            Err(e) => Err(e),
        }
    }

    /// 新しい`VerifiedVideo`を作成
    ///
    /// `Self::validate_consistency`は通す
    ///
    /// # Importance:
    ///
    /// VerifiedClipsはvideo_detailを基に作成する.
    /// 呼び出し時に意図しない(不正な)video_detailを基にVerifiedClipを作成しないように.
    ///
    /// # Error:
    /// - クリップの範囲が重複しているとき
    /// - クリップが空のとき
    fn new(
        record: crate::model::VideoRecord,
        clips: Vec<crate::model::VerifiedClip>,
    ) -> Result<Self, super::VerifiedVideoError> {
        Self::validate_consistency(&clips, record.get_video_id())?;
        Ok(VerifiedVideo { record, clips })
    }

    /// 動画のクリップの整合性を検証
    ///
    /// - クリップに指定した範囲が重複していないか
    /// - クリップが空でないか
    fn validate_consistency(
        clips: &[crate::model::VerifiedClip],
        video_id: &crate::model::VideoId,
    ) -> Result<(), super::VerifiedVideoError> {
        // クリップが空でないか
        if clips.is_empty() {
            return Err(super::VerifiedVideoError::NoClips(video_id.clone()));
        }

        // クリップの範囲が重複していないか確認
        let mut overlap_clips = Vec::new();
        for w in clips.windows(2) {
            if w[0].get_end_time() > w[1].get_start_time() {
                overlap_clips.push(w[0].clone());
                overlap_clips.push(w[1].clone());
            }
        }
        if overlap_clips.is_empty() {
            Ok(())
        } else {
            Err(super::VerifiedVideoError::ClipsOverlap {
                id: video_id.clone(),
                clips_title: overlap_clips
                    .iter()
                    .map(|c| c.get_song_title().to_string())
                    .collect(),
            })
        }
    }

    /// 動画を認証
    ///
    /// - Err(e): クリップの情報が不正なとき(動画の長さに対してclipが不正)
    /// - Ok(verified_clips): 認証されたクリップのリスト
    fn verify_clips_from_unverified(
        clips: Vec<crate::model::UnverifiedClip>,
        video_duration: &crate::model::Duration,
    ) -> Result<Vec<crate::model::VerifiedClip>, super::VerifiedVideoError> {
        let (oks, errs): (Vec<_>, Vec<_>) = clips
            .into_iter()
            .map(|clip| clip.try_into_verified_clip(video_duration))
            // ここでoks, errsに分割しているため後方の処理では
            // それぞれunwrapを使用. パニックしない.
            .partition(Result::is_ok);

        if !errs.is_empty() {
            Err(super::VerifiedVideoError::InvalidClip(
                errs.into_iter().map(Result::unwrap_err).collect(),
            ))
        } else {
            Ok(oks.into_iter().map(Result::unwrap).collect())
        }
    }
}

// MARK: For Tests
#[cfg(test)]
impl VerifiedVideo {
    pub(crate) fn self_a() -> Self {
        let detail = crate::model::VideoRecord::self_a();
        let clips = vec![
            crate::model::VerifiedClip::self_a_1(),
            crate::model::VerifiedClip::self_a_2(),
            crate::model::VerifiedClip::self_a_3(),
        ];
        Self::new(detail, clips).expect("should create valid VerifiedVideo")
    }
    pub(crate) fn self_b() -> Self {
        let detail = crate::model::VideoRecord::self_b();
        let clips = vec![
            crate::model::VerifiedClip::self_b_1(),
            crate::model::VerifiedClip::self_b_2(),
            crate::model::VerifiedClip::self_b_3(),
        ];
        Self::new(detail, clips).expect("should create valid VerifiedVideo")
    }
}

// MARK: Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verified_video_for_test_methods() {
        let _a = VerifiedVideo::self_a();
        let _b = VerifiedVideo::self_b();
    }

    #[test]
    fn test_verified_video_from_anonymous_video_invalid() {
        let local = crate::model::LocalVideoInfo::self_a();
        let anonymous_clips = vec![
            crate::model::AnonymousClip::self_a_1(),
            crate::model::AnonymousClip::self_a_2(),
            crate::model::AnonymousClip::self_a_3(),
        ];
        let anonymous_video = crate::model::AnonymousVideo::new(local, anonymous_clips)
            .expect("should create valid AnonymousVideo");
        let result = VerifiedVideo::from_anonymous_video(
            anonymous_video,
            // 動画IDが違う
            crate::model::VideoRecord::self_b().get_api().clone(),
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            super::super::VerifiedVideoError::VideoIdMismatch { .. }
        ));
    }

    #[test]
    fn test_verified_video_with_new_video_detail_not_modified() {
        use chrono::TimeZone;

        let verified_video = VerifiedVideo {
            record: crate::model::VideoRecord::self_a(),
            clips: vec![
                crate::model::VerifiedClip::self_a_1(),
                crate::model::VerifiedClip::self_a_2(),
            ],
        };
        let synced_at = chrono::Utc.with_ymd_and_hms(2025, 8, 8, 8, 8, 8).unwrap();
        let new_record =
            crate::model::VideoRecord::self_a().update_synced_at(synced_at);
        let updated_video = verified_video
            .with_new_api_info(new_record.get_api().clone())
            .expect("should update video detail");

        assert_eq!(updated_video.record, new_record);
        assert_eq!(updated_video.clips.len(), 2);
    }

    #[test]
    fn test_verified_video_with_new_video_detail_modified() {
        use chrono::TimeZone;
        let verified_video = VerifiedVideo {
            record: crate::model::VideoRecord::self_a(),
            clips: vec![
                crate::model::VerifiedClip::self_a_1(),
                crate::model::VerifiedClip::self_a_2(),
            ],
        };
        let synced_at = chrono::Utc.with_ymd_and_hms(2025, 8, 8, 8, 8, 8).unwrap();
        let new_record = crate::model::VideoRecord::self_a()
            .update_synced_at(synced_at)
            // 変更を加える
            .set_duration(crate::model::Duration::from_secs_u16(1));
        // 変更を加えて動画時間が短くなり, クリップが無効になったとき
        let updated_video =
            verified_video.with_new_api_info(new_record.get_api().clone());
        assert!(updated_video.is_err());
    }

    #[test]
    fn test_verified_video_validate_consistency_empty() {
        let res = VerifiedVideo::validate_consistency(
            &[],
            &crate::model::VideoId::test_id_1(),
        );
        assert!(matches!(
            res,
            Err(super::super::VerifiedVideoError::NoClips(_))
        ));
    }

    #[test]
    fn test_verified_video_validate_consistency_overlap() {
        // 2つのクリップが重複する場合
        let mut c1 = crate::model::VerifiedClip::self_a_1();
        let c2 = crate::model::VerifiedClip::self_a_2();
        // c1のend_timeをc2のstart_timeより後ろに
        c1.set_end_time(
            c2.get_start_time()
                .try_add(&crate::model::Duration::from_secs_u16(100))
                .unwrap(),
        );
        let res = VerifiedVideo::validate_consistency(
            &[c1, c2],
            &crate::model::VideoId::test_id_1(),
        );
        assert!(matches!(
            res,
            Err(super::super::VerifiedVideoError::ClipsOverlap { .. })
        ));
    }
}
