#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DatasetBuildId(String);

impl DatasetBuildId {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn is_valid(value: &str) -> bool {
        let bytes = value.as_bytes();
        if !(8..=128).contains(&bytes.len()) {
            return false;
        }

        fn is_head(byte: u8) -> bool {
            byte.is_ascii_digit() || byte.is_ascii_lowercase()
        }

        fn is_tail(byte: u8) -> bool {
            is_head(byte) || matches!(byte, b'.' | b'_' | b'-')
        }

        is_head(bytes[0]) && bytes[1..].iter().copied().all(is_tail)
    }
}

impl std::fmt::Display for DatasetBuildId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for DatasetBuildId {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(value) {
            Ok(Self(value.to_string()))
        } else {
            Err("dataset_build_id must match ^[a-z0-9][a-z0-9._-]{7,127}$".to_string())
        }
    }
}

#[derive(Debug, Clone)]
pub struct BuildMetadata {
    dataset_build_id: DatasetBuildId,
}

impl BuildMetadata {
    pub const SCHEMA_VERSION: u32 = 1;

    pub fn new(dataset_build_id: DatasetBuildId) -> Self {
        Self { dataset_build_id }
    }

    pub fn dataset_build_id(&self) -> &str {
        self.dataset_build_id.as_str()
    }
}

#[derive(Debug, Clone)]
pub struct InputSetHashBuilder {
    hasher: sha2::Sha256,
}

impl InputSetHashBuilder {
    pub fn new(domain: &str) -> Self {
        use sha2::Digest as _;

        let mut hasher = sha2::Sha256::new();
        hasher.update(domain.as_bytes());
        hasher.update(b"\0");
        Self { hasher }
    }

    pub fn add_entry(&mut self, logical_path: &str, bytes: &[u8]) {
        update_len_prefixed(&mut self.hasher, logical_path.as_bytes());
        update_len_prefixed(&mut self.hasher, bytes);
    }

    pub fn add_file(
        &mut self,
        logical_path: &str,
        path: &std::path::Path,
    ) -> Result<(), std::io::Error> {
        let bytes = std::fs::read(path)?;
        self.add_entry(logical_path, &bytes);
        Ok(())
    }

    pub fn add_serializable<T>(
        &mut self,
        logical_path: &str,
        value: &T,
    ) -> Result<(), serde_json::Error>
    where
        T: serde::Serialize,
    {
        let bytes = serde_json::to_vec(value)?;
        self.add_entry(logical_path, &bytes);
        Ok(())
    }

    pub fn finish_hex(self) -> String {
        use sha2::Digest as _;

        let digest = self.hasher.finalize();
        to_hex(&digest)
    }
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::Digest as _;

    let digest = sha2::Sha256::digest(bytes);
    to_hex(&digest)
}

fn update_len_prefixed(hasher: &mut sha2::Sha256, bytes: &[u8]) {
    use sha2::Digest as _;

    hasher.update((bytes.len() as u64).to_le_bytes());
    hasher.update(bytes);
}

fn to_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";

    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MinEnvelope<'a, T>
where
    T: serde::Serialize,
{
    schema_version: u32,
    dataset_build_id: &'a str,
    data: &'a T,
}

impl<'a, T> MinEnvelope<'a, T>
where
    T: serde::Serialize,
{
    pub fn new(data: &'a T, metadata: &'a BuildMetadata) -> Self {
        Self {
            schema_version: BuildMetadata::SCHEMA_VERSION,
            dataset_build_id: metadata.dataset_build_id(),
            data,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_dataset_build_id_accepts_valid_values() {
        let cases = [
            "20260508t143210z-abcd1234",
            "manual-20260508-01",
            "release.2026_05_08",
        ];

        for case in cases {
            let actual = case.parse::<super::DatasetBuildId>().unwrap();
            assert_eq!(actual.as_str(), case);
        }
    }

    #[test]
    fn test_dataset_build_id_rejects_invalid_values() {
        let cases = [
            "short7",
            "UPPERCASE-allowed",
            "_leading_underscore",
            "contains/slash",
            "space value",
        ];

        for case in cases {
            assert!(case.parse::<super::DatasetBuildId>().is_err());
        }
    }

    #[test]
    fn test_input_set_hash_builder_is_stable_for_same_entries() {
        let mut left = super::InputSetHashBuilder::new("test-domain");
        left.add_entry("a.json", br#"{"a":1}"#);
        left.add_entry("b.json", br#"{"b":2}"#);

        let mut right = super::InputSetHashBuilder::new("test-domain");
        right.add_entry("a.json", br#"{"a":1}"#);
        right.add_entry("b.json", br#"{"b":2}"#);

        assert_eq!(left.finish_hex(), right.finish_hex());
    }

    #[test]
    fn test_input_set_hash_builder_changes_when_logical_path_changes() {
        let mut left = super::InputSetHashBuilder::new("test-domain");
        left.add_entry("a.json", br#"{"a":1}"#);

        let mut right = super::InputSetHashBuilder::new("test-domain");
        right.add_entry("other.json", br#"{"a":1}"#);

        assert_ne!(left.finish_hex(), right.finish_hex());
    }
}
