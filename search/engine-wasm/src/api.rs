pub(crate) const API_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SearchRequest {
    #[serde(default)]
    pub(crate) api_version: Option<u32>,
    pub(crate) query: Option<QueryNode>,
    pub(crate) sort: SortSpec,
    pub(crate) page: PageSpec,
    pub(crate) total_mode: TotalMode,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SortSpec {
    pub(crate) field: SortField,
    pub(crate) order: SortOrder,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PageSpec {
    pub(crate) limit: u32,
    pub(crate) cursor: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum QueryNode {
    And { children: Vec<QueryNode> },
    Or { children: Vec<QueryNode> },
    Not { child: Box<QueryNode> },
    Term { term: TermNode },
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum TermNode {
    ArtistAnyIn { values: Vec<String> },
    TagAnyIn { values: Vec<String> },
    ChannelAnyIn { values: Vec<String> },
    IsUnlistedEq { value: bool },
    EmbeddableEq { value: bool },
    PublishedAtRange { range: DateRange },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SortField {
    PublishedAt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum TotalMode {
    Exact,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct DateRange {
    pub(crate) lower: Option<RangeBound>,
    pub(crate) upper: Option<RangeBound>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RangeBound {
    pub(crate) value: i64,
    pub(crate) inclusive: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub(crate) struct SearchResponse {
    pub(crate) clip_uuids: Vec<String>,
    pub(crate) next_cursor: Option<String>,
    pub(crate) total_mode: TotalMode,
    pub(crate) total: Option<u32>,
    pub(crate) has_more: bool,
    pub(crate) warnings: Vec<QueryWarning>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)] // serializeしてフロントで見たときにわかりやすいように
pub(crate) enum QueryWarning {
    UnknownArtistId { value: String },
    UnknownTagId { value: String },
    UnknownChannelId { value: String },
}

impl SearchRequest {
    pub(crate) fn into_engine(
        self,
    ) -> Result<engine::api::query::input::SearchRequest, crate::error::SearchError>
    {
        if let Some(version) = self.api_version
            && version != API_VERSION
        {
            return Err(crate::error::SearchError::invalid_request(format!(
                "unsupported api_version: expected {API_VERSION}, got {version}",
            )));
        }

        Ok(engine::api::query::input::SearchRequest {
            query: self.query.map(QueryNode::into_engine),
            sort: vec![engine::api::query::input::SortSpec {
                field: self.sort.field.into_engine(),
                order: self.sort.order.into_engine(),
            }],
            page: engine::api::query::input::PageSpec {
                limit: self.page.limit,
                cursor: self
                    .page
                    .cursor
                    .as_deref()
                    .map(crate::cursor::decode)
                    .transpose()?,
            },
            total_mode: self.total_mode.into_engine(),
        })
    }
}

impl QueryNode {
    fn into_engine(self) -> engine::api::query::input::QueryNode {
        match self {
            Self::And { children } => engine::api::query::input::QueryNode::And {
                children: children.into_iter().map(QueryNode::into_engine).collect(),
            },
            Self::Or { children } => engine::api::query::input::QueryNode::Or {
                children: children.into_iter().map(QueryNode::into_engine).collect(),
            },
            Self::Not { child } => engine::api::query::input::QueryNode::Not {
                child: Box::new(child.into_engine()),
            },
            Self::Term { term } => {
                engine::api::query::input::QueryNode::Term(term.into_engine())
            }
        }
    }
}

impl TermNode {
    fn into_engine(self) -> engine::api::query::input::TermNode {
        match self {
            Self::ArtistAnyIn { values } => {
                engine::api::query::input::TermNode::ArtistAnyIn { values }
            }
            Self::TagAnyIn { values } => {
                engine::api::query::input::TermNode::TagAnyIn { values }
            }
            Self::ChannelAnyIn { values } => {
                engine::api::query::input::TermNode::ChannelAnyIn { values }
            }
            Self::IsUnlistedEq { value } => {
                engine::api::query::input::TermNode::IsUnlistedEq { value }
            }
            Self::EmbeddableEq { value } => {
                engine::api::query::input::TermNode::EmbeddableEq { value }
            }
            Self::PublishedAtRange { range } => {
                engine::api::query::input::TermNode::PublishedAtRange(
                    range.into_engine(),
                )
            }
        }
    }
}

impl SortField {
    pub(crate) fn into_engine(self) -> engine::api::query::types::SortField {
        match self {
            Self::PublishedAt => engine::api::query::types::SortField::PublishedAt,
        }
    }

    pub(crate) fn from_engine(field: engine::api::query::types::SortField) -> Self {
        match field {
            engine::api::query::types::SortField::PublishedAt => Self::PublishedAt,
        }
    }
}

impl SortOrder {
    pub(crate) fn into_engine(self) -> engine::api::query::types::SortOrder {
        match self {
            Self::Asc => engine::api::query::types::SortOrder::Asc,
            Self::Desc => engine::api::query::types::SortOrder::Desc,
        }
    }

    pub(crate) fn from_engine(order: engine::api::query::types::SortOrder) -> Self {
        match order {
            engine::api::query::types::SortOrder::Asc => Self::Asc,
            engine::api::query::types::SortOrder::Desc => Self::Desc,
        }
    }
}

impl TotalMode {
    fn into_engine(self) -> engine::api::query::types::TotalMode {
        match self {
            Self::Exact => engine::api::query::types::TotalMode::Exact,
            Self::None => engine::api::query::types::TotalMode::None,
        }
    }

    fn from_engine(mode: engine::api::query::types::TotalMode) -> Self {
        match mode {
            engine::api::query::types::TotalMode::Exact => Self::Exact,
            engine::api::query::types::TotalMode::None => Self::None,
        }
    }
}

impl DateRange {
    fn into_engine(self) -> engine::api::query::types::DateRange {
        engine::api::query::types::DateRange {
            lower: self.lower.map(RangeBound::into_engine),
            upper: self.upper.map(RangeBound::into_engine),
        }
    }
}

impl RangeBound {
    fn into_engine(self) -> engine::api::query::types::RangeBound {
        engine::api::query::types::RangeBound {
            value: self.value.into(),
            inclusive: self.inclusive,
        }
    }
}

impl SearchResponse {
    pub(crate) fn from_engine(
        response: engine::api::response::SearchResponse,
    ) -> Result<Self, crate::error::SearchError> {
        let next_cursor = response
            .next_cursor
            .as_ref()
            .map(crate::cursor::encode)
            .transpose()?;

        if response.has_more != next_cursor.is_some() {
            return Err(crate::error::SearchError::internal(
                "engine response violated has_more/next_cursor invariant",
            ));
        }

        Ok(Self {
            clip_uuids: response.clip_uuids,
            next_cursor,
            total_mode: TotalMode::from_engine(response.total_mode),
            total: response.total,
            has_more: response.has_more,
            warnings: response
                .warnings
                .into_iter()
                .map(QueryWarning::from_engine)
                .collect(),
        })
    }
}

impl QueryWarning {
    fn from_engine(warning: engine::api::response::QueryWarning) -> Self {
        match warning {
            engine::api::response::QueryWarning::UnknownArtistId { value } => {
                Self::UnknownArtistId { value }
            }
            engine::api::response::QueryWarning::UnknownTagId { value } => {
                Self::UnknownTagId { value }
            }
            engine::api::response::QueryWarning::UnknownChannelId { value } => {
                Self::UnknownChannelId { value }
            }
        }
    }
}
