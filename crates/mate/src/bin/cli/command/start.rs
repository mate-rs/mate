use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use mate::{client::SocketClient, repl::Repl, SCHEDULER_PORT};

const MATE_SCHEDULER_BIN: &str = "./target/debug/mate-scheduler";

#[derive(Clone, Debug, Args)]
pub struct StartOpt {
    /// URL to connect to Redis
    #[clap(long, default_value = "redis://127.0.0.1:6379/")]
    redis_url: String,
    /// Scheduler threshold seconds
    #[clap(long, default_value = "1")]
    threshold: u64,
}

impl StartOpt {
    pub async fn exec(&self) -> Result<()> {
        let _scheduler_task = tokio::spawn(async move {
            tokio::process::Command::new(PathBuf::from(MATE_SCHEDULER_BIN))
                // .stdout(Stdio::null())
                // .stderr(Stdio::null())
                .spawn()
                .expect("Failed to spawn process for mate-scheduler");
        });
        let sc = SocketClient::new(format!("127.0.0.1:{}", SCHEDULER_PORT)).await?;
        let repl = Repl::new(sc.clone());

        repl.start().await?;
        Ok(())
    }
}
