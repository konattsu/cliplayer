/// 双方向マップ
///
/// str -> id: HashMap
/// id -> str: Vec (indexがidに対応する)
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BiMap<Id> {
    str_to_id: std::collections::HashMap<String, Id>,
    id_to_str: Vec<String>,
}

impl<Id> BiMap<Id>
where
    Id: From<u32> + Copy + std::cmp::Eq + std::hash::Hash,
{
    /// 双方向マップを作成
    pub fn build(keys: std::collections::BTreeSet<String>) -> Self {
        let mut id_to_str = Vec::<String>::with_capacity(keys.len());
        let mut str_to_id =
            std::collections::HashMap::<String, Id>::with_capacity(keys.len());

        if keys.len() > (u32::MAX as usize) {
            panic!("too many keys for BiMap: {}", keys.len());
        }

        for (idx, key) in keys.into_iter().enumerate() {
            let id = Id::from(idx as u32);
            str_to_id.insert(key.clone(), id);
            id_to_str.push(key);
        }

        Self {
            str_to_id,
            id_to_str,
        }
    }

    pub fn get_by_str(&self, value: &str) -> Option<Id> {
        self.str_to_id.get(value).copied()
    }

    pub fn ordered_strings(&self) -> &[String] {
        &self.id_to_str
    }

    pub fn len(&self) -> usize {
        self.id_to_str.len()
    }

    pub fn is_empty(&self) -> bool {
        self.id_to_str.is_empty()
    }

    pub fn from_ordered_strings(strings: Vec<String>) -> Result<Self, &'static str> {
        let mut str_to_id =
            std::collections::HashMap::<String, Id>::with_capacity(strings.len());

        if strings.len() > (u32::MAX as usize) {
            return Err("too many keys for BiMap");
        }

        for (idx, key) in strings.iter().enumerate() {
            if str_to_id
                .insert(key.clone(), Id::from(idx as u32))
                .is_some()
            {
                return Err("duplicate key in BiMap");
            }
        }

        Ok(Self {
            str_to_id,
            id_to_str: strings,
        })
    }
}

/// [`ColumnsStore`]の中で、可変長のフィールドを詰めて持つための構造体
///
/// Vec<Vec<T>>のがたがたな二次元配列を避けられ、一次配列に落とし込めるので、キャッシュ効率が良くなる。
///
/// e.g.
///
/// ```rs
/// doc0 = [10, 20]
/// doc1 = []
/// doc2 = [30]
///
/// values  = [10, 20, 30]
/// offsets = [0, 2, 2, 3]
/// ```
///
/// このとき
///
/// - doc0 の範囲は values[0..2]
/// - doc1 の範囲は values[2..2]
/// - doc2 の範囲は values[2..3]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct U32ListColumn {
    offsets: Vec<u32>,
    values: Vec<u32>,
}

impl U32ListColumn {
    /// `Self`を生成する
    pub fn build(lists: &[Vec<u32>]) -> Self {
        let mut offsets = Vec::with_capacity(lists.len() + 1);
        let mut values = Vec::new();

        offsets.push(0);

        for list in lists {
            values.extend(list.iter().copied());
            offsets.push(values.len() as u32);
        }

        Self { offsets, values }
    }

    /// `index`番目のリストを返す
    pub fn get(&self, index: usize) -> &[u32] {
        let start = self.offsets[index] as usize;
        let end = self.offsets[index + 1] as usize;
        &self.values[start..end]
    }

    pub fn offsets(&self) -> &[u32] {
        &self.offsets
    }

    pub fn values(&self) -> &[u32] {
        &self.values
    }

    pub fn from_parts(offsets: Vec<u32>, values: Vec<u32>) -> Self {
        Self { offsets, values }
    }
}
