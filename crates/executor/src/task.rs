use std::time::Duration;

use anyhow::Result;
use tokio::{sync::mpsc::Sender, time::sleep};

pub struct ExecutorTask {
    main_process_tx: Sender<String>,
}

impl ExecutorTask {
    pub async fn new(main_process_tx: Sender<String>) -> Result<Self> {
        Ok(Self { main_process_tx })
    }

    pub async fn run(&self) {
        loop {
            sleep(Duration::from_secs(1)).await;
        }
    }
}
