/// ページング再開位置を表す cursor。
///
/// 現状の search index では `doc_id` から sort key の値を O(1) で引けるため、
/// cursor 自体は `doc_id` だけを持てばよい。
///
/// 将来、sort の種類ごとに再開位置に必要な payload が変わるなら、
/// `Cursor` を enum にして sort ごとの variant に `doc_id` 以外の値を
/// 持たせる設計も有用である。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cursor {
    pub doc_id: index::schema::ids::DocId,
}
