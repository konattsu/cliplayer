pub mod artist_data;
mod artist_id;
mod color;
mod string_non_empty;

pub(in crate::artist) use artist_data::Artists;
pub(in crate::artist) use artist_id::ArtistId;
pub(in crate::artist) use color::Color;
pub(in crate::artist) use string_non_empty::StringNonEmpty;
