use std::{io::Write, net::TcpListener};
use std::net::TcpStream;

use anyhow::Result;
use bson::Document;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::SCHEDULER_PORT;

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    List,
}

#[derive(Clone, Debug)]
pub struct Client;

impl Client {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    pub async fn send(&self, cmd: Command) -> Result<()> {
        let mut stream = TcpStream::connect(format!("127.0.0.1:{}", SCHEDULER_PORT))?;

        info!("Sending {:?}", cmd);
        let mut doc = Document::new();
        doc.insert("cmd", bson::to_bson(&cmd)?);
        let mut buf = Vec::new();
        doc.to_writer(&mut buf).unwrap();
        stream.write_all(&buf)?;
        Ok(())
    }
}
