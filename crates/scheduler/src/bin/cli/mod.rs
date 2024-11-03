use std::{path::PathBuf, time::Duration};

use anyhow::Result;
use clap::Parser;

use tokio::time::sleep;
use tracing::{error, info};

use mate::scheduler::{backend::redis::RedisBackend, Scheduler, SchedulerBackend};
use mate_fifo::{proto::Message, NPipeHandle};

#[derive(Debug, Parser)]
#[command(
    name = "mate-scheduler",
    author = "Esteban Borai <estebanborai@gmail.com> (https://github.com/mate-rs/mate)",
    next_line_help = true
)]
pub struct MateSchedulerCli {
    #[clap(long = "main-pipe")]
    pub main_pipe: PathBuf,
    #[clap(long = "scheduler-pipe")]
    pub scheduler_pipe: PathBuf,
}

impl MateSchedulerCli {
    pub async fn exec(self) -> Result<()> {
        tokio::select! {
            _ = listen(&self.main_pipe, &self.scheduler_pipe) => {},
            _ = dispatch(&self.main_pipe) => {},
        }

        Ok(())
    }
}

async fn listen(main_pipe: &PathBuf, scheduler_pipe: &PathBuf) -> Result<()> {
    let main_pipe = NPipeHandle::new(&main_pipe).await?;
    let scheduler_pipe = NPipeHandle::new(&scheduler_pipe).await?;

    loop {
        match scheduler_pipe.recv().await? {
            Message::Text(p) => {
                println!("<< {}", p);
                main_pipe.send(&Message::Ack).await?;
                sleep(Duration::from_millis(500)).await;
            }
            Message::Ack => panic!("Didn't expect Ack now."),
        }
    }
}

async fn dispatch(main_pipe: &PathBuf) -> Result<()> {
    let main_pipe = NPipeHandle::new(&main_pipe).await?;
    let backend = RedisBackend::new(String::from("redis://127.0.0.1:6379/")).await?;
    let scheduler = Scheduler::new(backend);

    loop {
        sleep(Duration::from_secs(1)).await;

        match scheduler.pop().await {
            Ok(jobs) => {
                if jobs.is_empty() {
                    continue;
                }

                if let Err(err) = main_pipe.send(&Message::Text(format!("Jobs found: {}", jobs.len()))).await {
                    error!(%err, "Failed to send message to fifo");
                }

                for job in jobs {
                    job.dispatch();
                }
            }
            Err(err) => {
                if let Err(err) = main_pipe.send(&Message::Text(format!("Failed to fetch jobs: {}", err))).await {
                    error!(%err, "Failed to send message to fifo");
                }
            }
        }
    }
}