use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash, Hasher};
pub struct DictHt<K, V, H = RandomState> 
where K : Hash + PartialEq
{
    hash_builder: H,
    pub(crate) dict_entry_array: Vec<Option<Box<DictEntry<K, V>>>>,
    size: usize,
    pub(crate) used: usize,
}

struct DictEntry<K, V> {
    key: K,
    value: Option<V>,
    next: Option<Box<DictEntry<K, V>>>,
}

impl<K, V> DictHt<K, V> 
where K : Hash + PartialEq{
    pub fn new(size: usize) -> Self {
        let mut dict_entry_array = vec![];
        for _idx in 0..size {
            dict_entry_array.push(None);
        }
        return DictHt {
            hash_builder: Default::default(),
            used: 0,
            size,
            dict_entry_array,
        };
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let option_entry = self.get_mut_ref_entry(&key);
        if let Some(entry) = option_entry {
            let res = entry.value.take();
            entry.value = Some(value);
            return res;
        }
        let idx = self.hash(&key) as usize;

        return None;
    }

    fn get_ref_entry(&mut self, key: &K) -> Option<&DictEntry<K, V>> {
        return match self.get_mut_ref_entry(key) {
            Some(entry) => { Some(entry) }
            None => { None }
        };
    }

    fn get_mut_ref_entry(&mut self, key: &K) -> Option<&mut DictEntry<K, V>> {
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

    fn remove(&mut self, key: &K) -> Option<V> {
        return match self.remove_by_entry(key) {
            Some(entry) => { entry.value }
            _ => { None }
        };
    }

    fn remove_by_entry(&mut self, key: &K) -> Option<DictEntry<K, V>> {
        let idx = self.hash(&key) as usize;
        let mut bucket = self.dict_entry_array.get_mut(idx)?;
        while let Some(entry) = bucket {
            if &entry.key == key {
                self.used -= 1;
                let next = entry.next.take();
                let res = bucket.take()?;
                match next {
                    Some(entry) => {
                        let _ = bucket.insert(entry);
                    }
                    _ => {}
                }
                return Some(*res);
            } else {
                bucket = &mut entry.next;
            }
        }
        return None;
    }

    pub(crate) fn hash(&mut self, val: &K) -> u64
    {
        let mut hasher = self.hash_builder.build_hasher();
        val.hash(&mut hasher);
        return hasher.finish() & self.dict_entry_array.len() as u64 - 1;
    }
}