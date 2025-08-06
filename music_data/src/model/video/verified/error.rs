/// `VerifiedVideo`を作ろうとしたときのエラー
#[derive(Debug)]
pub(crate) enum VerifiedVideoError {
    /// クリップの情報が不正
    InvalidClip(Vec<crate::model::VerifiedClipError>),
    /// 動画IDが一致しない
    VideoIdMismatch {
        local: crate::model::VideoId,
        fetched: crate::model::VideoId,
    },
    /// クリップの範囲が重複
    ClipsOverlap {
        id: crate::model::VideoId,
        clips_title: Vec<String>,
    },
    /// クリップが存在しない
    NoClips(crate::model::VideoId),
    /// 動画のapiから取得できる詳細情報が欠如
    MissingApiInfo(crate::model::VideoId),
}

/// `VerifiedVideo`を作ろうとしたときのエラー
#[derive(Debug)]
pub(crate) struct VerifiedVideoErrors {
    errs: Vec<VerifiedVideoError>,
}

impl From<Vec<VerifiedVideoError>> for VerifiedVideoErrors {
    fn from(value: Vec<VerifiedVideoError>) -> Self {
        Self { errs: value }
    }
}

impl From<VerifiedVideoErrors> for Vec<VerifiedVideoError> {
    fn from(value: VerifiedVideoErrors) -> Self {
        value.errs
    }
}

impl VerifiedVideoError {
    pub(super) fn ensure_video_id_match(
        expected: &crate::model::VideoId,
        actual: &crate::model::VideoId,
    ) -> Result<(), Self> {
        if expected == actual {
            Ok(())
        } else {
            Err(VerifiedVideoError::VideoIdMismatch {
                local: expected.clone(),
                fetched: actual.clone(),
            })
        }
    }

    /// 成形して表示する用の文字列をつくる
    pub fn to_pretty_string(&self) -> String {
        let mut msg = "Failed to create VerifiedVideo: ".to_string();
        match self {
            Self::VideoIdMismatch {
                local: brief,
                fetched,
            } => {
                msg.push_str(&format!(
                    "video_id mismatch: expected {brief}, got {fetched}",
                ));
            }
            Self::InvalidClip(errors) => {
                let invalid_clip_err_msgs =
                    errors.iter().map(|e| e.to_string()).collect::<Vec<_>>();
                msg.push_str(&format!(
                    "Invalid clips found ({}):\n\t{}",
                    errors.len(),
                    invalid_clip_err_msgs.join("\n\t")
                ));
            }
            Self::ClipsOverlap { id, clips_title } => {
                msg.push_str(&format!(
                    "Clips overlap in video ID {id}: song titles`{clips_title:?}`"
                ));
            }
            Self::NoClips(id) => {
                msg.push_str(&format!("No clips found for video ID {id}"));
            }
            Self::MissingApiInfo(id) => {
                msg.push_str(&format!("Missing api info for video ID {id}"));
            }
        }
        msg
    }
}

impl VerifiedVideoErrors {
    pub fn to_pretty_string(&self) -> String {
        let mut err_str = String::new();
        for e in self.errs.iter() {
            err_str.push_str(&e.to_pretty_string());
        }
        err_str
    }
}
