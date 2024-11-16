use std::path::PathBuf;
use std::process;

use anyhow::Result;
use mate_executor::storage::SharedStorage;
use mate_proto::Job;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Mutex;

use mate_fifo::message::{ExecutorRequest, MainReply, Message};
use mate_fifo::NPipeHandle;
use tracing::{error, info};

pub struct IpcServer {
    storage: SharedStorage,
    main_pipe: NPipeHandle,
    executor_pipe: NPipeHandle,
    tx: Mutex<Sender<Vec<Job>>>,
}

impl IpcServer {
    pub async fn new(
        storage: SharedStorage,
        main_pipe: &PathBuf,
        executor_pipe: &PathBuf,
    ) -> Result<(Self, Receiver<Vec<Job>>)> {
        let main_pipe = NPipeHandle::new(main_pipe).await?;
        let executor_pipe = NPipeHandle::new(executor_pipe).await?;
        let (tx, rx) = channel::<Vec<Job>>(1024);
        let tx = Mutex::new(tx);

        Ok((
            Self {
                storage,
                main_pipe,
                executor_pipe,
                tx,
            },
            rx,
        ))
    }

    pub async fn listen(&self) -> Result<()> {
        tokio::select! {
            _ = self.handle_ipc() => {},
        }

        Ok(())
    }

    async fn handle_ipc(&self) -> Result<()> {
        loop {
            if let Message::ExecutorRequest(req) = self.executor_pipe.recv().await? {
                match req {
                    ExecutorRequest::ListTasks => match self.storage.list_tasks().await {
                        Ok(tasks) => {
                            self.main_pipe
                                .send(&Message::MainReply(MainReply::TasksList(tasks)))
                                .await?;
                        }
                        Err(err) => {
                            self.main_pipe
                                .send(&Message::MainReply(MainReply::Error(format!(
                                    "Failed to create task: {err}"
                                ))))
                                .await?;
                        }
                    },
                    ExecutorRequest::ExecuteJobs(jobs) => {
                        if let Err(err) = self.tx.lock().await.send(jobs).await {
                            error!("Failed to send job to executor: {:?}", err);
                        }
                    }
                    ExecutorRequest::CreateTask(task) => {
                        match self.storage.create_task(&task).await {
                            Ok(_) => {
                                info!(?task, "Task created");
                                self.main_pipe
                                    .send(&Message::MainReply(MainReply::TaskCreated(task.name)))
                                    .await?;
                            }
                            Err(err) => {
                                self.main_pipe
                                    .send(&Message::MainReply(MainReply::Error(format!(
                                        "Failed to create task: {err}"
                                    ))))
                                    .await?;
                            }
                        }
                    }
                    ExecutorRequest::Exit => {
                        info!("Exiting...");
                        self.main_pipe
                            .send(&Message::MainReply(MainReply::ExecutorExited))
                            .await?;
                        process::exit(0);
                    }
                }
            }
        }
    }
}
