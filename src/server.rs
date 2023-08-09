use std::{future::Future, sync::Arc, time::Duration};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{
        broadcast::{self},
        mpsc::{self},
        Semaphore,
    }, time,
};
use tracing::{error, info};

use crate::{MAX_CONNECTIONS, buffer::Buffer};

#[derive(Debug)]
struct Listener {
    listener: TcpListener,
    limit_connections: Arc<Semaphore>,
    notify_shutdown: broadcast::Sender<()>,
    shutdown_complete_tx: mpsc::Sender<()>,
    shutdown_complete_rx: mpsc::Receiver<()>,
}

struct Handler {
    buffer: Buffer,
    limit_connections: Arc<Semaphore>,
    _shutdown_complete: mpsc::Sender<()>,
}



pub async fn run(listener: TcpListener, shutdown: impl Future) {
    let (shutdown_complete_tx, shutdown_complete_rx) = mpsc::channel(1);
    let mut server = Listener {
        listener,
        limit_connections: Arc::new(Semaphore::new(MAX_CONNECTIONS)),
        notify_shutdown: broadcast::channel(1).0,
        shutdown_complete_tx,
        shutdown_complete_rx,
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
           let handler = Handler {
                buffer : Buffer {  },
                limit_connections : self.limit_connections.clone(),
                _shutdown_complete : self.shutdown_complete_tx.clone()
           };
           tokio::spawn(async move {
              // Process the connection. If an error is encountered, log it.
              if let Err(err) = handler.run().await {
                  error!(cause = ?err, "connection error");
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
