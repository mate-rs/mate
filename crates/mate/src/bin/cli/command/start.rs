use std::path::PathBuf;
use std::process::Stdio;

use anyhow::Result;
use clap::Args;

use mate::scheduler::SchedulerBackend;
use mate::Mate;

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
        let mate = Mate::new()?;

        tokio::select! {
            _ = self.spawn(MATE_SCHEDULER_BIN.into()) => {},
            _ = mate.repl() => {},
        }

        Ok(())
    }

    async fn spawn(&self, bin: PathBuf) -> Result<()> {
        tokio::spawn(async move {
            tokio::process::Command::new(bin)
                // .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .expect("Failed to spawn process for mate-scheduler");
        })
        .await?;

        Ok(())
    }
}
