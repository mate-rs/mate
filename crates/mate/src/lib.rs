pub mod client;
pub mod executor;
pub mod job;
pub mod repl;
pub mod scheduler;

use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use repl::Repl;

use self::client::Client;
use self::executor::Executor;
use self::scheduler::{Scheduler, SchedulerBackend};

pub struct Mate<SB: SchedulerBackend> {
    pub client: Arc<Client<SB>>,
    pub repl: Repl<SB>,
}

impl<SB: SchedulerBackend> Mate<SB> {
    pub fn new(scheduler: Scheduler<SB>, threshold: Duration) -> Self {
        let scheduler = Arc::new(scheduler);
        let executor = Arc::new(Executor::new(Arc::clone(&scheduler), threshold));
        let client = Arc::new(Client::new(executor, scheduler));
        let repl = Repl::new(Arc::clone(&client));

        Self { client, repl }
    }

    pub async fn repl(&self) -> Result<()> {
        self.repl.start().await
    }

    pub async fn run(&self) -> Result<()> {
        let executor = Arc::clone(&self.client.executor);
        tokio::spawn(async move {
            if let Err(err) = executor.run().await {
                eprintln!("Executor error: {:?}", err);
            }
        })
        .await?;

        Ok(())
    }
}
