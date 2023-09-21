use crate::{
    command::Command,
    common,
    structure::{dict::Dict, dict_ht::DictEntry},
};
use common::date_util as DateUtil;
use std::{hash::Hash, vec};
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct DbHandler {
    sender_list: Vec<crate::MpscSender>,
}

#[derive(Debug)]
pub struct KedisKey {
    key: String,
    ttl: i128,
}

impl KedisKey {
    pub fn new(key: String) -> Self {
        return KedisKey { key, ttl: -1 };
    }
    pub fn set_ttl(&mut self, ttl: i128) {
        self.ttl = ttl;
    }
}

impl PartialEq for KedisKey {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
impl Hash for KedisKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}
impl ToString for KedisKey {
    fn to_string(&self) -> String {
        return self.key.clone();
    }
}

#[derive(Debug)]
pub struct Db {
    dict: Dict<KedisKey, Structure>,
    sender: crate::MpscSender,
    receiver: crate::MpscReceiver,
}

#[derive(Debug)]
pub enum Structure {
    String(String),
    Hash,
    List,
    Set,
    SortSet,
}

impl DbHandler {
    pub fn new(size: usize) -> Self {
        let mut db_list = vec![];
        let mut sender_list = vec![];
        for _idx in 0..size {
            let db = Db::new();
            sender_list.push(db.sender.clone());
            db_list.push(db);
        }
        for mut db in db_list {
            tokio::spawn(async move {
                db.run().await;
            });
        }
        return DbHandler { sender_list };
    }
    pub fn get_sender(&self, idx: usize) -> Option<crate::MpscSender> {
        return self.sender_list.get(idx).map(|item| item.clone());
    }
    pub fn get_size(&self) -> usize {
        return self.sender_list.len();
    }
}

impl Db {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(1024);
        return Db {
            dict: Dict::new(1024),
            sender: tx,
            receiver: rx,
        };
    }
    async fn run(&mut self) {
        while let Some((sender, command)) = self.receiver.recv().await {
            let frame = match command {
                Command::Get(get) => get.apply(self),
                Command::Set(set) => set.apply(self),
                Command::Scan(scan) => scan.apply(self),
                Command::Type(scan) => scan.apply(self),
                Command::Ttl(ttl) => ttl.apply(self),
                _ => Err("Error".into()),
            };
            let _ = sender.send(frame);
        }
    }
    pub fn get_pattern_entry(
        &mut self,
        pre_idx: usize,
        match_str: String,
        count: usize,
    ) -> crate::Result<(usize, Vec<String>)> {
        let item = self.dict.get_pattern_entry(pre_idx, match_str, count)?;
        let mut list = vec![];
        for key in item.1 {
            if !self.remove_expired_key(&KedisKey::new(key.clone())) {
                list.push(key);
            }
        }
        return Ok((item.0, list));
    }
    pub fn insert(&mut self, key: KedisKey, value: Structure) -> Option<Structure> {
        self.remove_expired_key(&key);
        return self.dict.insert(key, value);
    }

    pub fn get(&mut self, key: &KedisKey) -> Option<&Structure> {
        self.remove_expired_key(key);
        let entry = self.dict.get(key)?;
        return entry.value.as_ref();
    }

    pub fn get_mut(&mut self, key: &KedisKey) -> Option<&mut Structure> {
        self.remove_expired_key(key);
        let entry = self.dict.get_mut(key)?;
        return entry.value.as_mut();
    }

    pub fn get_entry(&mut self, key: &KedisKey) -> Option<&DictEntry<KedisKey, Structure>> {
        self.remove_expired_key(key);
        return self.dict.get(key);
    }

    fn remove_expired_key(&mut self, key: &KedisKey) -> bool {
        let entry = self.dict.get(key);
        if let Some(entry) = entry {
            if Self::check_expired(entry.key.ttl) {
                self.dict.remove(key);
                return true;
            }
        }
        return false;
    }

    fn check_expired(ttl: i128) -> bool {
        if ttl >= 0 && ttl <= DateUtil::get_now_date_time_as_millis() {
            return true;
        }
        return false;
    }
}

impl Structure {
    pub fn get_type(&self) -> &str {
        return match self {
            Structure::String(_) => "string",
            Structure::Hash => "hash",
            Structure::List => "list",
            Structure::Set => "set",
            Structure::SortSet => "zset",
        };
    }
}

impl KedisKey {
    pub fn get_expired_by_seconds(&self) -> String {
        let mut ttl = self.ttl - DateUtil::get_now_date_time_as_millis();
        if ttl <= 0 {
            ttl = -2;
        } else {
            ttl /= 1000;
        }
        return ttl.to_string();
    }
}
