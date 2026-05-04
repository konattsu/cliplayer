mod assemble;
mod dictionaries;
mod load;
mod normalize;

pub use assemble::build_search_index;
pub use assemble::build_search_index_binary;

#[cfg(test)]
mod tests;
