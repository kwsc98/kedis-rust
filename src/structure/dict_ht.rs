use std::hash::{BuildHasher, Hash, Hasher};

#[derive(Debug)]
pub struct DictHt<K, V, H>
{
    hash_builder: H,
    pub dict_entry_array: Vec<Option<Box<DictEntry<K, V>>>>,
    _size: usize,
    pub used: usize,
}

#[derive(Debug)]
pub struct DictEntry<K, V> {
    pub key: K,
    pub value: Option<V>,
    pub next: Option<Box<DictEntry<K, V>>>,
}
impl<K, V> DictEntry<K, V> {
    fn new(key: K, value: V) -> Self {
        return DictEntry {
            key,
            value: Some(value),
            next: None,
        };
    }
}

impl<K, V, H: BuildHasher> DictHt<K, V, H>
where
    K: Hash + PartialEq + ToString
{
    pub fn new(size: usize, hash_builder: H) -> Self {
        let mut dict_entry_array = vec![];
        for _idx in 0..size {
            dict_entry_array.push(None);
        }
        return DictHt {
            hash_builder,
            used: 0,
            _size: size,
            dict_entry_array,
        };
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let option_entry = self.get_mut_ref_entry(&key);
        if let Some(entry) = option_entry {
            let res = entry.value.take();
            let _ = entry.value.insert(value);
            return res;
        }
        self.used += 1;
        let idx = self.hash(&key) as usize;
        let option_entry = self.get_mut_ref_last_entry(idx);
        let _ = option_entry.insert(Box::new(DictEntry::new(key, value)));
        return None;
    }

    pub fn get_ref_entry(&self, key: &K) -> Option<&DictEntry<K, V>> {
        let idx = self.hash(&key) as usize;
        let mut bucket = self.dict_entry_array.get(idx)?;
        while let Some(entry) = bucket {
            if &entry.key == key {
                return Some(entry);
            } else {
                bucket = &entry.next;
            }
        }
        return None;
    }

    pub fn get_mut_ref_entry(&mut self, key: &K) -> Option<&mut DictEntry<K, V>> {
        let idx = self.hash(&key) as usize;
        let mut bucket = self.dict_entry_array.get_mut(idx)?;
        while let Some(entry) = bucket {
            if &entry.key == key {
                return Some(entry);
            } else {
                bucket = &mut entry.next;
            }
        }
        return None;
    }

    pub fn get_mut_ref_last_entry(&mut self, idx: usize) -> &mut Option<Box<DictEntry<K, V>>> {
        let mut bucket = self.dict_entry_array.get_mut(idx).unwrap();
        while let Some(entry) = bucket {
            bucket = &mut entry.next;
        }
        return bucket;
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        return match self.remove_by_entry(key) {
            Some(entry) => entry.value,
            _ => None,
        };
    }

    fn remove_by_entry(&mut self, key: &K) -> Option<Box<DictEntry<K, V>>> {
        let idx: usize = self.hash(&key) as usize;
        let bucket = self.dict_entry_array.get_mut(idx)?;

        match bucket {
            Some(entry) => {
                if &entry.key == key {
                    self.used -= 1;
                    let mut entry = self.dict_entry_array[idx].take()?;
                    let next = entry.next.take();
                    self.dict_entry_array[idx] = next;
                    return Some(entry);
                }
            }
            None => return None,
        };
        let mut bucket = self.dict_entry_array.get_mut(idx)?;
        let mut res = None;
        while let Some(entry) = bucket {
            if let Some(next) = &mut entry.next {
                if &next.key == key {
                    let mut next = entry.next.take()?;
                    let next_next = next.next.take();    
                    let _ = entry.next = next_next;
                    self.used -= 1;
                    res = Some(next);
                }
            }
            bucket = &mut entry.next;
        }

        return res;
    }

    pub(crate) fn hash(&self, val: &K) -> u64 {
        let mut hasher = self.hash_builder.build_hasher();
        val.hash(&mut hasher);
        return hasher.finish() & self.dict_entry_array.len() as u64 - 1;
    }

}
