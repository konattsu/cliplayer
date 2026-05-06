mod channel_id;
mod duration;
mod privacy_status;
mod tag;
mod uploader_name;
mod uuid;
mod video;
mod video_id;
mod video_published_at;

pub mod clip;

pub(crate) use channel_id::ChannelId;
pub(crate) use clip::{AnonymousClip, UnverifiedClip, VerifiedClipError};
pub(crate) use duration::Duration;
pub(crate) use privacy_status::PrivacyStatus;
pub(crate) use tag::VideoTagIds;
pub(crate) use uploader_name::UploaderName;
pub(crate) use video::{
    AnonymousVideo, AnonymousVideos, ApiVideoInfo, ApiVideoInfoInitializer,
    ApiVideoInfoList, LocalVideoInfo, VerifiedVideoError, VerifiedVideoErrors,
    VideoRecord,
};
pub(crate) use video_published_at::VideoPublishedAt;

#[cfg(any(test, feature = "test-helpers"))]
pub use uuid::UuidVer4;
#[cfg(not(any(test, feature = "test-helpers")))]
pub(crate) use uuid::UuidVer4;

#[cfg(any(test, feature = "test-helpers"))]
pub use video_id::{VideoId, VideoIds};
#[cfg(not(any(test, feature = "test-helpers")))]
pub(crate) use video_id::{VideoId, VideoIds};

pub use clip::VerifiedClip;
pub use video::{VerifiedVideo, VerifiedVideos};
