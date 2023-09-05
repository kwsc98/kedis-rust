use crate::structure::dict_ht::DictHt;
use std::{collections::hash_map::RandomState, hash::Hash, usize};


#[warn(dead_code)]
#[derive(Debug)]
pub struct Dict<K, V, H = RandomState> 
{
    capacity: usize,
    dict_hts: [Option<DictHt<K, V, H>>; 2],
    rehash_idx: i64,
    hash_builder: H,
}

impl<K, V> Dict<K, V>
where
    K: Hash + PartialEq,
{
    pub fn new(size: usize) -> Self {
        let mut dict: Dict<K, V, RandomState> = Dict {
            hash_builder: Default::default(),
            capacity: size,
            rehash_idx: -1,
            dict_hts: [None, None],
        };
        let dict_ht: DictHt<K, V, RandomState> = DictHt::new(size,dict.hash_builder.clone());
        let _ = dict.dict_hts[0].insert(dict_ht);
        return dict;
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.do_rehash();
        let mut idx = 0;
        if self.is_rehashing() {
            idx = 1;
        }
        let dict_ht = self.dict_hts.get_mut(idx)?;
        if let Some(dict_ht) = dict_ht {
            return dict_ht.insert(key, value);
        }
        return None;
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let mut idx = 0;
        if let None = self.get_value(key, idx) {
            if self.is_rehashing() {
                idx = 1;
            } else {
                return None;
            }
        }
        return self.get_value(key, idx);
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.do_rehash();
        let mut idx = 0;
        if let None = self.get_value(key, idx) {
            if self.is_rehashing() {
                idx = 1;
            } else {
                return None;
            }
        }
        return self.get_mut_value(key, idx);
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.do_rehash();
        let mut idx = 0;
        if let None = self.get_value(key, 0) {
            if self.is_rehashing() {
                idx = 1;
            } else {
                println!("dsdsd");
                return None;
            }
        }
        let dict_ht = self.dict_hts[idx].as_mut()?;
        return dict_ht.remove(key);
    }

    fn get_mut_value(&mut self, key: &K, idx: usize) -> Option<&mut V> {
        let entry = self.dict_hts[idx].as_mut()?.get_mut_ref_entry(key);
        if let Some(entry) = entry {
            return entry.value.as_mut();
        }
        return None;
    }

    fn get_value(&self, key: &K, idx: usize) -> Option<&V> {
        let entry = self.dict_hts[idx].as_ref()?.get_ref_entry(key);
        if let Some(entry) = entry {
            return entry.value.as_ref()
        }
        return None;
    }

    fn do_rehash(&mut self) {
        if !self.is_need_rehash() {
            return;
        }
        if self.rehash_idx == -1 {
            self.dict_hts[1] = Some(DictHt::new(self.capacity * 2, self.hash_builder.clone()));
            self.capacity *= 2;
        }
        let dict_ht_0 = self.dict_hts[0].as_mut().unwrap();
        self.rehash_idx += 1;
        if self.rehash_idx == dict_ht_0.dict_entry_array.len() as i64 {
            self.dict_hts[0] = self.dict_hts[1].take();
            self.rehash_idx = -1;
            return;
        }
        let mut count = 0;
        {
            let first_entry = dict_ht_0
                .dict_entry_array
                .get_mut(self.rehash_idx as usize)
                .unwrap();
            let mut first_entry = first_entry.take();
            let dict_ht_1 = self.dict_hts[1].as_mut().unwrap();
            while let Some(entry) = first_entry {
                count += 1;
                let key = entry.key;
                let value = entry.value.unwrap();
                dict_ht_1.insert(key, value);
                first_entry = entry.next;
            }
        }
        self.dict_hts[0].as_mut().unwrap().used -= count;
    }

    fn is_need_rehash(&mut self) -> bool {
        if self.is_rehashing() {
            return true;
        }
        if let Some(dict_ht) = &self.dict_hts[0] {
            return dict_ht.used > self.capacity;
        } else {
            return false;
        }
    }

    fn is_rehashing(&self) -> bool {
        return self.rehash_idx != -1;
    }

    fn _get_next_idx(mut idx : u64,len : u64) -> u64 {
        idx |= !len + 1;
        idx = Self::_rev(idx);
        idx += 1 ;
        idx = Self::_rev(idx);
        return idx;
    }
    
    fn _rev(mut index : u64) -> u64 {
        let mut rev = 0;
        let mut i = 0;
        while i < 64 && index != 0 {
            rev |= (index & 1) << (63 - i);
            index >>= 1;
            i += 1;
        }
        return rev;
    }
}
