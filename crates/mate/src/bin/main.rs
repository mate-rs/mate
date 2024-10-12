use std::env::var;
use std::time::Duration;

use anyhow::Result;

use mate::scheduler::Scheduler;
use mate::Mate;

#[tokio::main]
async fn main() -> Result<()> {
    let redis_url = var("REDIS_URL").map_err(|_| anyhow::anyhow!("REDIS_URL is not set"))?;
    let backend = RedisBackend::new(redis_url).await?;
    let scheduler = Scheduler::new(backend);
    let mate = Mate::new(scheduler, Duration::from_secs(1));

    tokio::select! {
        _ = mate.run() => {},
        _ = mate.repl() => {},
    }

    Ok(())
}
