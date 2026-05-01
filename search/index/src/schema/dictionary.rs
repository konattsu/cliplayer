/// 辞書。
///
/// 内部検索で高速化するために、文字列 ID を整数 ID に変換して扱う。
/// その変換表をまとめたもの。
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Dictionaries {
    pub clips: crate::util::BiMap<crate::schema::ids::ClipId>,
    pub videos: crate::util::BiMap<crate::schema::ids::VideoId>,
    pub channels: crate::util::BiMap<crate::schema::ids::ChannelId>,
    pub artists: crate::util::BiMap<crate::schema::ids::ArtistId>,
    pub tags: crate::util::BiMap<crate::schema::ids::TagId>,
}
