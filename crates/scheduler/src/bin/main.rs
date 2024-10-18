use std::time::Duration;

use anyhow::Result;
use tokio::time::sleep;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use mate::scheduler::{backend::redis::RedisBackend, Scheduler, SchedulerBackend};
use mate_scheduler::server::Server;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    println!("Spawning processes");
    tokio::select! {
        _ = listen() => {},
        _ = dispatch() => {},
    }

    Ok(())
}

async fn listen() -> Result<()> {
    let server = Server::new().await?;
    server.run().await
}

async fn dispatch() -> Result<()> {
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
            Err(_) => {
                // error!("Failed to pop job from queue: {:?}", err);
            }
        }
    }
}
