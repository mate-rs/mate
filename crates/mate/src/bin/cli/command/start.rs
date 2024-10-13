use std::time::Duration;

use anyhow::Result;
use clap::Args;

use mate::scheduler::backend::redis::RedisBackend;
use mate::scheduler::{Scheduler, SchedulerBackend};
use mate::Mate;

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
        let backend = RedisBackend::new(self.redis_url.clone()).await?;
        let scheduler = Scheduler::new(backend);
        let mate = Mate::new(scheduler, Duration::from_secs(self.threshold));

        tokio::select! {
            _ = mate.run() => {},
            _ = mate.repl() => {},
        }

        Ok(())
    }
}
