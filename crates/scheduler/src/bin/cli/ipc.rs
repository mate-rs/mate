use std::path::PathBuf;
use std::process;
use std::sync::Arc;

use anyhow::Result;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Mutex;
use tracing::{error, info};

use mate_fifo::message::{MainReply, Message, SchedulerRequest};
use mate_fifo::NPipeHandle;

use mate_scheduler::Scheduler;
use mate_scheduler::backend::redis::RedisBackend;

pub struct IpcServer {
    main_pipe: NPipeHandle,
    scheduler_pipe: NPipeHandle,
    rx: Mutex<Receiver<String>>,
}

impl IpcServer {
    pub async fn new(
        main_pipe: &PathBuf,
        scheduler_pipe: &PathBuf,
    ) -> Result<(Self, Sender<String>)> {
        let main_pipe = NPipeHandle::new(main_pipe).await?;
        let scheduler_pipe = NPipeHandle::new(scheduler_pipe).await?;
        let (tx, rx) = channel::<String>(1024);
        let rx = Mutex::new(rx);

        Ok((
            Self {
                main_pipe,
                scheduler_pipe,
                rx,
            },
            tx,
        ))
    }

    pub async fn listen(&self, scheduler: Arc<Scheduler<RedisBackend>>) -> Result<()> {
        tokio::select! {
            _ = self.handle_ipc(scheduler) => {},
            _ = self.handle_main() => {},
        }

        Ok(())
    }

    async fn handle_ipc(&self, scheduler: Arc<Scheduler<RedisBackend>>) -> Result<()> {
        loop {
            if let Message::SchedulerRequest(req) = self.scheduler_pipe.recv().await? {
                match req {
                    SchedulerRequest::PushJob(job) => match scheduler.push(job).await {
                        Ok(_) => {
                            info!("Created job");
                            self.main_pipe
                                .send(&Message::MainReply(MainReply::JobCreated(String::from(
                                    "JOB ID",
                                ))))
                                .await?
                        }
                        Err(err) => {
                            error!(%err, "Failed to create job");
                            self.main_pipe
                                .send(&Message::MainReply(MainReply::Error(err.to_string())))
                                .await?
                        }
                    },
                    SchedulerRequest::PopJob => match scheduler.pop().await {
                        Ok(jobs) => {
                            info!("Popped job");
                            self.main_pipe
                                .send(&Message::MainReply(MainReply::JobPopped(jobs)))
                                .await?
                        }
                        Err(err) => {
                            error!(%err, "Failed to create job");
                            self.main_pipe
                                .send(&Message::MainReply(MainReply::Error(err.to_string())))
                                .await?
                        }
                    },
                    SchedulerRequest::ListJobs => match scheduler.list().await {
                        Ok(jobs) => {
                            info!(?jobs, "Listed jobs");
                            self.main_pipe
                                .send(&Message::MainReply(MainReply::ListJobs(jobs)))
                                .await?
                        }
                        Err(err) => {
                            error!(%err, "Failed to list jobs");
                            self.main_pipe
                                .send(&Message::MainReply(MainReply::Error(err.to_string())))
                                .await?
                        }
                    },
                    SchedulerRequest::Exit => {
                        info!("Exiting...");
                        self.main_pipe
                            .send(&Message::MainReply(MainReply::SchedulerExited))
                            .await?;
                        process::exit(0);
                    }
                }
            }
        }
    }

    async fn handle_main(&self) -> Result<()> {
        let mut rx = self.rx.lock().await;

        loop {
            if let Some(msg) = rx.recv().await {
                println!("Got main process message: {msg}")
            } else {
                println!("No message from main process");
            }
        }
    }
}
