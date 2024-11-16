use std::path::PathBuf;
use std::process;
use std::sync::Arc;

use anyhow::Result;
use mate_proto::Job;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Mutex;
use tracing::{error, info};

use mate_fifo::message::{MainReply, Message, SchedulerRequest};
use mate_fifo::NPipeHandle;

use mate_scheduler::backend::redis::RedisBackend;
use mate_scheduler::Scheduler;

pub struct IpcServer {
    main_pipe: NPipeHandle,
    executor_pipe: NPipeHandle,
    scheduler_pipe: NPipeHandle,
    rx: Mutex<Receiver<Vec<Job>>>,
}

impl IpcServer {
    pub async fn new(
        main_pipe: &PathBuf,
        scheduler_pipe: &PathBuf,
        executor_pipe: &PathBuf,
    ) -> Result<(Self, Sender<Vec<Job>>)> {
        let main_pipe = NPipeHandle::new(main_pipe).await?;
        let executor_pipe = NPipeHandle::new(executor_pipe).await?;
        let scheduler_pipe = NPipeHandle::new(scheduler_pipe).await?;
        let (tx, rx) = channel::<Vec<Job>>(1024);
        let rx = Mutex::new(rx);

        Ok((
            Self {
                main_pipe,
                executor_pipe,
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

        while let Some(jobs) = rx.recv().await {
            match self
                .executor_pipe
                .send(&Message::ExecutorRequest(
                    mate_fifo::message::ExecutorRequest::ExecuteJobs(jobs),
                ))
                .await
            {
                Ok(_) => {
                    info!("Sent jobs to executor");
                }
                Err(err) => {
                    error!(%err, "Failed to send jobs to executor");
                }
            }
        }

        Ok(())
    }
}
