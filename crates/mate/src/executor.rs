use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use tokio::time::sleep;

use crate::scheduler::{Scheduler, SchedulerBackend};

pub struct Executor<SB: SchedulerBackend> {
    pub(crate) scheduler: Arc<Scheduler<SB>>,
    pub(crate) threshold: Duration,
}

impl<SB: SchedulerBackend> Executor<SB> {
    pub fn new(scheduler: Arc<Scheduler<SB>>, threshold: Duration) -> Self {
        Self {
            scheduler,
            threshold,
        }
    }

    pub async fn run(&self) -> Result<()> {
        loop {
            sleep(self.threshold).await;

            match self.scheduler.pop().await {
                Ok(jobs) => {
                    if jobs.is_empty() {
                        continue;
                    }
                }
                Err(_) => {
                    // error!("Failed to pop job from queue: {:?}", err);
                }
            }
        }
    }
}
