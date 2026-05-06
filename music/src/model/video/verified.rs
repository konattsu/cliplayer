mod error;
mod video;
mod videos;

pub(crate) use error::{VerifiedVideoError, VerifiedVideoErrors};
pub use video::VerifiedVideo;
pub use videos::VerifiedVideos;
