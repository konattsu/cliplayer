/// UTC unix time seconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimestampSecs(i64);

impl From<i64> for TimestampSecs {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<u32> for TimestampSecs {
    fn from(value: u32) -> Self {
        Self(value as i64)
    }
}

impl From<TimestampSecs> for i64 {
    fn from(value: TimestampSecs) -> Self {
        value.0
    }
}
