use crate::structure::dict_ht::DictHt;
use std::hash::Hash;

pub struct Dict<K, V> 
where K : Hash + PartialEq{
    capacity: usize,
    dict_hts: [Option<DictHt<K, V>>; 2],
    rehash_idx: i64,
}


impl<K, V> Dict<K, V> 
where K : Hash + PartialEq {

    fn insert(&mut self,key: K, value: V) {
        self.do_rehash();

    }


    fn new(size: usize) -> Self {
        let dict_hts = [Some(DictHt::new(size)), None];
        return Dict {
            capacity: 2,
            rehash_idx: -1,
            dict_hts,
        };
    }
    fn do_rehash(&mut self) {
        if self.dict_is_need_rehash() {
            return;
        }
        if self.rehash_idx == -1 {
            self.dict_hts[1] = Some(DictHt::new(self.capacity * 2));
        }
        if let Some(dict_ht) = &self.dict_hts[0] {
            let dict_entry_array = &dict_ht.dict_entry_array;
            if self.rehash_idx == dict_entry_array.len() as i64 {
                self.dict_hts[0] = self.dict_hts[1].take();
                self.rehash_idx = -1;
                return;
            }
            self.rehash_idx += 1;
            let dict_entry = &dict_entry_array[self.rehash_idx as usize];
        }
    }

    fn dict_is_need_rehash(&mut self) -> bool {
        if self.dict_is_rehashing() {
            return true;
        }
        if let Some(dict_ht) = &self.dict_hts[0] {
            return dict_ht.used > self.capacity;
        } else {
            return false;
        }
    }

    fn dict_is_rehashing(&mut self) -> bool {
        return self.rehash_idx != -1;
    }
}

