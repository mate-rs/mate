use anyhow::Result;
use async_trait::async_trait;

use mate_proto::Job;

#[async_trait]
pub trait JobExt {
    async fn dispatch(&self) -> Result<()>;
}

#[async_trait]
impl JobExt for Job {
    async fn dispatch(&self) -> Result<()> {
        println!("Dispatching job: {:?}", self);
        Ok(())
    }
}
