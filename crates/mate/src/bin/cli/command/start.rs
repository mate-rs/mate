use std::{path::PathBuf, process::Stdio};

use anyhow::Result;
use clap::Args;
use tokio::process::{Child, Command};

use mate::repl::Repl;
use mate_fifo::NPipe;
use tracing::info;

const MATE_SCHEDULER_BIN: &str = "./target/debug/mate-scheduler";

#[derive(Clone, Debug, Args)]
pub struct StartOpt {
    /// URL to connect to Redis
    #[clap(long, default_value = "redis://127.0.0.1:6379/")]
    redis_url: String,
    /// Scheduler threshold seconds
    #[clap(long, default_value = "1")]
    threshold: u64,
    /// Do not spawn child procesess for scheduler or executor
    #[clap(long, default_value = "false")]
    standalone: bool,
}

impl StartOpt {
    pub async fn exec(&self) -> Result<()> {
        let main_pipe = NPipe::new("main")?;
        let main_pipe_handler = main_pipe.open().await?;
        let scheduler_pipe = NPipe::new("scheduler")?;
        let schuduler_pipe_handler = scheduler_pipe.open().await?;

        if !self.standalone {
            info!("Spawning scheduler...");
            let _scheduler_task = spawn_scheduler(main_pipe.path(), scheduler_pipe.path())?;
        }

        let repl = Repl::new(main_pipe_handler, schuduler_pipe_handler);
        repl.start().await?;
        Ok(())
    }
}

fn spawn_scheduler(main_pipe: &PathBuf, scheduler_pipe: &PathBuf) -> Result<Child> {
    let child = Command::new(PathBuf::from(MATE_SCHEDULER_BIN))
        .arg("--main-pipe")
        .arg(main_pipe.to_str().unwrap())
        .arg("--scheduler-pipe")
        .arg(scheduler_pipe.to_str().unwrap())
        .arg("--redis-url")
        .arg(String::from("redis://127.0.0.1:6379/"))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    Ok(child)
}
