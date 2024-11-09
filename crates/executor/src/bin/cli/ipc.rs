use std::path::PathBuf;

use anyhow::Result;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Mutex;

use mate_fifo::message::{ExecutorRequest, Message};
use mate_fifo::NPipeHandle;

pub struct IpcServer {
    main_pipe: NPipeHandle,
    executor_pipe: NPipeHandle,
    rx: Mutex<Receiver<String>>,
}

impl IpcServer {
    pub async fn new(
        main_pipe: &PathBuf,
        executor_pipe: &PathBuf,
    ) -> Result<(Self, Sender<String>)> {
        let main_pipe = NPipeHandle::new(main_pipe).await?;
        let executor_pipe = NPipeHandle::new(executor_pipe).await?;
        let (tx, rx) = channel::<String>(1024);
        let rx = Mutex::new(rx);

        Ok((
            Self {
                main_pipe,
                executor_pipe,
                rx,
            },
            tx,
        ))
    }

    pub async fn listen(&self) -> Result<()> {
        tokio::select! {
            _ = self.handle_ipc() => {},
            _ = self.handle_main() => {},
        }

        Ok(())
    }

    async fn handle_ipc(&self) -> Result<()> {
        loop {
            if let Message::ExecutorRequest(req) = self.executor_pipe.recv().await? {
                match req {
                    ExecutorRequest::ExecuteJob(job) => {
                        println!("Received job: {job:#?}");
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
