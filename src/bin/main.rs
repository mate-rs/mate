use std::time::Duration;

use anyhow::Result;

use mate::scheduler::redis::RedisBackend;
use mate::scheduler::{Scheduler, SchedulerBackend};
use mate::Mate;

#[tokio::main]
async fn main() -> Result<()> {
    let backend = RedisBackend::new(String::from("redis://127.0.0.1:6379/")).await?;
    let scheduler = Scheduler::new(backend);
    let mate = Mate::new(scheduler, Duration::from_secs(1));

    tokio::select! {
        _ = mate.run() => {},
        _ = mate.repl() => {},
    }

    Ok(())
}
