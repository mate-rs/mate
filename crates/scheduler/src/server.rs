use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use anyhow::Result;

use tokio::{io::AsyncReadExt, net::TcpListener};

use mate::SCHEDULER_PORT;
use tracing::{error, info};

pub struct Server {
    stream: TcpListener,
}

impl Server {
    pub async fn new() -> Result<Self> {
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), SCHEDULER_PORT);
        let stream = TcpListener::bind(socket).await?;

        Ok(Self { stream })
    }

    pub async fn run(&self) -> Result<()> {
        info!("Listening on port {}", SCHEDULER_PORT);

        loop {
            let (mut stream, addr) = self.stream.accept().await?;
            let buf = &mut Vec::new();

            info!(?addr, "Got message");

            if let Err(err) = stream.read_to_end(buf).await {
                error!("Failed to read from stream: {:?}", err);
            }

            info!(?buf, "Message received");
        }
    }
}
