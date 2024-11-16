use std::{sync::Arc, time::Duration};

use anyhow::Result;
use tokio::{sync::mpsc::Sender, time::sleep};

use mate_proto::Job;
use tracing::error;

use crate::backend::redis::RedisBackend;
use crate::Scheduler;

pub struct SchedulerTask {
    scheduler: Arc<Scheduler<RedisBackend>>,
    main_process_tx: Sender<Vec<Job>>,
}

impl SchedulerTask {
    pub async fn new(
        main_process_tx: Sender<Vec<Job>>,
        scheduler: Arc<Scheduler<RedisBackend>>,
    ) -> Result<Self> {
        Ok(Self {
            scheduler,
            main_process_tx,
        })
    }

    pub async fn run(&self) {
        loop {
            sleep(Duration::from_secs(1)).await;

            match self.scheduler.pop().await {
                Ok(jobs) => {
                    if jobs.is_empty() {
                        continue;
                    }

                    if let Err(err) = self.main_process_tx.send(jobs).await {
                        error!(%err, "Failed to send jobs to main process");
                    }
                }
                Err(err) => {
                    error!(%err, "Failed to pop jobs from scheduler");
                }
            }
        }
    }
}
