use std::time::Duration;

use anyhow::Result;
use mate_proto::Job;
use tokio::{sync::mpsc::Sender, time::sleep};

use crate::Executor;

const DEMO_WAT: &str = r#"
        (module
            (import "host" "host_func" (func $host_hello (param i32)))

            (func (export "hello")
                i32.const 3
                call $host_hello)
        )
    "#;

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
            let executor = Executor::new();
            match executor
                .execute(Job {
                    id: "1234".to_string(),
                    wat: DEMO_WAT.to_string(),
                })
                .await
            {
                Ok(_) => {
                    self.main_process_tx.send("done".to_string()).await.unwrap();
                }
                Err(e) => {
                    self.main_process_tx
                        .send(format!("error: {}", e))
                        .await
                        .unwrap();
                }
            }
        }
    }
}
