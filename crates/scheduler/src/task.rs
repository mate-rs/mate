use std::{sync::Arc, time::Duration};

use anyhow::Result;
use tokio::{sync::mpsc::Sender, time::sleep};

use crate::Scheduler;
use crate::backend::redis::RedisBackend;

pub struct SchedulerTask {
    scheduler: Arc<Scheduler<RedisBackend>>,
    main_process_tx: Sender<String>,
}

impl SchedulerTask {
    pub async fn new(
        main_process_tx: Sender<String>,
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

            // match scheduler.pop().await {
            //     Ok(jobs) => {
            //         if jobs.is_empty() {
            //             continue;
            //         }

            //         if let Err(err) = main_pipe
            //             .send(&Message::Text(format!("Jobs found: {}", jobs.len())))
            //             .await
            //         {
            //             error!(%err, "Failed to send message to fifo");
            //         }
            //     }
            //     Err(err) => {
            //         if let Err(err) = main_pipe
            //             .send(&Message::Text(format!("Failed to fetch jobs: {}", err)))
            //             .await
            //         {
            //             error!(%err, "Failed to send message to fifo");
            //         }
            //     }
            // }
        }
    }
}
