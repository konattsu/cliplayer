/// 適切な音量を表す
///
/// - `0-100%` で0は含まない, 100は含む
///   - Ok: `42`, `1`, `50`, `100`, etc.
///   - Err: `0`, `0.1`, `101`, etc.
#[derive(serde::Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct VolumePercent(std::num::NonZeroU8);

impl VolumePercent {
    fn validate_range(value: u8) -> Result<(), String> {
        if (1..=100).contains(&value) {
            Ok(())
        } else {
            Err(format!(
                "Invalid volume percent: {value}. Must be in range (0, 100].",
            ))
        }
    }
}

impl<'de> serde::Deserialize<'de> for VolumePercent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        VolumePercent::validate_range(value).map_err(serde::de::Error::custom)?;

        // 0でないのでunwrapする
        Ok(VolumePercent(std::num::NonZeroU8::new(value).unwrap()))
    }
}

// MARK: For Tests

#[cfg(test)]
impl VolumePercent {
    fn get(&self) -> u8 {
        self.0.get()
    }
}

// MARK: Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn volume_percent_deserialize_valid() {
        for v in [1u8, 42, 100] {
            let json = format!("{v}");
            let vp: VolumePercent = serde_json::from_str(&json).unwrap();
            assert_eq!(vp.get(), v);
        }
    }

    #[test]
    fn volume_percent_deserialize_invalid() {
        for v in [0u8, 101, 255] {
            let json = format!("{v}");
            let vp: Result<VolumePercent, _> = serde_json::from_str(&json);
            assert!(vp.is_err(), "{v} should be invalid for deserialization");
        }
    }
}
