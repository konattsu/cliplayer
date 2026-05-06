#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidFormat(&'static str),
    MissingSection(u32),
    DuplicateSection(u32),
    UnsupportedVersion(u32),
    UnsupportedRequiredFeatures(u64),
    UnsupportedEncoding(u32),
    Utf8,
    TooLarge(&'static str),
    Io(&'static str),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFormat(message) => {
                write!(f, "invalid index format: {message}")
            }
            Self::MissingSection(section_id) => {
                write!(f, "missing required section: 0x{section_id:04x}")
            }
            Self::DuplicateSection(section_id) => {
                write!(f, "duplicate section id: 0x{section_id:04x}")
            }
            Self::UnsupportedVersion(version) => {
                write!(f, "unsupported index format version: {version}")
            }
            Self::UnsupportedRequiredFeatures(bits) => {
                write!(f, "unsupported required features: 0x{bits:016x}")
            }
            Self::UnsupportedEncoding(encoding) => {
                write!(f, "unsupported section encoding: {encoding}")
            }
            Self::Utf8 => write!(f, "invalid utf-8 in section payload"),
            Self::TooLarge(message) => {
                write!(f, "value too large for v1 format: {message}")
            }
            Self::Io(message) => write!(f, "io error while encoding index: {message}"),
        }
    }
}

impl std::error::Error for Error {}
