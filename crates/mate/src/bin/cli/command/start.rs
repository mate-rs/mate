use std::{path::PathBuf, process::Stdio};

use anyhow::Result;
use clap::Args;

use mate::{client::SocketClient, repl::Repl, SCHEDULER_PORT};
use mate_fifo::NPipe;

const MATE_SCHEDULER_BIN: &str = "./target/debug/mate-scheduler";

#[derive(Clone, Debug, Args)]
pub struct StartOpt {
    /// URL to connect to Redis
    #[clap(long, default_value = "redis://127.0.0.1:6379/")]
    redis_url: String,
    /// Scheduler threshold seconds
    #[clap(long, default_value = "1")]
    threshold: u64,
}

impl StartOpt {
    pub async fn exec(&self) -> Result<()> {
        let main_pipe = NPipe::new("main")?;
        let main_pipe_handler = main_pipe.open().await?;
        let scheduler_pipe = NPipe::new("scheduler")?;
        let schuduler_pipe_handler = scheduler_pipe.open().await?;
        // let _scheduler_task = tokio::spawn(async move {
        //     tokio::process::Command::new(PathBuf::from(MATE_SCHEDULER_BIN))
        //         .arg("--fifo")
        //         .arg(scheduler_pipe.path().to_str().unwrap())
        //         // .stdout(Stdio::null())
        //         // .stderr(Stdio::null())
        //         .spawn()
        //         .expect("Failed to spawn process for mate-scheduler");
        // });

        let repl = Repl::new(main_pipe_handler, schuduler_pipe_handler);

        repl.start().await?;
        Ok(())
    }
}
