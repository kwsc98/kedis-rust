use common::date_util as DateUtil;
use kedis_rust::{common, structure::dict::Dict};
use std::{
    collections::hash_map::RandomState,
    hash::{BuildHasher, Hash, Hasher},
};

#[test]
fn dict_test_1() {
    let start_time = DateUtil::get_now_date_time_as_millis();
    let mut dict = Dict::new(2000);
    for idx in 0..1000000 {
        dict.insert(idx, idx + 1);
    }
    let mut res = 0;
    for idx in 0..2000000 {
        if let Some(value) = dict.get(&idx) {
            res += 1;
            assert_eq!(idx, value - 1);
        }
    }
    assert_eq!(1000000, res);
    println!(
        "dict_test_1 done run time (millis): {}",
        DateUtil::get_now_date_time_as_millis() - start_time
    );
}

#[test]
fn dict_test_2() {
    let start_time = DateUtil::get_now_date_time_as_millis();
    let mut dict = Dict::new(2000);
    for idx in 0..1000000 {
        dict.insert(idx, idx + 1);
    }
    let mut res = 0;
    for idx in 0..2000000 {
        if let Some(value) = dict.get_mut(&idx) {
            res += 1;
            assert_eq!(idx, *value - 1);
            *value += 1;
        }
    }
    assert_eq!(1000000, res);
    let mut res = 0;
    for idx in 0..2000000 {
        if let Some(value) = dict.get_mut(&idx) {
            res += 1;
            assert_eq!(idx, *value - 2);
            *value += 1;
        }
    }
    assert_eq!(1000000, res);
    println!(
        "dict_test_1 done run time (millis): {}",
        DateUtil::get_now_date_time_as_millis() - start_time
    );
}

#[test]
fn dict_test_3() {
    let start_time = DateUtil::get_now_date_time_as_millis();
    let mut dict = Dict::new(2000);
    for idx in 0..1000000 {
        dict.insert(idx, idx + 1);
    }
    let mut res = 0;
    for idx in 0..2000000 {
        if let Some(value) = dict.get_mut(&idx) {
            res += 1;
            assert_eq!(idx, *value - 1);
            *value += 1;
        }
    }
    assert_eq!(1000000, res);
    for idx in 0..1000000 {
        dict.insert(idx, idx + 2);
    }
    let mut res = 0;
    for idx in 0..2000000 {
        if let Some(value) = dict.get_mut(&idx) {
            res += 1;
            assert_eq!(idx, *value - 2);
        }
    }
    assert_eq!(1000000, res);
    println!(
        "dict_test_1 done run time (millis): {}",
        DateUtil::get_now_date_time_as_millis() - start_time
    );
}

#[test]
fn dict_test_4() {
    let start_time = DateUtil::get_now_date_time_as_millis();
    let mut dict = Dict::new(2000);
    for idx in 0..1000000 {
        dict.insert(idx, idx + 1);
    }
    let mut res = 0;
    for idx in 0..2000000 {
        if let Some(value) = dict.get(&idx) {
            res += 1;
            assert_eq!(idx, *value - 1);
        }
    }
    assert_eq!(1000000, res);
    for idx in 0..1000000 {
        let value = dict.remove(&idx);
        assert_eq!(value, Some(idx + 1));
    }
    let mut res = 0;
    for idx in 0..2000000 {
        if let Some(_value) = dict.get(&idx) {
            res += 1;
        }
    }
    assert_eq!(0, res);
    println!(
        "dict_test_1 done run time (millis): {}",
        DateUtil::get_now_date_time_as_millis() - start_time
    );
}

#[test]
fn dict_test_5() {
    let hash_builder = RandomState::new();
    let pre = hash(&hash_builder, &1, 8);
    println!("{}", pre);
    println!("{}", get_next_idx(pre, 16));
    println!("-------");
    println!("{}", hash(&hash_builder, &1, 16));
    println!("{}", hash(&hash_builder, &1, 16));
    println!("{}", hash(&hash_builder, &1, 16));
    println!("{}", hash(&hash_builder, &1, 16));
}

// 0 1
// 0 1 2 3
// 0 1 2 3 4 5 6 7
// 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15

pub(crate) fn hash<K>(hash_builder: &RandomState, val: &K, size: usize) -> u64
where
    K: Hash,
{
    let mut hasher = hash_builder.build_hasher();
    val.hash(&mut hasher);
    return hasher.finish() & (size as u64 - 1);
}

pub fn get_next_idx(mut idx: u64, len: u64) -> u64 {
    idx |= !len + 1;
    idx = rev(idx);
    idx += 1;
    idx = rev(idx);
    return idx;
}

fn rev(mut index: u64) -> u64 {
    let mut rev = 0;
    let mut i = 0;
    while i < 64 && index != 0 {
        rev |= (index & 1) << (63 - i);
        index >>= 1;
        i += 1;
    }
    return rev;
}
