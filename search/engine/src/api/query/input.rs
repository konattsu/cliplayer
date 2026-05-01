/// 検索条件の sort 指定。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SortSpec {
    pub field: crate::api::query::types::SortField,
    pub order: crate::api::query::types::SortOrder,
}

/// ページング条件。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageSpec {
    pub limit: u32,
    pub cursor: Option<crate::api::pagination::Cursor>,
}

/// 検索エンジンに渡すリクエスト。
///
/// frontend 側では `required_filter` と `user_query` を request 直前に
/// 1 本の `query` へ合成済みであることを前提にする。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchRequest {
    pub query: Option<QueryNode>,
    pub sort: Vec<SortSpec>,
    pub page: PageSpec,
    pub total_mode: crate::api::query::types::TotalMode,
}

/// 検索条件 AST。
///
/// parse 直後は任意の木構造を許すが、評価前には NNF
/// (Negation Normal Form) まで正規化してよい。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryNode {
    /// 子ノードの論理積。
    ///
    /// 正規化後は入れ子の `And` を flatten し、子の並び順は
    /// evaluator が扱いやすい順に並べ替えてよい。
    And {
        children: Vec<QueryNode>,
    },
    /// 子ノードの論理和。
    ///
    /// 正規化後は入れ子の `Or` を flatten し、子の並び順は
    /// evaluator が扱いやすい順に並べ替えてよい。
    Or {
        children: Vec<QueryNode>,
    },
    /// 否定。
    ///
    /// 評価前の正規化では次を適用してよい。
    /// - `Not(Not(x)) -> x`
    /// - `Not(And(xs)) -> Or(Not(x)...)`
    /// - `Not(Or(xs)) -> And(Not(x)...)`
    ///
    /// つまり最終的には `Not` が term 直上にだけ現れる NNF を
    /// 目標にする。
    Not {
        child: Box<QueryNode>,
    },
    Term(TermNode),
}

/// 葉の検索条件。
///
/// `any_in.values` は query 正規化時に以下を満たす形へそろえる。
/// - empty を禁止する
/// - sort 済み
/// - dedup 済み
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TermNode {
    ArtistAnyIn {
        values: Vec<index::schema::ids::ArtistIdString>,
    },
    TagAnyIn {
        values: Vec<index::schema::ids::TagIdString>,
    },
    ChannelAnyIn {
        values: Vec<index::schema::ids::ChannelIdString>,
    },
    IsUnlistedEq {
        value: bool,
    },
    EmbeddableEq {
        value: bool,
    },
    PublishedAtRange(crate::api::query::types::DateRange),
}
