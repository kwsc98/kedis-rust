use std::{future::Future, sync::Arc, time::Duration};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{
        broadcast::{self},
        mpsc::{self},
        oneshot, Semaphore,
    },
    time,
};
use tracing::{debug, error, info};

use crate::{buffer::Buffer, command::Command, shutdown::Shutdown, MAX_CONNECTIONS};
use crate::{db::DbHandler, frame::Frame};

#[derive(Debug)]
struct Listener {
    listener: TcpListener,
    limit_connections: Arc<Semaphore>,
    notify_shutdown: broadcast::Sender<()>,
    shutdown_complete_tx: mpsc::Sender<()>,
    shutdown_complete_rx: mpsc::Receiver<()>,
    db_handler: Arc<DbHandler>,
}

pub struct Handler {
    buffer: Buffer,
    limit_connections: Arc<Semaphore>,
    shutdown: Shutdown,
    _shutdown_complete: mpsc::Sender<()>,
    db_sender: crate::MpscSender,
    db_handler: Arc<DbHandler>,
}

pub async fn run(listener: TcpListener, shutdown: impl Future, db_size: usize) {
    let (shutdown_complete_tx, shutdown_complete_rx) = mpsc::channel(1);
    let mut server = Listener {
        listener,
        limit_connections: Arc::new(Semaphore::new(MAX_CONNECTIONS)),
        notify_shutdown: broadcast::channel(1).0,
        shutdown_complete_tx,
        shutdown_complete_rx,
        db_handler: Arc::new(DbHandler::new(db_size)),
    };
    tokio::select! {
       res = server.run() => {
          if let Err(err) = res {
             error!(cause = %err, "failed to accept");
         }
       },
       _ = shutdown => {
             info!("server shutting down");
       }
    }
    let Listener {
        mut shutdown_complete_rx,
        shutdown_complete_tx,
        notify_shutdown,
        ..
    } = server;
    drop(notify_shutdown);
    drop(shutdown_complete_tx);
    let _ = shutdown_complete_rx.recv().await;
}
impl Listener {
    async fn run(&mut self) -> crate::Result<()> {
        loop {
            self.limit_connections.acquire().await.unwrap().forget();
            let socket = self.accept().await?;
            debug!("receive new connect");
            let mut handler = Handler {
                buffer: Buffer::new(socket),
                shutdown: Shutdown::new(self.notify_shutdown.subscribe()),
                limit_connections: self.limit_connections.clone(),
                _shutdown_complete: self.shutdown_complete_tx.clone(),
                db_sender: self.db_handler.as_ref().get_sender(0).unwrap(),
                db_handler: self.db_handler.clone(),
            };
            tokio::spawn(async move {
                if let Err(err) = handler.run().await {
                    error!(cause = ?err, "handler error");
                }
            });
        }
    }
    async fn accept(&mut self) -> crate::Result<TcpStream> {
        let mut backoff = 1;
        loop {
            match self.listener.accept().await {
                Ok((socket, _)) => return Ok(socket),
                Err(err) => {
                    if backoff > 64 {
                        return Err(err.into());
                    }
                }
            }
            time::sleep(Duration::from_secs(backoff)).await;
            backoff *= 2;
        }
    }
}
impl Handler {
    async fn run(&mut self) -> crate::Result<()> {
        while !self.shutdown.is_shutdown() {
            let frame = tokio::select! {
                res = self.buffer.read_frame() => res?,
                _ = self.shutdown.recv() => {
                    return Ok(());
                }
            };
            if let Some(frame) = frame {
                let cmd = Command::from_frame(frame)?;
                let fist_do = match &cmd {
                    Command::Unknown(unknown) => Some(unknown.apply()),
                    Command::Info(info) => Some(info.apply()),
                    Command::Ping(ping) => Some(ping.apply()),
                    Command::Select(select) => Some(select.apply(self)),
                    Command::Config(config) => Some(config.apply(self)),
                    _ => None,
                };
                let frame;
                if let Some(result) = fist_do {
                    frame = match result {
                        Ok(frame) => frame,
                        Err(err) => Frame::Error(err.to_string()),
                    };
                } else {
                    let (sender, receiver) = oneshot::channel();
                    self.db_sender.send((sender, cmd)).await?;
                    frame = receiver.await?;
                }
                self.buffer.write_frame(&frame).await?;
            }
        }
        return Ok(());
    }

    pub fn change_db_sender(&mut self, idx: usize) -> crate::Result<()> {
        let sender_list = self
            .db_handler
            .get_sender(idx)
            .ok_or("ERR invalid DB index")?;
        self.db_sender = sender_list;
        return Ok(());
    }

    pub fn get_db_size(&mut self) -> usize {
        return self.db_handler.get_size();
    }
}
impl Drop for Handler {
    fn drop(&mut self) {
        self.limit_connections.add_permits(1);
    }
}
