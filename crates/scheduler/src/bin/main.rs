use std::io::Read;
use std::{net::TcpListener, time::Duration};

use anyhow::Result;
use tokio::time::sleep;
use tracing::{error, Level};
use tracing_subscriber::FmtSubscriber;

use mate::scheduler::{backend::redis::RedisBackend, Scheduler, SchedulerBackend};
use mate::SCHEDULER_PORT;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    tokio::select! {
        _ = listen() => {},
        _ = dispatch() => {},
    }

    Ok(())
}

async fn listen() -> Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", SCHEDULER_PORT))?;

    loop {
        let (mut stream, _) = listener.accept()?;

        tokio::spawn(async move {
            let buf = &mut Vec::new();
            stream.read_to_end(buf).unwrap();
            println!("{:?}", buf);
        });
    }
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
            Err(err) => {
                error!("Failed to pop job from queue: {:?}", err);
            }
        }
    }
}
