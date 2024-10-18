use std::{fmt::Display, sync::Arc, time::Duration};

use anyhow::Result;
use bson::Document;
use serde::{Deserialize, Serialize};
use tokio::{
    io::AsyncWriteExt,
    net::{TcpStream, ToSocketAddrs},
    sync::Mutex,
    time::sleep,
};
use tracing::{debug, info};

const MAX_CONNECTION_ATTEMPTS: u32 = 10;

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    List,
}

#[derive(Clone, Debug)]
pub struct SocketClient {
    socket: Arc<Mutex<TcpStream>>,
}

impl SocketClient {
    pub async fn new<A: ToSocketAddrs + Clone + Display>(addr: A) -> Result<Self> {
        let mut attempts = 1;
        let socket = {
            while attempts < MAX_CONNECTION_ATTEMPTS {
                sleep(Duration::from_secs(1)).await;

                match TcpStream::connect(addr.clone()).await {
                    Ok(_) => {
                        debug!(
                            attempts,
                            max_attempts = MAX_CONNECTION_ATTEMPTS,
                            %addr,
                            "Connected to SocketClient...",
                        );
                        break;
                    }
                    Err(err) => {
                        attempts += 1;
                        debug!(
                            attempts,
                            max_attempts = MAX_CONNECTION_ATTEMPTS,
                            %addr,
                            %err,
                            "Failed to connect to SocketClient...",
                        );
                    }
                }
            }

            Arc::new(Mutex::new(TcpStream::connect(addr).await?))
        };

        debug!(
            attempts,
            max_attempts = MAX_CONNECTION_ATTEMPTS,
            "SocketClient is Ready...",
        );

        Ok(Self { socket })
    }

    pub async fn send(&self, cmd: Command) -> Result<()> {
        info!("Sending command to scheduler...");
        let mut doc = Document::new();
        doc.insert("cmd", bson::to_bson(&cmd)?);
        let mut buf = Vec::new();
        doc.to_writer(&mut buf).unwrap();
        let mut socket = self.socket.lock().await;
        socket.write_all(&buf).await?;
        Ok(())
    }
}
