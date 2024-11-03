mod cli;

use anyhow::Result;
use clap::Parser;
use cli::MateSchedulerCli;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    MateSchedulerCli::parse().exec().await
}
