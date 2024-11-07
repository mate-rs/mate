pub mod backend;

use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use async_trait::async_trait;

use mate_proto::{Job, JobId, PushJobDto};

pub const SCHEDULER_JOB_PREFIX: &str = "mate:job";

pub type Timestamp = u128;

pub struct Scheduler<B: SchedulerBackend> {
    backend: B,
}

impl<B: SchedulerBackend> Scheduler<B> {
    pub fn new(backend: B) -> Self {
        Self { backend }
    }

    #[inline]
    pub async fn push(&self, job: PushJobDto) -> Result<JobId> {
        self.backend.push(job).await
    }

    #[inline]
    pub async fn pop(&self) -> Result<Vec<Job>> {
        self.backend.pop().await
    }

    #[inline]
    pub async fn list(&self) -> Result<Vec<Job>> {
        self.backend.list().await
    }
}

#[async_trait]
pub trait SchedulerBackend: Sized + Send + Sync + 'static {
    type Config;

    fn timestamp(&self) -> Result<Timestamp> {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|dur| dur.as_millis())
            .context("Failed to get current timestamp")
    }

    async fn new(config: Self::Config) -> Result<Self>;
    async fn push(&self, job: PushJobDto) -> Result<JobId>;
    async fn pop(&self) -> Result<Vec<Job>>;
    async fn list(&self) -> Result<Vec<Job>>;
}
