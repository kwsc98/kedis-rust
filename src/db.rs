use crate::{command::Command, frame::Frame, structure::dict::Dict};
use std::vec;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct DbHandler {
    sender_list: Vec<crate::MpscSender>,
}

#[derive(Debug)]
pub struct Db {
    dict: Dict<String, Structure>,
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
                _ => Err("Error".into()),
            };
            let _ = match frame {
                Ok(frame) => sender.send(frame),
                Err(err) => sender.send(Frame::Error(err.to_string())),
            };
        }
    }
    pub fn get_dict(&mut self)-> &mut Dict<String, Structure>{
        return &mut self.dict;
    }
}
