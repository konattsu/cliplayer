/// Borrowed metadata view over the metadata section payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetadataView<'a> {
    builder_version: &'a str,
}

impl<'a> MetadataView<'a> {
    pub(crate) fn new(
        payload: &'a [u8],
        item_count: u32,
    ) -> Result<Self, crate::binary::Error> {
        use crate::binary::Error;

        if item_count != 1 {
            return Err(Error::InvalidFormat("metadata item_count must be 1"));
        }
        if payload.len() < 4 {
            return Err(Error::InvalidFormat("metadata payload too small"));
        }

        let builder_version_len = crate::binary::codec::read_u32(payload)? as usize;
        if payload.len() != 4 + builder_version_len {
            return Err(Error::InvalidFormat("metadata payload length mismatch"));
        }

        let builder_version =
            std::str::from_utf8(&payload[4..]).map_err(|_| Error::Utf8)?;

        Ok(Self { builder_version })
    }

    pub fn builder_version(&self) -> &'a str {
        self.builder_version
    }
}

/// Borrowed view over a little-endian `u32` slice payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U32SliceView<'a> {
    bytes: &'a [u8],
}

impl<'a> U32SliceView<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Result<Self, crate::binary::Error> {
        use crate::binary::Error;

        if !bytes.len().is_multiple_of(4) {
            return Err(Error::InvalidFormat(
                "u32 slice length must be multiple of 4",
            ));
        }

        Ok(Self { bytes })
    }

    pub fn len(&self) -> usize {
        self.bytes.len() / 4
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<u32> {
        if index >= self.len() {
            None
        } else {
            crate::binary::codec::read_u32_at(self.bytes, index * 4).ok()
        }
    }

    pub fn iter(&self) -> U32SliceIter<'a> {
        U32SliceIter {
            chunks: self.bytes.chunks_exact(4),
        }
    }

    pub fn to_vec(&self) -> Vec<u32> {
        self.iter().collect()
    }

    pub(crate) fn slice(
        &self,
        start: usize,
        end: usize,
    ) -> Result<Self, crate::binary::Error> {
        use crate::binary::Error;

        if start > end || end > self.len() {
            return Err(Error::InvalidFormat("u32 slice range out of bounds"));
        }

        Self::new(&self.bytes[start * 4..end * 4])
    }
}

#[derive(Debug, Clone)]
pub struct U32SliceIter<'a> {
    chunks: std::slice::ChunksExact<'a, u8>,
}

impl Iterator for U32SliceIter<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.chunks
            .next()
            .map(|chunk| u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
    }
}

/// Borrowed view over a little-endian `i64` slice payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I64SliceView<'a> {
    bytes: &'a [u8],
}

impl<'a> I64SliceView<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Result<Self, crate::binary::Error> {
        use crate::binary::Error;

        if !bytes.len().is_multiple_of(8) {
            return Err(Error::InvalidFormat(
                "i64 slice length must be multiple of 8",
            ));
        }

        Ok(Self { bytes })
    }

    pub fn len(&self) -> usize {
        self.bytes.len() / 8
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<i64> {
        if index >= self.len() {
            return None;
        }

        let start = index * 8;
        let chunk = &self.bytes[start..start + 8];
        Some(i64::from_le_bytes([
            chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6],
            chunk[7],
        ]))
    }

    pub fn iter(&self) -> I64SliceIter<'a> {
        I64SliceIter {
            chunks: self.bytes.chunks_exact(8),
        }
    }

    pub fn to_vec(&self) -> Vec<i64> {
        self.iter().collect()
    }
}

#[derive(Debug, Clone)]
pub struct I64SliceIter<'a> {
    chunks: std::slice::ChunksExact<'a, u8>,
}

impl Iterator for I64SliceIter<'_> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        self.chunks.next().map(|chunk| {
            i64::from_le_bytes([
                chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6],
                chunk[7],
            ])
        })
    }
}

/// Borrowed view over a `bool` payload encoded as `0` / `1` bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoolSliceView<'a> {
    bytes: &'a [u8],
}

impl<'a> BoolSliceView<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Result<Self, crate::binary::Error> {
        use crate::binary::Error;

        if bytes.iter().any(|value| !matches!(*value, 0 | 1)) {
            return Err(Error::InvalidFormat("bool value must be 0 or 1"));
        }

        Ok(Self { bytes })
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<bool> {
        self.bytes.get(index).map(|value| *value == 1)
    }

    pub fn iter(&self) -> BoolSliceIter<'a> {
        BoolSliceIter {
            values: self.bytes.iter(),
        }
    }

    pub fn to_vec(&self) -> Vec<bool> {
        self.iter().collect()
    }
}

#[derive(Debug, Clone)]
pub struct BoolSliceIter<'a> {
    values: std::slice::Iter<'a, u8>,
}

impl Iterator for BoolSliceIter<'_> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.values.next().map(|value| *value == 1)
    }
}

/// Borrowed view over a dictionary section.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringDictionaryView<'a> {
    offsets: U32SliceView<'a>,
    string_bytes: &'a [u8],
}

impl<'a> StringDictionaryView<'a> {
    pub(crate) fn new(
        payload: &'a [u8],
        item_count: u32,
    ) -> Result<Self, crate::binary::Error> {
        use crate::binary::Error;

        if payload.len() < 4 {
            return Err(Error::InvalidFormat("dictionary payload too small"));
        }

        let count = crate::binary::codec::read_u32(payload)? as usize;
        if count != item_count as usize {
            return Err(Error::InvalidFormat("dictionary count mismatch"));
        }

        let offsets_len = 4usize
            .checked_mul(count + 1)
            .ok_or(Error::InvalidFormat("dictionary offsets length overflow"))?;
        let header_len = 4usize
            .checked_add(offsets_len)
            .ok_or(Error::InvalidFormat("dictionary header overflow"))?;
        if payload.len() < header_len {
            return Err(Error::InvalidFormat("dictionary payload truncated"));
        }

        let offsets = U32SliceView::new(&payload[4..header_len])?;
        validate_offsets_start_at_zero(
            &offsets,
            "dictionary offsets must start at zero",
        )?;
        validate_monotonic_offsets(
            &offsets,
            "dictionary offsets must be monotonically increasing",
        )?;

        let string_bytes = &payload[header_len..];
        if offsets.get(count) != Some(string_bytes.len() as u32) {
            return Err(Error::InvalidFormat("dictionary offsets length mismatch"));
        }

        let view = Self {
            offsets,
            string_bytes,
        };
        view.validate_strings()?;
        Ok(view)
    }

    pub fn len(&self) -> usize {
        self.offsets.len().saturating_sub(1)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: u32) -> Result<Option<&'a str>, crate::binary::Error> {
        let index = index as usize;
        if index >= self.len() {
            Ok(None)
        } else {
            Ok(Some(self.string_at(index)?))
        }
    }

    pub fn iter(&self) -> DictionaryIter<'a> {
        DictionaryIter {
            view: *self,
            index: 0,
        }
    }

    fn validate_strings(&self) -> Result<(), crate::binary::Error> {
        use crate::binary::Error;

        let mut seen = std::collections::HashSet::with_capacity(self.len());
        for index in 0..self.len() {
            let value = self.string_at(index)?;
            if value.is_empty() {
                return Err(Error::InvalidFormat(
                    "dictionary strings must be non-empty",
                ));
            }
            if !seen.insert(value) {
                return Err(Error::InvalidFormat("duplicate dictionary string"));
            }
        }

        Ok(())
    }

    fn string_at(&self, index: usize) -> Result<&'a str, crate::binary::Error> {
        use crate::binary::Error;

        let start = self
            .offsets
            .get(index)
            .ok_or(Error::InvalidFormat("dictionary offset out of bounds"))?
            as usize;
        let end = self
            .offsets
            .get(index + 1)
            .ok_or(Error::InvalidFormat("dictionary offset out of bounds"))?
            as usize;
        if start == end {
            return Err(Error::InvalidFormat("dictionary strings must be non-empty"));
        }

        std::str::from_utf8(&self.string_bytes[start..end]).map_err(|_| Error::Utf8)
    }
}

#[derive(Debug, Clone)]
pub struct DictionaryIter<'a> {
    view: StringDictionaryView<'a>,
    index: usize,
}

impl<'a> Iterator for DictionaryIter<'a> {
    type Item = Result<&'a str, crate::binary::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.view.len() {
            None
        } else {
            let result = self.view.string_at(self.index);
            self.index += 1;
            Some(result)
        }
    }
}

/// Borrowed view over a `offsets + values` list column.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U32ListColumnView<'a> {
    offsets: U32SliceView<'a>,
    values: U32SliceView<'a>,
}

impl<'a> U32ListColumnView<'a> {
    pub(crate) fn new(
        payload: &'a [u8],
        record_count: usize,
        max_id: usize,
        name: &'static str,
    ) -> Result<Self, crate::binary::Error> {
        use crate::binary::Error;

        let offsets_len = 4usize
            .checked_mul(record_count + 1)
            .ok_or(Error::InvalidFormat("list column offsets overflow"))?;
        if payload.len() < offsets_len {
            return Err(Error::InvalidFormat("list column payload truncated"));
        }

        let offsets = U32SliceView::new(&payload[..offsets_len])?;
        validate_offsets_start_at_zero(
            &offsets,
            "list column offsets must start at zero",
        )?;
        validate_monotonic_offsets(
            &offsets,
            "list column offsets must be monotonically increasing",
        )?;

        let values = U32SliceView::new(&payload[offsets_len..])?;
        if offsets.get(record_count) != Some(values.len() as u32) {
            return Err(Error::InvalidFormat("list column offsets length mismatch"));
        }

        let view = Self { offsets, values };
        view.validate(max_id, name)?;
        Ok(view)
    }

    pub fn len(&self) -> usize {
        self.offsets.len().saturating_sub(1)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(
        &self,
        index: usize,
    ) -> Result<Option<U32SliceView<'a>>, crate::binary::Error> {
        use crate::binary::Error;

        if index >= self.len() {
            return Ok(None);
        }

        let start = self
            .offsets
            .get(index)
            .ok_or(Error::InvalidFormat("list column offset out of bounds"))?
            as usize;
        let end = self
            .offsets
            .get(index + 1)
            .ok_or(Error::InvalidFormat("list column offset out of bounds"))?
            as usize;
        Ok(Some(self.values.slice(start, end)?))
    }

    pub fn offsets(&self) -> U32SliceView<'a> {
        self.offsets
    }

    pub fn values(&self) -> U32SliceView<'a> {
        self.values
    }

    fn validate(
        &self,
        max_id: usize,
        name: &'static str,
    ) -> Result<(), crate::binary::Error> {
        use crate::binary::Error;

        for index in 0..self.len() {
            let list = self
                .get(index)?
                .ok_or(Error::InvalidFormat("list column offset out of bounds"))?;
            validate_sorted_unique_u32_values(&list, max_id, name)?;
        }

        Ok(())
    }
}

/// Borrowed view over a dense postings section.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DensePostingsView<'a> {
    offsets: U32SliceView<'a>,
    doc_ids: U32SliceView<'a>,
}

impl<'a> DensePostingsView<'a> {
    pub(crate) fn new(
        payload: &'a [u8],
        term_count: usize,
        record_count: u32,
    ) -> Result<Self, crate::binary::Error> {
        use crate::binary::Error;

        let offsets_len = 4usize
            .checked_mul(term_count + 1)
            .ok_or(Error::InvalidFormat("postings offsets overflow"))?;
        if payload.len() < offsets_len {
            return Err(Error::InvalidFormat("postings payload truncated"));
        }

        let offsets = U32SliceView::new(&payload[..offsets_len])?;
        validate_offsets_start_at_zero(
            &offsets,
            "postings offsets must start at zero",
        )?;
        validate_monotonic_offsets(
            &offsets,
            "postings offsets must be monotonically increasing",
        )?;

        let doc_ids = U32SliceView::new(&payload[offsets_len..])?;
        if offsets.get(term_count) != Some(doc_ids.len() as u32) {
            return Err(Error::InvalidFormat("postings offsets length mismatch"));
        }

        let view = Self { offsets, doc_ids };
        view.validate(record_count)?;
        Ok(view)
    }

    pub fn len(&self) -> usize {
        self.offsets.len().saturating_sub(1)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(
        &self,
        term_id: usize,
    ) -> Result<Option<U32SliceView<'a>>, crate::binary::Error> {
        use crate::binary::Error;

        if term_id >= self.len() {
            return Ok(None);
        }

        let start = self
            .offsets
            .get(term_id)
            .ok_or(Error::InvalidFormat("postings offset out of bounds"))?
            as usize;
        let end = self
            .offsets
            .get(term_id + 1)
            .ok_or(Error::InvalidFormat("postings offset out of bounds"))?
            as usize;
        Ok(Some(self.doc_ids.slice(start, end)?))
    }

    pub fn offsets(&self) -> U32SliceView<'a> {
        self.offsets
    }

    pub fn doc_ids(&self) -> U32SliceView<'a> {
        self.doc_ids
    }

    fn validate(&self, record_count: u32) -> Result<(), crate::binary::Error> {
        use crate::binary::Error;

        for term_id in 0..self.len() {
            let posting_list = self
                .get(term_id)?
                .ok_or(Error::InvalidFormat("postings offset out of bounds"))?;
            validate_posting_list(&posting_list, record_count)?;
        }

        Ok(())
    }
}

/// Borrowed view over the two bool postings lists.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoolPostingsView<'a> {
    false_docs: U32SliceView<'a>,
    true_docs: U32SliceView<'a>,
}

impl<'a> BoolPostingsView<'a> {
    pub(crate) fn new(
        payload: &'a [u8],
        item_count: u32,
        record_count: u32,
    ) -> Result<Self, crate::binary::Error> {
        use crate::binary::Error;

        if item_count != 2 {
            return Err(Error::InvalidFormat("bool postings item_count must be 2"));
        }
        if payload.len() < 12 {
            return Err(Error::InvalidFormat("bool postings payload truncated"));
        }

        let offsets = U32SliceView::new(&payload[..12])?;
        validate_offsets_start_at_zero(
            &offsets,
            "bool postings offsets must start at zero",
        )?;
        validate_monotonic_offsets(
            &offsets,
            "bool postings offsets must be monotonically increasing",
        )?;

        let values = U32SliceView::new(&payload[12..])?;
        if offsets.len() != 3 || offsets.get(2) != Some(values.len() as u32) {
            return Err(Error::InvalidFormat(
                "bool postings offsets length mismatch",
            ));
        }

        let false_docs = values.slice(0, offsets.get(1).unwrap_or(0) as usize)?;
        let true_docs = values.slice(
            offsets.get(1).unwrap_or(0) as usize,
            offsets.get(2).unwrap_or(0) as usize,
        )?;

        validate_posting_list(&false_docs, record_count)?;
        validate_posting_list(&true_docs, record_count)?;

        Ok(Self {
            false_docs,
            true_docs,
        })
    }

    pub fn false_docs(&self) -> U32SliceView<'a> {
        self.false_docs
    }

    pub fn true_docs(&self) -> U32SliceView<'a> {
        self.true_docs
    }

    pub fn validate_matches_column(
        &self,
        column: BoolSliceView<'_>,
    ) -> Result<(), crate::binary::Error> {
        use crate::binary::Error;

        let expected_false = column
            .iter()
            .enumerate()
            .filter_map(|(doc_id, value)| (!value).then_some(doc_id as u32))
            .collect::<Vec<_>>();
        let expected_true = column
            .iter()
            .enumerate()
            .filter_map(|(doc_id, value)| value.then_some(doc_id as u32))
            .collect::<Vec<_>>();

        if self.false_docs.to_vec() != expected_false
            || self.true_docs.to_vec() != expected_true
        {
            return Err(Error::InvalidFormat(
                "bool postings must partition all docs and match column values",
            ));
        }

        Ok(())
    }
}

/// Borrowed view over a sort index section.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SortIndexView<'a> {
    doc_ids_asc: U32SliceView<'a>,
}

impl<'a> SortIndexView<'a> {
    pub(crate) fn new(doc_ids_asc: U32SliceView<'a>) -> Self {
        Self { doc_ids_asc }
    }

    pub fn doc_ids_asc(&self) -> U32SliceView<'a> {
        self.doc_ids_asc
    }

    pub fn validate_against_timestamps(
        &self,
        published_ats: &I64SliceView<'_>,
    ) -> Result<(), crate::binary::Error> {
        use crate::binary::Error;

        if self.doc_ids_asc.len() != published_ats.len() {
            return Err(Error::InvalidFormat(
                "sort index length does not match record_count",
            ));
        }

        let mut seen = vec![false; self.doc_ids_asc.len()];
        for doc_id in self.doc_ids_asc.iter() {
            let doc_id = usize::try_from(doc_id)
                .map_err(|_| Error::InvalidFormat("doc_id does not fit usize"))?;
            if doc_id >= published_ats.len() {
                return Err(Error::InvalidFormat("sort index doc_id out of range"));
            }
            if seen[doc_id] {
                return Err(Error::InvalidFormat("sort index doc_ids must be unique"));
            }
            seen[doc_id] = true;
        }

        let doc_ids = self.doc_ids_asc.to_vec();
        for window in doc_ids.windows(2) {
            let left_doc = window[0] as usize;
            let right_doc = window[1] as usize;
            let left = (
                published_ats
                    .get(left_doc)
                    .ok_or(Error::InvalidFormat("published_at out of bounds"))?,
                window[0],
            );
            let right = (
                published_ats
                    .get(right_doc)
                    .ok_or(Error::InvalidFormat("published_at out of bounds"))?,
                window[1],
            );
            if left > right {
                return Err(Error::InvalidFormat(
                    "sort index must be ordered by (published_at asc, doc_id asc)",
                ));
            }
        }

        Ok(())
    }
}

fn validate_offsets_start_at_zero(
    offsets: &U32SliceView<'_>,
    message: &'static str,
) -> Result<(), crate::binary::Error> {
    if offsets.get(0) == Some(0) {
        Ok(())
    } else {
        Err(crate::binary::Error::InvalidFormat(message))
    }
}

fn validate_monotonic_offsets(
    offsets: &U32SliceView<'_>,
    message: &'static str,
) -> Result<(), crate::binary::Error> {
    if offsets.len() < 2 {
        return Ok(());
    }

    let mut previous = offsets.get(0).unwrap_or(0);
    for current in offsets.iter().skip(1) {
        if previous > current {
            return Err(crate::binary::Error::InvalidFormat(message));
        }
        previous = current;
    }

    Ok(())
}

fn validate_sorted_unique_u32_values(
    values: &U32SliceView<'_>,
    max_id: usize,
    name: &'static str,
) -> Result<(), crate::binary::Error> {
    let mut previous = None;
    for value in values.iter() {
        if (value as usize) >= max_id {
            return Err(crate::binary::Error::InvalidFormat(name));
        }
        if previous.is_some_and(|prev| prev >= value) {
            return Err(crate::binary::Error::InvalidFormat(name));
        }
        previous = Some(value);
    }

    Ok(())
}

fn validate_posting_list(
    posting_list: &U32SliceView<'_>,
    record_count: u32,
) -> Result<(), crate::binary::Error> {
    let mut previous = None;
    for doc_id in posting_list.iter() {
        if doc_id >= record_count {
            return Err(crate::binary::Error::InvalidFormat(
                "posting list doc_id out of range",
            ));
        }
        if previous.is_some_and(|prev| prev >= doc_id) {
            return Err(crate::binary::Error::InvalidFormat(
                "posting lists must be sorted and unique",
            ));
        }
        previous = Some(doc_id);
    }

    Ok(())
}
