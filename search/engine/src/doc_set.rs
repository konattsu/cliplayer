#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SortedDocIds(Vec<index_core::schema::ids::DocId>);

impl SortedDocIds {
    pub(crate) fn new(
        doc_ids: Vec<index_core::schema::ids::DocId>,
        record_count: u32,
    ) -> Result<Option<Self>, crate::EngineError> {
        if doc_ids.is_empty() {
            return Ok(None);
        }

        let mut previous = None;
        for &doc_id in &doc_ids {
            if doc_id >= record_count {
                return Err(crate::EngineError::InternalIndex(
                    "sorted doc ids out of range",
                ));
            }
            if previous.is_some_and(|prev| prev >= doc_id) {
                return Err(crate::EngineError::InternalIndex(
                    "sorted doc ids must be strictly increasing",
                ));
            }
            previous = Some(doc_id);
        }

        Ok(Some(Self(doc_ids)))
    }

    pub(crate) fn as_slice(&self) -> &[index_core::schema::ids::DocId] {
        &self.0
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DocSet {
    All,
    Empty,
    SortedDocIds(SortedDocIds),
    BitSet(Vec<u64>),
}

impl DocSet {
    pub(crate) fn from_sorted_doc_ids(
        doc_ids: Vec<index_core::schema::ids::DocId>,
        record_count: u32,
    ) -> Result<Self, crate::EngineError> {
        Ok(match SortedDocIds::new(doc_ids, record_count)? {
            Some(doc_ids) => Self::SortedDocIds(doc_ids),
            None => Self::Empty,
        })
    }

    pub(crate) fn from_unsorted_doc_ids(
        mut doc_ids: Vec<index_core::schema::ids::DocId>,
        record_count: u32,
    ) -> Result<Self, crate::EngineError> {
        doc_ids.sort_unstable();
        doc_ids.dedup();
        Self::from_sorted_doc_ids(doc_ids, record_count)
    }

    pub(crate) fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    pub(crate) fn contains(&self, doc_id: index_core::schema::ids::DocId) -> bool {
        match self {
            Self::All => true,
            Self::Empty => false,
            Self::SortedDocIds(doc_ids) => {
                doc_ids.as_slice().binary_search(&doc_id).is_ok()
            }
            Self::BitSet(bits) => {
                let word_index = doc_id as usize / 64;
                let bit_index = doc_id as usize % 64;
                bits.get(word_index)
                    .is_some_and(|word| (word & (1u64 << bit_index)) != 0)
            }
        }
    }

    pub(crate) fn count(&self, record_count: u32) -> u32 {
        match self {
            Self::All => record_count,
            Self::Empty => 0,
            Self::SortedDocIds(doc_ids) => doc_ids.len() as u32,
            Self::BitSet(bits) => {
                let mut total = 0u32;
                for &word in &mask_tail_bits(bits.clone(), record_count) {
                    total += word.count_ones();
                }
                total
            }
        }
    }

    pub(crate) fn to_bitset(&self, record_count: u32) -> Vec<u64> {
        match self {
            Self::All => all_bits(record_count),
            Self::Empty => vec![0; word_len(record_count)],
            Self::SortedDocIds(doc_ids) => {
                let mut bits = vec![0; word_len(record_count)];
                for &doc_id in doc_ids.as_slice() {
                    set_bit(&mut bits, doc_id);
                }
                bits
            }
            Self::BitSet(bits) => mask_tail_bits(bits.clone(), record_count),
        }
    }

    pub(crate) fn intersect(
        left: &Self,
        right: &Self,
        record_count: u32,
    ) -> Result<Self, crate::EngineError> {
        match (left, right) {
            (Self::Empty, _) | (_, Self::Empty) => Ok(Self::Empty),
            (Self::All, other) | (other, Self::All) => Ok(other.clone()),
            (Self::SortedDocIds(left), Self::SortedDocIds(right)) => {
                let mut out = Vec::new();
                let mut left_index = 0;
                let mut right_index = 0;
                while left_index < left.len() && right_index < right.len() {
                    let left_doc = left.as_slice()[left_index];
                    let right_doc = right.as_slice()[right_index];
                    match left_doc.cmp(&right_doc) {
                        std::cmp::Ordering::Less => left_index += 1,
                        std::cmp::Ordering::Greater => right_index += 1,
                        std::cmp::Ordering::Equal => {
                            out.push(left_doc);
                            left_index += 1;
                            right_index += 1;
                        }
                    }
                }
                Self::from_sorted_doc_ids(out, record_count)
            }
            _ => {
                let mut out = left.to_bitset(record_count);
                for (target, mask) in out.iter_mut().zip(right.to_bitset(record_count))
                {
                    *target &= mask;
                }
                Ok(bitset_to_doc_set(out, record_count))
            }
        }
    }

    pub(crate) fn union(
        left: &Self,
        right: &Self,
        record_count: u32,
    ) -> Result<Self, crate::EngineError> {
        match (left, right) {
            (Self::All, _) | (_, Self::All) => Ok(Self::All),
            (Self::Empty, other) | (other, Self::Empty) => Ok(other.clone()),
            (Self::SortedDocIds(left), Self::SortedDocIds(right)) => {
                let mut out = Vec::with_capacity(left.len() + right.len());
                let mut left_index = 0;
                let mut right_index = 0;
                while left_index < left.len() || right_index < right.len() {
                    match (
                        left.as_slice().get(left_index),
                        right.as_slice().get(right_index),
                    ) {
                        (Some(&left_doc), Some(&right_doc)) => {
                            match left_doc.cmp(&right_doc) {
                                std::cmp::Ordering::Less => {
                                    out.push(left_doc);
                                    left_index += 1;
                                }
                                std::cmp::Ordering::Greater => {
                                    out.push(right_doc);
                                    right_index += 1;
                                }
                                std::cmp::Ordering::Equal => {
                                    out.push(left_doc);
                                    left_index += 1;
                                    right_index += 1;
                                }
                            }
                        }
                        (Some(&left_doc), None) => {
                            out.push(left_doc);
                            left_index += 1;
                        }
                        (None, Some(&right_doc)) => {
                            out.push(right_doc);
                            right_index += 1;
                        }
                        (None, None) => break,
                    }
                }
                Self::from_sorted_doc_ids(out, record_count)
            }
            _ => {
                let mut out = left.to_bitset(record_count);
                for (target, mask) in out.iter_mut().zip(right.to_bitset(record_count))
                {
                    *target |= mask;
                }
                Ok(bitset_to_doc_set(out, record_count))
            }
        }
    }

    pub(crate) fn difference(
        left: &Self,
        right: &Self,
        record_count: u32,
    ) -> Result<Self, crate::EngineError> {
        match (left, right) {
            (Self::Empty, _) => Ok(Self::Empty),
            (_, Self::Empty) => Ok(left.clone()),
            (Self::All, other) => {
                let mut out = all_bits(record_count);
                for (target, mask) in out.iter_mut().zip(other.to_bitset(record_count))
                {
                    *target &= !mask;
                }
                Ok(bitset_to_doc_set(out, record_count))
            }
            (Self::SortedDocIds(left), Self::SortedDocIds(right)) => {
                let mut out = Vec::with_capacity(left.len());
                let mut left_index = 0;
                let mut right_index = 0;
                while left_index < left.len() {
                    let left_doc = left.as_slice()[left_index];
                    match right.as_slice().get(right_index).copied() {
                        Some(right_doc) => match left_doc.cmp(&right_doc) {
                            std::cmp::Ordering::Less => {
                                out.push(left_doc);
                                left_index += 1;
                            }
                            std::cmp::Ordering::Greater => right_index += 1,
                            std::cmp::Ordering::Equal => {
                                left_index += 1;
                                right_index += 1;
                            }
                        },
                        None => {
                            out.extend_from_slice(&left.as_slice()[left_index..]);
                            break;
                        }
                    }
                }
                Self::from_sorted_doc_ids(out, record_count)
            }
            _ => {
                let mut out = left.to_bitset(record_count);
                for (target, mask) in out.iter_mut().zip(right.to_bitset(record_count))
                {
                    *target &= !mask;
                }
                Ok(bitset_to_doc_set(out, record_count))
            }
        }
    }
}

pub(crate) fn word_len(record_count: u32) -> usize {
    record_count.div_ceil(64) as usize
}

pub(crate) fn bitset_byte_len(record_count: u32) -> usize {
    word_len(record_count) * std::mem::size_of::<u64>()
}

pub(crate) fn set_bit(bits: &mut [u64], doc_id: index_core::schema::ids::DocId) {
    let word_index = doc_id as usize / 64;
    let bit_index = doc_id as usize % 64;
    bits[word_index] |= 1u64 << bit_index;
}

fn all_bits(record_count: u32) -> Vec<u64> {
    let bits = vec![u64::MAX; word_len(record_count)];
    mask_tail_bits(bits, record_count)
}

fn mask_tail_bits(mut bits: Vec<u64>, record_count: u32) -> Vec<u64> {
    if let Some(last) = bits.last_mut() {
        let remainder = record_count as usize % 64;
        if remainder != 0 {
            *last &= (1u64 << remainder) - 1;
        }
    }
    bits
}

fn bitset_to_doc_set(bits: Vec<u64>, record_count: u32) -> DocSet {
    let bits = mask_tail_bits(bits, record_count);
    if bits.iter().all(|word| *word == 0) {
        DocSet::Empty
    } else {
        DocSet::BitSet(bits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doc_set_masks_tail_bits_in_count() {
        let set = DocSet::BitSet(vec![u64::MAX]);
        assert_eq!(set.count(3), 3);
    }

    #[test]
    fn test_sorted_doc_ids_rejects_duplicates() {
        let err = SortedDocIds::new(vec![1, 1], 3).unwrap_err();
        assert_eq!(
            err,
            crate::EngineError::InternalIndex(
                "sorted doc ids must be strictly increasing",
            ),
        );
    }
}
