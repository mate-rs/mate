use anyhow::Result;
use async_trait::async_trait;
use redis::Cmd;

use mate_proto::{Job, JobId, PushJobDto};
use tracing::info;

use crate::{SchedulerBackend, SCHEDULER_JOB_PREFIX};

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

    async fn push(&self, job: PushJobDto) -> Result<JobId> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;

        let incr_cmd = Cmd::incr(JOB_COUNTER_KEY, 1);
        let count: i32 = incr_cmd.query_async(&mut conn).await?;
        let job_id = format!("{SCHEDULER_JOB_PREFIX}:{count}");
        let job_payload = serde_json::to_string(&Job {
            id: job_id.clone(),
            task: job.task,
        })?;

        let set_cmd = Cmd::set(&job_id, job_payload);
        set_cmd.exec_async(&mut conn).await?;

        // FIXME: review downcast data loss
        // https://en.wikipedia.org/wiki/Arithmetic_underflow
        let zadd_cmd = Cmd::zadd(SCHEDULER_JOB_PREFIX, &job_id, self.timestamp()? as u64);
        zadd_cmd.exec_async(&mut conn).await?;

        Ok(job_id)
    }

    async fn pop(&self) -> Result<Vec<Job>> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;

        let zrange_cmd = Cmd::zrange(SCHEDULER_JOB_PREFIX, 0, 1);
        let job_keys: Vec<String> = zrange_cmd.query_async(&mut conn).await?;

        info!(?job_keys, "Found keys for jobs that need to be executed");

        let mget_cmd = Cmd::mget(&job_keys);
        let job_data: Vec<Job> = mget_cmd
            .query_async::<Vec<String>>(&mut conn)
            .await?
            .into_iter()
            .filter_map(|data| match serde_json::from_str::<Job>(&data) {
                Ok(job) => Some(job),
                Err(err) => {
                    info!(?err, "Failed to deserialize job data");
                    None
                }
            })
            .collect::<Vec<Job>>();

        info!(?job_data, "Found data for jobs that need to be executed");

        Ok(job_data)
    }

    async fn list(&self) -> Result<Vec<Job>> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;

        let zrange_cmd = Cmd::zrange(SCHEDULER_JOB_PREFIX, 0, -1);
        let job_keys: Vec<String> = zrange_cmd.query_async(&mut conn).await?;

        info!(?job_keys, "Found keys for jobs that need to be executed");

        let mget_cmd = Cmd::mget(&job_keys);
        let job_data: Vec<Job> = mget_cmd
            .query_async::<Vec<String>>(&mut conn)
            .await?
            .into_iter()
            .filter_map(|data| match serde_json::from_str::<Job>(&data) {
                Ok(job) => Some(job),
                Err(err) => {
                    info!(?err, "Failed to deserialize job data");
                    None
                }
            })
            .collect::<Vec<Job>>();

        info!(?job_data, "Found data for jobs that need to be executed");

        Ok(job_data)
    }
}
