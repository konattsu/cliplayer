pub(super) fn encode_metadata(
    builder_version: &str,
) -> Result<Vec<u8>, crate::binary::Error> {
    let len = u32::try_from(builder_version.len())
        .map_err(|_| crate::binary::Error::TooLarge("builder_version length"))?;
    let mut out = Vec::with_capacity(4 + builder_version.len());
    write_u32(&mut out, len);
    out.extend_from_slice(builder_version.as_bytes());
    Ok(out)
}

pub(super) fn encode_dictionary<Id>(
    dictionary: &crate::util::BiMap<Id>,
) -> Result<Vec<u8>, crate::binary::Error>
where
    Id: From<u32> + Copy + std::cmp::Eq + std::hash::Hash,
{
    use crate::binary::Error;

    let strings = dictionary.ordered_strings();
    let count = u32::try_from(strings.len())
        .map_err(|_| Error::TooLarge("dictionary count"))?;
    let mut offsets = Vec::with_capacity(strings.len() + 1);
    let mut string_bytes = Vec::new();
    offsets.push(0u32);

    for value in strings {
        if value.is_empty() {
            return Err(Error::InvalidFormat("dictionary strings must be non-empty"));
        }
        string_bytes.extend_from_slice(value.as_bytes());
        offsets.push(
            u32::try_from(string_bytes.len())
                .map_err(|_| Error::TooLarge("dictionary string pool"))?,
        );
    }

    let mut out = Vec::with_capacity(4 + offsets.len() * 4 + string_bytes.len());
    write_u32(&mut out, count);
    for offset in offsets {
        write_u32(&mut out, offset);
    }
    out.extend_from_slice(&string_bytes);
    Ok(out)
}

pub(super) fn encode_u32_slice(values: &[u32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(values.len() * 4);
    for &value in values {
        write_u32(&mut out, value);
    }
    out
}

pub(super) fn encode_i64_slice<T>(values: &[T]) -> Vec<u8>
where
    T: Copy + Into<i64>,
{
    let mut out = Vec::with_capacity(values.len() * 8);
    for &value in values {
        let value: i64 = value.into();
        out.extend_from_slice(&value.to_le_bytes());
    }
    out
}

pub(super) fn encode_bool_slice(values: &[bool]) -> Vec<u8> {
    values.iter().map(|value| u8::from(*value)).collect()
}

pub(super) fn encode_u32_list_column(column: &crate::util::U32ListColumn) -> Vec<u8> {
    let mut out =
        Vec::with_capacity((column.offsets().len() + column.values().len()) * 4);
    for &offset in column.offsets() {
        write_u32(&mut out, offset);
    }
    for &value in column.values() {
        write_u32(&mut out, value);
    }
    out
}

pub(super) fn encode_dense_postings(
    term_count: usize,
    postings: &std::collections::HashMap<u32, crate::schema::PostingList>,
) -> Result<Vec<u8>, crate::binary::Error> {
    let mut offsets = Vec::with_capacity(term_count + 1);
    let mut values = Vec::new();
    offsets.push(0u32);

    for term_id in 0..term_count {
        if let Some(posting_list) = postings.get(&(term_id as u32)) {
            values.extend_from_slice(posting_list);
        }
        offsets.push(
            u32::try_from(values.len()).map_err(|_| {
                crate::binary::Error::TooLarge("posting list value count")
            })?,
        );
    }

    let mut out = Vec::with_capacity((offsets.len() + values.len()) * 4);
    for offset in offsets {
        write_u32(&mut out, offset);
    }
    for value in values {
        write_u32(&mut out, value);
    }
    Ok(out)
}

pub(super) fn encode_bool_postings(
    postings: &[crate::schema::PostingList; 2],
) -> Result<Vec<u8>, crate::binary::Error> {
    use crate::binary::Error;

    let false_len = u32::try_from(postings[0].len())
        .map_err(|_| Error::TooLarge("false bool postings length"))?;
    let true_end = postings[0]
        .len()
        .checked_add(postings[1].len())
        .ok_or(Error::TooLarge("bool postings total length"))?;
    let true_end = u32::try_from(true_end)
        .map_err(|_| Error::TooLarge("bool postings total length"))?;

    let mut out = Vec::with_capacity((3 + postings[0].len() + postings[1].len()) * 4);
    write_u32(&mut out, 0);
    write_u32(&mut out, false_len);
    write_u32(&mut out, true_end);
    for &doc_id in &postings[0] {
        write_u32(&mut out, doc_id);
    }
    for &doc_id in &postings[1] {
        write_u32(&mut out, doc_id);
    }
    Ok(out)
}

pub(super) fn read_u32(bytes: &[u8]) -> Result<u32, crate::binary::Error> {
    use crate::binary::Error;
    let bytes: [u8; 4] = bytes
        .get(..4)
        .ok_or(Error::InvalidFormat("u32 read out of bounds"))?
        .try_into()
        .map_err(|_| Error::InvalidFormat("invalid u32 bytes"))?;
    Ok(u32::from_le_bytes(bytes))
}

pub(super) fn read_u32_at(
    bytes: &[u8],
    offset: usize,
) -> Result<u32, crate::binary::Error> {
    use crate::binary::Error;

    read_u32(
        bytes
            .get(offset..offset + 4)
            .ok_or(Error::InvalidFormat("u32 read out of bounds"))?,
    )
}

pub(super) fn read_u64_at(
    bytes: &[u8],
    offset: usize,
) -> Result<u64, crate::binary::Error> {
    use crate::binary::Error;

    let bytes: [u8; 8] = bytes
        .get(offset..offset + 8)
        .ok_or(Error::InvalidFormat("u64 read out of bounds"))?
        .try_into()
        .map_err(|_| Error::InvalidFormat("invalid u64 bytes"))?;
    Ok(u64::from_le_bytes(bytes))
}

pub(super) fn write_u32(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_le_bytes());
}

pub(super) fn write_u64(out: &mut Vec<u8>, value: u64) {
    out.extend_from_slice(&value.to_le_bytes());
}

pub(super) fn section_table_len(
    section_count: u32,
) -> Result<usize, crate::binary::Error> {
    use crate::binary::Error;
    use crate::binary::format::SECTION_TABLE_ENTRY_SIZE;

    usize::try_from(section_count)
        .map_err(|_| Error::TooLarge("section count"))?
        .checked_mul(SECTION_TABLE_ENTRY_SIZE)
        .ok_or(Error::TooLarge("section table byte length"))
}

pub(super) fn align_up(value: u64) -> u64 {
    (value + 7) & !7
}

pub(super) fn pad_to_offset(
    out: &mut Vec<u8>,
    offset: u64,
) -> Result<(), crate::binary::Error> {
    use crate::binary::Error;

    let offset =
        usize::try_from(offset).map_err(|_| Error::TooLarge("section offset"))?;
    if out.len() > offset {
        return Err(Error::Io("attempted to pad backwards"));
    }
    out.resize(offset, 0);
    Ok(())
}
