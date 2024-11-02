mod cli;

use std::{path::PathBuf, str::FromStr, time::Duration};

use anyhow::Result;
use tokio::time::sleep;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use mate_fifo::{NPipe, NPipeHandle};
use mate_fifo::proto::Message;

use self::cli::MateCli;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // let args = MateCli::parse();

    // args.exec().await?;

    let mut args = std::env::args();
    let _ = args.next();

    match args.next().as_ref().map(String::as_str) {
        Some("listen") => {
            let npipe = NPipe::new("rust-fifo-test")?;
            listen(npipe.path()).await?
        },
        Some("send") => {
            let msg = args.next().unwrap();
            let path = args.next().unwrap();
            info!("Sending message: {} to {}", msg, path);
            send(&PathBuf::from_str(&path).unwrap(), msg).await?;
        }
        _ => {
            eprintln!("Please either listen or send.");
        }
    }
    Ok(())
}

async fn listen(path: &PathBuf) -> Result<()> {
    let mut handle = NPipeHandle::new(&path).await?;
    println!("Listening on: {}", path.display());
    loop {
        match handle.recv().await? {
            Message::Text(p) => {
                println!("<< {}", p);
                handle.send(&Message::Ack).await?;
                sleep(Duration::from_millis(500)).await;
            },
            Message::Ack => panic!("Didn't expect Ack now."),
        }
    }
}

async fn send(path: &PathBuf, s: String) -> Result<()> {
    let mut handle = NPipeHandle::new(&path).await?;
    handle.send(&Message::Text(s)).await?;
    sleep(Duration::from_millis(200)).await;
    match handle.recv().await? {
        Message::Text(p) => println!(">> {}", p),
        Message::Ack => println!("Ack received."),
    }
    Ok(())
}
