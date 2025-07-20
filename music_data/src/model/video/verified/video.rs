/// 内部のclipsの整合性が全て取れている動画
///
/// clipsの`start_time`順にソートされていることを保証
#[derive(serde::Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct VerifiedVideo {
    /// 動画の詳細情報
    #[serde(flatten)]
    video_detail: crate::model::VideoDetail,
    /// クリップ
    ///
    /// `start_time`順にソートされていることを保証
    clips: Vec<crate::model::VerifiedClip>,
}

// 以下をデシリアライズ時に行うためのカスタムデシリアライザ
// - video_detailの情報を基にVerifiedClipを作成する必要がある
// - clipsをソート
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
            video_detail: crate::model::VideoDetail,
            clips: Vec<crate::model::UnverifiedClip>,
        }
        let raw = RawVerifiedVideo::deserialize(deserializer)?;
        // ここでdetailの情報を基にVerifiedClipを作成
        let mut verified_clips = raw
            .clips
            .into_iter()
            .map(|clip| {
                clip.try_into_verified_clip(
                    raw.video_detail.get_published_at(),
                    raw.video_detail.get_duration(),
                )
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(serde::de::Error::custom)?;
        // ソートして返す
        verified_clips.sort_by_key(|clip| clip.get_start_time().as_secs());
        Self::new(raw.video_detail, verified_clips)
            .map_err(|e| serde::de::Error::custom(e.to_pretty_string()))
    }
}

impl VerifiedVideo {
    pub(crate) fn get_detail(&self) -> &crate::model::VideoDetail {
        &self.video_detail
    }
    pub(crate) fn get_year(&self) -> usize {
        self.video_detail.get_published_at().get_year()
    }
    pub(crate) fn get_month(&self) -> usize {
        self.video_detail.get_published_at().get_month()
    }
    pub(crate) fn get_video_id(&self) -> &crate::model::VideoId {
        self.video_detail.get_video_id()
    }
    pub(crate) fn get_published_at(&self) -> &crate::model::VideoPublishedAt {
        self.video_detail.get_published_at()
    }

    pub(crate) fn into_clips(self) -> Vec<crate::model::VerifiedClip> {
        self.clips
    }

    /// `AnonymousVideo`と`VideoDetail`から`VerifiedVideo`を作成
    ///
    /// Error:
    /// - `video_detail`の動画IDと`anonymous_video`の動画IDが一致しないとき
    /// - `anonymous_video`のクリップの情報が不正なとき
    pub(crate) fn from_anonymous_video(
        anonymous_video: crate::model::AnonymousVideo,
        video_detail: crate::model::VideoDetail,
    ) -> Result<Self, super::VerifiedVideoError> {
        super::VerifiedVideoError::ensure_video_id_match(
            anonymous_video.get_video_id(),
            video_detail.get_video_id(),
        )?;
        let (_brief, clips) = anonymous_video.into_inner();

        let (oks, errs): (Vec<_>, Vec<_>) = clips
            .into_iter()
            .map(|clip| {
                clip.try_into_verified_clip(
                    video_detail.get_published_at(),
                    video_detail.get_duration(),
                )
            })
            // ここでoks, errsに分割しているため後方の処理では
            // それぞれunwrapを使用. パニックしない.
            .partition(Result::is_ok);

        if !errs.is_empty() {
            Err(super::VerifiedVideoError::InvalidClip(
                errs.into_iter().map(Result::unwrap_err).collect(),
            ))
        } else {
            let clips = oks.into_iter().map(Result::unwrap).collect();
            Self::new(video_detail, clips)
        }
    }

    /// 既存の`VerifiedVideo`に新しい動画の詳細情報を適用する
    pub(crate) fn with_new_video_detail(
        self,
        detail: crate::model::VideoDetail,
    ) -> Result<Self, super::VerifiedVideoError> {
        // 内容が変更されていないとき
        if self.video_detail.is_same_except_modified_at(&detail) {
            // modified_atを更新する
            return Ok(Self {
                video_detail: detail,
                clips: self.clips,
            });
        }

        // 動画idが変更されていないかどうか確認
        super::VerifiedVideoError::ensure_video_id_match(
            self.video_detail.get_video_id(),
            detail.get_video_id(),
        )?;

        let unverified_clips: Vec<crate::model::UnverifiedClip> = self
            .clips
            .into_iter()
            .map(crate::model::UnverifiedClip::from_verified_clip)
            .collect();

        let (oks, errs): (Vec<_>, Vec<_>) = unverified_clips
            .into_iter()
            .map(|clip| {
                clip.try_into_verified_clip(
                    detail.get_published_at(),
                    detail.get_duration(),
                )
            })
            // ここでoks, errsに分割しているため後方の処理では
            // それぞれunwrapを使用. パニックしない.
            .partition(Result::is_ok);

        if !errs.is_empty() {
            Err(super::VerifiedVideoError::InvalidClip(
                errs.into_iter().map(Result::unwrap_err).collect(),
            ))
        } else {
            let clips = oks.into_iter().map(Result::unwrap).collect();
            Self::new(detail, clips)
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
        video_detail: crate::model::VideoDetail,
        clips: Vec<crate::model::VerifiedClip>,
    ) -> Result<Self, super::VerifiedVideoError> {
        Self::validate_consistency(&clips, video_detail.get_video_id())?;
        Ok(VerifiedVideo {
            video_detail,
            clips,
        })
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
}

// MARK: For Tests
#[cfg(test)]
impl VerifiedVideo {
    pub(crate) fn self_a() -> Self {
        let detail = crate::model::VideoDetail::self_a();
        let clips = vec![
            crate::model::VerifiedClip::self_a_1(),
            crate::model::VerifiedClip::self_a_2(),
            crate::model::VerifiedClip::self_a_3(),
        ];
        Self::new(detail, clips).expect("should create valid VerifiedVideo")
    }
    pub(crate) fn self_b() -> Self {
        let detail = crate::model::VideoDetail::self_b();
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
    fn test_verified_video_from_anonymous_video_valid() {
        let brief = crate::model::VideoBrief::self_a();
        let anonymous_clips = vec![
            // ソートされてない
            crate::model::AnonymousClip::self_a_3(),
            crate::model::AnonymousClip::self_a_1(),
            crate::model::AnonymousClip::self_a_2(),
        ];
        let anonymous_video = crate::model::AnonymousVideo::new(brief, anonymous_clips)
            .expect("should create valid AnonymousVideo");
        let created_verified_video = VerifiedVideo::from_anonymous_video(
            anonymous_video,
            crate::model::VideoDetail::self_a(),
        )
        .expect("should create valid VerifiedVideo");

        let clips = created_verified_video.into_clips();
        assert_eq!(clips.len(), 3);
        // ソートできているか確認. uuidは自動生成なので比較対象には含めない
        assert_eq!(clips[0].get_start_time().as_secs(), 5);
        assert_eq!(clips[1].get_start_time().as_secs(), 15);
        assert_eq!(clips[2].get_start_time().as_secs(), 25);
    }

    #[test]
    fn test_verified_video_from_anonymous_video_invalid() {
        let brief = crate::model::VideoBrief::self_a();
        let anonymous_clips = vec![
            crate::model::AnonymousClip::self_a_1(),
            crate::model::AnonymousClip::self_a_2(),
            crate::model::AnonymousClip::self_a_3(),
        ];
        let anonymous_video = crate::model::AnonymousVideo::new(brief, anonymous_clips)
            .expect("should create valid AnonymousVideo");
        let result = VerifiedVideo::from_anonymous_video(
            anonymous_video,
            // 動画IDが違う
            crate::model::VideoDetail::self_b(),
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
            video_detail: crate::model::VideoDetail::self_a(),
            clips: vec![
                crate::model::VerifiedClip::self_a_1(),
                crate::model::VerifiedClip::self_a_2(),
            ],
        };
        let modified_at = chrono::Utc.with_ymd_and_hms(2025, 8, 8, 8, 8, 8).unwrap();
        let new_detail =
            crate::model::VideoDetail::self_a().update_modified_at(modified_at);
        let updated_video = verified_video
            .with_new_video_detail(new_detail.clone())
            .expect("should update video detail");

        assert_eq!(updated_video.video_detail, new_detail);
        assert_eq!(updated_video.clips.len(), 2);
    }

    #[test]
    fn test_verified_video_with_new_video_detail_modified() {
        use chrono::TimeZone;
        let verified_video = VerifiedVideo {
            video_detail: crate::model::VideoDetail::self_a(),
            clips: vec![
                crate::model::VerifiedClip::self_a_1(),
                crate::model::VerifiedClip::self_a_2(),
            ],
        };
        let modified_at = chrono::Utc.with_ymd_and_hms(2025, 8, 8, 8, 8, 8).unwrap();
        let new_detail = crate::model::VideoDetail::self_a()
            .update_modified_at(modified_at)
            // 変更を加える
            .set_duration(crate::model::Duration::from_secs(1));
        // 変更を加えて動画時間が短くなり, クリップが無効になったとき
        let updated_video = verified_video.with_new_video_detail(new_detail.clone());
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
                .try_add(&crate::model::Duration::from_secs(100))
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
