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
