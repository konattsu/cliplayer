/// `VerifiedVideo`を作ろうとしたときのエラー
#[derive(thiserror::Error, Debug)]
pub(crate) enum VerifiedVideoError {
    /// クリップの情報が不正
    #[error(
        "invalid clips found ({count}):\n\t{msgs}",
        count = .0.len(),
        msgs = .0.iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n\t"),
    )]
    InvalidClip(Vec<crate::model::VerifiedClipError>),
    /// 動画IDが一致しない
    #[error("video_id mismatch: expected {local}, got {fetched}")]
    VideoIdMismatch {
        local: crate::model::VideoId,
        fetched: crate::model::VideoId,
    },
    /// クリップの範囲が重複
    #[error("clips overlap in video ID {id}: song titles {clips_title:?}")]
    ClipsOverlap {
        id: crate::model::VideoId,
        clips_title: Vec<String>,
    },
    /// クリップが存在しない
    #[error("no clips found for video ID {0}")]
    NoClips(crate::model::VideoId),
    /// 動画のapiから取得できる詳細情報が欠如
    #[error("missing api info for video ID {0}")]
    MissingApiInfo(crate::model::VideoId),
}

/// 複数の`VerifiedVideoError`をまとめたもの
#[derive(Debug)]
pub struct VerifiedVideoErrors {
    errs: Vec<VerifiedVideoError>,
}

impl std::error::Error for VerifiedVideoErrors {}

impl std::fmt::Display for VerifiedVideoErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.errs
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
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
    /// `local`と`fetched`の動画idが一致するか確認
    pub(super) fn ensure_video_id_match(
        local: &crate::model::VideoId,
        fetched: &crate::model::VideoId,
    ) -> Result<(), Self> {
        if local == fetched {
            Ok(())
        } else {
            Err(VerifiedVideoError::VideoIdMismatch {
                local: local.clone(),
                fetched: fetched.clone(),
            })
        }
    }
}
