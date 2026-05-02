mod assemble;
mod dictionaries;
mod load;
mod normalize;

pub use assemble::build_search_index;

#[cfg(test)]
mod tests;
