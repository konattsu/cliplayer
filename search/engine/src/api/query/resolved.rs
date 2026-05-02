/// 辞書解決後の検索条件 AST。
///
/// evaluator は文字列 ID を扱わず、解決済みの内部 ID だけを入力にする。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedQueryNode {
    And { children: Vec<ResolvedQueryNode> },
    Or { children: Vec<ResolvedQueryNode> },
    Not { child: Box<ResolvedQueryNode> },
    Term(ResolvedTermNode),
}

/// 辞書解決後の葉の検索条件。
///
/// `AnyIn.values` は空でなく、sort + dedup 済みであることを前提にしてよい。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedTermNode {
    ArtistAnyIn {
        values: Vec<index_core::schema::ids::ArtistId>,
    },
    TagAnyIn {
        values: Vec<index_core::schema::ids::TagId>,
    },
    ChannelAnyIn {
        values: Vec<index_core::schema::ids::ChannelId>,
    },
    IsUnlistedEq {
        value: bool,
    },
    EmbeddableEq {
        value: bool,
    },
    PublishedAtRange(crate::api::query::types::DateRange),
}
