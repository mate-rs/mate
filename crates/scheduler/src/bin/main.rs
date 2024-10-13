use std::time::Duration;

use anyhow::Result;
use mate::scheduler::{backend::redis::RedisBackend, Scheduler, SchedulerBackend};
use tokio::time::sleep;
use tracing::{error, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let backend = RedisBackend::new(String::from("redis://127.0.0.1:6379/")).await?;
    let scheduler = Scheduler::new(backend);

    loop {
        sleep(Duration::from_secs(1)).await;

        match scheduler.pop().await {
            Ok(jobs) => {
                if jobs.is_empty() {
                    continue;
                }

                for job in jobs {
                    job.dispatch();
                }
            }
            Err(err) => {
                error!("Failed to pop job from queue: {:?}", err);
            }
        }
    }
}
