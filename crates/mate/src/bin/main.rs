mod cli;

use anyhow::Result;
use clap::Parser;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use self::cli::MateCli;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let args = MateCli::parse();
    args.exec().await?;

    Ok(())
}

// async fn listen(path: &PathBuf) -> Result<()> {
//     let mut handle = NPipeHandle::new(&path).await?;
//     println!("Listening on: {}", path.display());
//     loop {
//         match handle.recv().await? {
//             Message::Text(p) => {
//                 println!("<< {}", p);
//                 handle.send(&Message::Ack).await?;
//                 sleep(Duration::from_millis(500)).await;
//             },
//             Message::Ack => panic!("Didn't expect Ack now."),
//         }
//     }
// }

// async fn send(path: &PathBuf, s: String) -> Result<()> {
//     let mut handle = NPipeHandle::new(&path).await?;
//     handle.send(&Message::Text(s)).await?;
//     sleep(Duration::from_millis(200)).await;
//     match handle.recv().await? {
//         Message::Text(p) => println!(">> {}", p),
//         Message::Ack => println!("Ack received."),
//     }
//     Ok(())
// }
