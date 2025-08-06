#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(in crate::artist) struct StringNonEmpty(String);

impl StringNonEmpty {
    pub fn new(s: String) -> Option<Self> {
        if s.is_empty() { None } else { Some(Self(s)) }
    }
}

impl std::fmt::Display for StringNonEmpty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> serde::Deserialize<'de> for StringNonEmpty {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        StringNonEmpty::new(s)
            .ok_or_else(|| serde::de::Error::custom("String cannot be empty"))
    }
}
