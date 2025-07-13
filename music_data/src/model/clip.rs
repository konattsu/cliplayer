mod anonymous;
mod unverified;
mod verified;

pub use anonymous::{AnonymousClip, AnonymousClipInitializer};
pub use unverified::{UnverifiedClip, UnverifiedClipError, UnverifiedClipInitializer};
pub use verified::{VerifiedClip, VerifiedClipError, VerifiedClipInitializer};

/// `start_time` < `end_time` の検証
///
/// - Ok: `start_time` < `end_time`のとき
/// - Error: `start_time` >= `end_time`のとき
fn validate_start_end_times(
    start_time: &crate::model::Duration,
    end_time: &crate::model::Duration,
) -> Result<(), String> {
    if start_time >= end_time {
        return Err(format!(
            "invalid clip time range: start({start_time}) must be less than to end({end_time})",
        ));
    }
    Ok(())
}
