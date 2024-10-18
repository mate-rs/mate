use std::{io::Write, sync::Arc, thread::sleep, time::Duration};

use anyhow::Result;
use bson::Document;
use serde::{Deserialize, Serialize};
use tokio::{
    io::AsyncWriteExt,
    net::{TcpStream, ToSocketAddrs},
    sync::{Mutex, MutexGuard},
};
use tracing::{info, warn};

use crate::SCHEDULER_PORT;

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    List,
}

#[derive(Clone, Debug)]
pub struct Client {
    scheduler: SocketClient,
}

impl Client {
    pub async fn new() -> Result<Self> {
        let scheduler = SocketClient::new(format!("127.0.0.1:{}", SCHEDULER_PORT)).await?;

        Ok(Self { scheduler })
    }

    pub async fn send(&self, cmd: Command) -> Result<()> {
        info!("Sending {:?}", cmd);
        let mut doc = Document::new();
        doc.insert("cmd", bson::to_bson(&cmd)?);
        let mut buf = Vec::new();
        doc.to_writer(&mut buf).unwrap();
        self.scheduler.socket().await.write_all(&buf).await?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct SocketClient {
    socket: Arc<Mutex<TcpStream>>,
}

impl SocketClient {
    pub async fn new<A: ToSocketAddrs + Clone>(addr: A) -> Result<Self> {
        let socket = {
            let mut attempts = 1;

            while attempts < 10 {
                match TcpStream::connect(addr.clone()).await {
                    Ok(_) => break,
                    Err(err) => {
                        warn!(
                            "Failed to connect to scheduler: {:?}, attempt: {} of {}",
                            err, attempts, 5
                        );
                        attempts += 1;
                    }
                }

                sleep(Duration::from_secs(1));
            }

            Arc::new(Mutex::new(TcpStream::connect(addr).await?))
        };

        Ok(Self { socket })
    }

    pub async fn socket(&self) -> MutexGuard<'_, TcpStream> {
        self.socket.lock().await
    }
}
