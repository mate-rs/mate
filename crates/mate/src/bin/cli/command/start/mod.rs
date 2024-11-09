mod child;

use std::{collections::HashMap, path::PathBuf};

use anyhow::Result;
use clap::Args;

use mate::repl::Repl;
use mate_fifo::NPipe;

use self::child::spawn;

const MATE_EXECUTOR_BIN: &str = "./target/debug/mate-executor";
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
        let executor_pipe = NPipe::new("executor")?;
        let executor_pipe_handler = executor_pipe.open().await?;

        if !self.standalone {
            let main_pipe = main_pipe.path().to_str().unwrap().to_string();
            let scheduler_pipe = scheduler_pipe.path().to_str().unwrap().to_string();
            let executor_pipe = executor_pipe.path().to_str().unwrap().to_string();

            let mut scheduler_args: HashMap<String, String> = HashMap::new();

            scheduler_args.insert("--main-pipe".to_string(), main_pipe.to_owned());
            scheduler_args.insert("--scheduler-pipe".to_string(), scheduler_pipe.to_owned());
            scheduler_args.insert("--executor-pipe".to_string(), executor_pipe.to_owned());
            scheduler_args.insert("--redis-url".to_string(), self.redis_url.to_string());

            spawn(&PathBuf::from(MATE_SCHEDULER_BIN), scheduler_args)?;

            let mut executor_args: HashMap<String, String> = HashMap::new();

            executor_args.insert("--main-pipe".to_string(), main_pipe);
            executor_args.insert("--executor-pipe".to_string(), executor_pipe);

            spawn(&PathBuf::from(MATE_EXECUTOR_BIN), executor_args)?;
        }

        let repl = Repl::new(
            main_pipe_handler,
            schuduler_pipe_handler,
            executor_pipe_handler,
        );
        repl.start().await?;
        Ok(())
    }
}
