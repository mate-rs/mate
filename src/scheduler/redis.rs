use anyhow::Result;
use async_trait::async_trait;
use redis::Cmd;

use crate::job::Job;

use super::{SchedulerBackend, SCHEDULER_JOB_PREFIX};

pub const JOB_COUNTER_KEY: &str = "mate:job:counter";

pub struct RedisBackend {
    client: redis::Client,
}

#[async_trait]
impl SchedulerBackend for RedisBackend {
    type Config = String;

    async fn new(config: Self::Config) -> anyhow::Result<Self> {
        let client = redis::Client::open(config)?;
        Ok(Self { client })
    }

    async fn push(&self, job: Job) -> Result<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;

        let incr_cmd = Cmd::incr(JOB_COUNTER_KEY, 1);
        let count: i32 = incr_cmd.query_async(&mut conn).await?;
        let job_key = format!("{SCHEDULER_JOB_PREFIX}:{count}");

        let set_cmd = Cmd::set(&job_key, job.data);
        set_cmd.exec_async(&mut conn).await?;

        // FIXME: review downcast data loss
        // https://en.wikipedia.org/wiki/Arithmetic_underflow
        let zadd_cmd = Cmd::zadd(SCHEDULER_JOB_PREFIX, job_key, self.timestamp()? as u64);
        zadd_cmd.exec_async(&mut conn).await?;

        Ok(())
    }

    async fn pop(&self) -> Result<Vec<Job>> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;

        let zrange_cmd = Cmd::zrange(SCHEDULER_JOB_PREFIX, 0, 1);
        let job_data: Vec<String> = zrange_cmd.query_async(&mut conn).await?;
        let jobs = job_data
            .iter()
            .map(|data| Job { data: data.clone() })
            .collect();

        Ok(jobs)
    }

    async fn list(&self) -> Result<Vec<Job>> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;

        let zrange_cmd = Cmd::zrange(SCHEDULER_JOB_PREFIX, 0, -1);
        let job_data: Vec<String> = zrange_cmd.query_async(&mut conn).await?;
        let jobs = job_data
            .iter()
            .map(|data| Job { data: data.clone() })
            .collect();

        Ok(jobs)
    }
}
