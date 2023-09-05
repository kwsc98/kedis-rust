use crate::{command::Command, frame::Frame, structure::dict::Dict};
use std::vec;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
pub struct DbHandler {
    sender_list: Vec<mpsc::Sender<(oneshot::Sender<Frame>, Command)>>,
}

#[derive(Debug)]
pub struct Db {
    dict: Dict<String, Structure>,
    sender: mpsc::Sender<(oneshot::Sender<Frame>, Command)>,
    receiver: mpsc::Receiver<(oneshot::Sender<Frame>, Command)>,
}

#[derive(Debug)]
enum Structure {
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
    pub fn get_sender(
        &mut self,
        idx: usize,
    ) -> Option<mpsc::Sender<(oneshot::Sender<Frame>, Command)>> {
        return self.sender_list.get_mut(idx).map(|item| item.clone());
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
        while let Some((sender, common)) = self.receiver.recv().await {
            let frame = match common {
                Command::Get(get) => get.apply(&self),
                Command::Set(set) => set.apply(&self),
                Command::Unknown(unknow) => unknow.apply(&self),
            };
            if let Some(frame) = frame {
                let _ = sender.send(frame);
            } else {
                let _ = sender.send(Frame::Error("Error".to_string()));
            }
        }
    }
}
