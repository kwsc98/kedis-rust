use crate::{command::Command, structure::dict::Dict};
use std::{hash::Hash, vec};
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct DbHandler {
    sender_list: Vec<crate::MpscSender>,
}

#[derive(Debug)]
pub struct KedisKey {
    key: String,
    ttl: i64,
}


impl KedisKey {
    pub fn new(key: String) -> Self {
        return KedisKey { key, ttl: -1 };
    }
    pub fn set_ttl(&mut self, ttl: i64) {
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
        return self.dict.get_pattern_entry(pre_idx, match_str, count);
    }
    pub fn insert(&mut self, key: KedisKey, value: Structure) -> Option<Structure> {
        return self.dict.insert(key, value);
    }

    pub fn get(&self, key: &KedisKey) -> Option<&Structure> {
        return self.dict.get(key);
    }

    pub fn get_mut(&mut self, key: &KedisKey) -> Option<&mut Structure> {
        return self.dict.get_mut(key);
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
