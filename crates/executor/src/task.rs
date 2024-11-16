use anyhow::Result;
use tokio::sync::mpsc::Receiver;
use tracing::{error, info};

use mate_proto::Job;

use crate::{storage::SharedStorage, Executor};

pub struct ExecutorTask {
    storage: SharedStorage,
    main_process_rx: Receiver<Vec<Job>>,
}

impl ExecutorTask {
    pub async fn new(storage: SharedStorage, main_process_rx: Receiver<Vec<Job>>) -> Result<Self> {
        Ok(Self {
            storage,
            main_process_rx,
        })
    }

    pub async fn run(&mut self) {
        while let Some(jobs) = self.main_process_rx.recv().await {
            info!(?jobs, "Received jobs from main process");
            for job in jobs {
                match self.execute(&job).await {
                    Ok(_) => {
                        info!("Executed job {} with success", job.id);
                    }
                    Err(err) => {
                        error!("Failed to execute job {}. {err}", job.id);
                    }
                }
            }
        }
    }

    async fn execute(&self, job: &Job) -> Result<()> {
        let task = self.storage.get_task(&job.id).await?;
        let executor = Executor::new();

        executor.execute(job, task).await?;

        Ok(())
    }
}
