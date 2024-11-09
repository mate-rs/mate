mod child;

use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use mate::repl::Repl;
use mate_fifo::NPipe;

use self::child::spawn;

const MATE_SCHEDULER_BIN: &str = "./target/debug/mate-scheduler";

#[derive(Clone, Debug, Args)]
pub struct StartOpt {
    /// URL to connect to Redis
    #[clap(long, default_value = "redis://127.0.0.1:6379/")]
    redis_url: String,
    /// Scheduler threshold seconds
    #[clap(long, default_value = "1")]
    threshold: u64,
    /// Do not spawn child procesess for scheduler and executor
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
            spawn(
                &PathBuf::from(MATE_SCHEDULER_BIN),
                main_pipe.path(),
                scheduler_pipe.path(),
                &self.redis_url,
            )?;
        }

        let repl = Repl::new(main_pipe_handler, schuduler_pipe_handler);
        repl.start().await?;
        Ok(())
    }
}
