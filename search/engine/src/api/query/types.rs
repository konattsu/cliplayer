/// sort 対象のフィールド。
///
/// 現状の初版では `published_at` だけをサポートする。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SortField {
    PublishedAt,
}

/// sort 順序。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SortOrder {
    Asc,
    Desc,
}

/// 総件数の返却方針。
///
/// `Exact` はページ返却に必要な件数が見つかった後も評価を継続し、
/// 一致件数の総数を返す。
/// `None` は総件数を返さず、ページ生成に十分な件数が見つかった時点で
/// 早期停止してよい。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TotalMode {
    Exact,
    None,
}

/// 範囲条件の片側境界。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RangeBound {
    pub value: index::schema::TimestampSecs,
    pub inclusive: bool,
}

/// 日時範囲。
///
/// `lower` と `upper` の両方を省略した場合は無制限の範囲を表す。
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct DateRange {
    pub lower: Option<RangeBound>,
    pub upper: Option<RangeBound>,
}
