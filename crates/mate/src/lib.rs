pub mod client;
pub mod executor;
pub mod job;
pub mod repl;
pub mod scheduler;

use std::sync::Arc;

use anyhow::Result;
use repl::Repl;

use self::client::Client;
use self::scheduler::SchedulerBackend;

pub const EXECUTOR_PORT: u16 = 62_833;
pub const SCHEDULER_PORT: u16 = 62_837;

pub struct Mate {
    pub client: Arc<Client>,
    pub repl: Repl,
}

impl Mate {
    pub fn new() -> Result<Self> {
        let client = Arc::new(Client::new()?);
        let repl = Repl::new(Arc::clone(&client));

        Ok(Self { client, repl })
    }

    pub async fn repl(&self) -> Result<()> {
        self.repl.start().await
    }
}
