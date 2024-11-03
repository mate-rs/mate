use std::{path::PathBuf, process, time::Duration};

use anyhow::Result;
use clap::Parser;

use tokio::time::sleep;
use tracing::{error, info};

use mate::scheduler::{backend::redis::RedisBackend, Scheduler, SchedulerBackend};
use mate_fifo::{
    proto::{MainReply, Message, SchedulerRequest},
    NPipeHandle,
};

#[derive(Debug, Parser)]
#[command(
    name = "mate-scheduler",
    author = "Esteban Borai <estebanborai@gmail.com> (https://github.com/mate-rs/mate)",
    next_line_help = true
)]
pub struct MateSchedulerCli {
    #[clap(long)]
    pub main_pipe: PathBuf,
    #[clap(long)]
    pub scheduler_pipe: PathBuf,
    #[clap(long)]
    pub redis_url: String,
}

impl MateSchedulerCli {
    pub async fn exec(self) -> Result<()> {
        tokio::select! {
            _ = listen(&self.main_pipe, &self.scheduler_pipe, &self.redis_url) => {},
            _ = dispatch(&self.main_pipe, &self.redis_url) => {},
        }

        Ok(())
    }
}

async fn listen(main_pipe: &PathBuf, scheduler_pipe: &PathBuf, redis_url: &String) -> Result<()> {
    let main_pipe = NPipeHandle::new(main_pipe).await?;
    let scheduler_pipe = NPipeHandle::new(scheduler_pipe).await?;
    let backend = RedisBackend::new(redis_url.to_owned()).await?;
    let scheduler = Scheduler::new(backend);

    loop {
        if let Message::SchedulerRequest(req) = scheduler_pipe.recv().await? {
            match req {
                SchedulerRequest::ListJobs => match scheduler.list().await {
                    Ok(jobs) => {
                        info!(?jobs, "Listed jobs");
                        main_pipe
                            .send(&Message::MainReply(MainReply::ListJobs(jobs)))
                            .await?
                    }
                    Err(err) => {
                        error!(%err, "Failed to list jobs");
                    }
                },
                SchedulerRequest::Exit => {
                    info!("Exiting...");
                    main_pipe
                        .send(&Message::MainReply(MainReply::SchedulerExited))
                        .await?;
                    process::exit(0);
                }
            }
        }
    }
}

async fn dispatch(main_pipe: &PathBuf, redis_url: &String) -> Result<()> {
    let main_pipe = NPipeHandle::new(main_pipe).await?;
    let backend = RedisBackend::new(redis_url.to_owned()).await?;
    let scheduler = Scheduler::new(backend);

    loop {
        sleep(Duration::from_secs(1)).await;

        match scheduler.pop().await {
            Ok(jobs) => {
                if jobs.is_empty() {
                    continue;
                }

                if let Err(err) = main_pipe
                    .send(&Message::Text(format!("Jobs found: {}", jobs.len())))
                    .await
                {
                    error!(%err, "Failed to send message to fifo");
                }
            }
            Err(err) => {
                if let Err(err) = main_pipe
                    .send(&Message::Text(format!("Failed to fetch jobs: {}", err)))
                    .await
                {
                    error!(%err, "Failed to send message to fifo");
                }
            }
        }
    }
}
