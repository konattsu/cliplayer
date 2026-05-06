#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum SearchErrorCode {
    InvalidRequest,
    InvalidCursor,
    QueryTooComplex,
    CorruptIndex,
    VersionMismatch,
    UnsupportedFeature,
    InternalIndex,
    Binary,
    Internal,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub(crate) struct SearchError {
    pub(crate) code: SearchErrorCode,
    pub(crate) message: String,
}

impl SearchError {
    pub(crate) fn invalid_request(message: impl Into<String>) -> Self {
        Self {
            code: SearchErrorCode::InvalidRequest,
            message: message.into(),
        }
    }

    pub(crate) fn invalid_cursor(message: impl Into<String>) -> Self {
        Self {
            code: SearchErrorCode::InvalidCursor,
            message: message.into(),
        }
    }

    pub(crate) fn internal(message: impl Into<String>) -> Self {
        Self {
            code: SearchErrorCode::Internal,
            message: message.into(),
        }
    }

    pub(crate) fn into_js_value(self) -> wasm_bindgen::JsValue {
        match serde_wasm_bindgen::to_value(&self) {
            Ok(value) => value,
            Err(_) => wasm_bindgen::JsValue::from_str(&format!(
                "{}: {}",
                self.code.as_str(),
                self.message
            )),
        }
    }

    pub(crate) fn from_engine(error: engine::EngineError) -> Self {
        let code = match error {
            engine::EngineError::InvalidRequest(_) => SearchErrorCode::InvalidRequest,
            engine::EngineError::InvalidCursor(_) => SearchErrorCode::InvalidCursor,
            engine::EngineError::QueryTooComplex(_) => SearchErrorCode::QueryTooComplex,
            engine::EngineError::CorruptIndex(_) => SearchErrorCode::CorruptIndex,
            engine::EngineError::VersionMismatch(_) => SearchErrorCode::VersionMismatch,
            engine::EngineError::UnsupportedFeature(_) => {
                SearchErrorCode::UnsupportedFeature
            }
            engine::EngineError::InternalIndex(_) => SearchErrorCode::InternalIndex,
            engine::EngineError::Binary(_) => SearchErrorCode::Binary,
        };

        Self {
            code,
            message: error.to_string(),
        }
    }
}

impl SearchErrorCode {
    fn as_str(self) -> &'static str {
        match self {
            Self::InvalidRequest => "INVALID_REQUEST",
            Self::InvalidCursor => "INVALID_CURSOR",
            Self::QueryTooComplex => "QUERY_TOO_COMPLEX",
            Self::CorruptIndex => "CORRUPT_INDEX",
            Self::VersionMismatch => "VERSION_MISMATCH",
            Self::UnsupportedFeature => "UNSUPPORTED_FEATURE",
            Self::InternalIndex => "INTERNAL_INDEX",
            Self::Binary => "BINARY",
            Self::Internal => "INTERNAL",
        }
    }
}
