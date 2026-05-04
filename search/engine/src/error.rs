#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineError {
    CorruptIndex(&'static str),
    VersionMismatch(&'static str),
    UnsupportedFeature(&'static str),
    InvalidRequest(&'static str),
    InvalidCursor(&'static str),
    QueryTooComplex(&'static str),
    InternalIndex(&'static str),
    Binary(index_core::binary::Error),
}

impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CorruptIndex(message) => write!(f, "corrupt index: {message}"),
            Self::VersionMismatch(message) => {
                write!(f, "index version mismatch: {message}")
            }
            Self::UnsupportedFeature(message) => {
                write!(f, "unsupported feature: {message}")
            }
            Self::InvalidRequest(message) => write!(f, "invalid request: {message}"),
            Self::InvalidCursor(message) => write!(f, "invalid cursor: {message}"),
            Self::QueryTooComplex(message) => {
                write!(f, "query too complex: {message}")
            }
            Self::InternalIndex(message) => {
                write!(f, "internal index error: {message}")
            }
            Self::Binary(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for EngineError {}

impl From<index_core::binary::Error> for EngineError {
    fn from(value: index_core::binary::Error) -> Self {
        use index_core::binary;

        match value {
            binary::Error::UnsupportedVersion(_) => {
                Self::VersionMismatch("unsupported index format version")
            }
            binary::Error::UnsupportedRequiredFeatures(_)
            | binary::Error::UnsupportedEncoding(_) => {
                Self::UnsupportedFeature("binary index contains unsupported feature")
            }
            binary::Error::MissingSection(_)
            | binary::Error::DuplicateSection(_)
            | binary::Error::Utf8
            | binary::Error::InvalidFormat(_) => {
                Self::CorruptIndex("binary index failed structural validation")
            }
            other => Self::Binary(other),
        }
    }
}
