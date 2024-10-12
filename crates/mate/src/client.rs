use std::sync::Arc;

use anyhow::Result;

use crate::executor::Executor;
use crate::job::Job;
use crate::scheduler::{Scheduler, SchedulerBackend};

pub struct Client<SB: SchedulerBackend> {
    pub(crate) executor: Arc<Executor<SB>>,
    pub(crate) scheduler: Arc<Scheduler<SB>>,
}

impl<SB: SchedulerBackend> Client<SB> {
    pub fn new(executor: Arc<Executor<SB>>, scheduler: Arc<Scheduler<SB>>) -> Self {
        Self {
            executor,
            scheduler,
        }
    }

    pub async fn enqueue(&self, job: Job) -> Result<()> {
        self.scheduler.push(job).await
    }

    pub async fn dequeue(&self) -> Option<String> {
        todo!()
    }

    pub async fn list(&self) -> Result<Vec<Job>> {
        self.scheduler.list().await
    }
}
