mod ipc;

use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use clap::Parser;

use mate_scheduler::{Scheduler, SchedulerBackend};
use mate_scheduler::backend::redis::RedisBackend;
use mate_scheduler::task::SchedulerTask;

use self::ipc::IpcServer;

#[derive(Debug, Parser)]
#[command(
    name = "mate-scheduler",
    author = "Esteban Borai <estebanborai@gmail.com> (https://github.com/mate-rs/mate)",
    next_line_help = true
)]
pub struct MateSchedulerCli {
    #[clap(long)]
    pub main_pipe: PathBuf,
    #[clap(long)]
    pub scheduler_pipe: PathBuf,
    #[clap(long)]
    pub redis_url: String,
}

impl MateSchedulerCli {
    pub async fn exec(self) -> Result<()> {
        let (ipc_server, main_tx) = IpcServer::new(&self.main_pipe, &self.scheduler_pipe).await?;
        let backend = RedisBackend::new(self.redis_url.to_owned()).await?;
        let scheduler = Arc::new(Scheduler::new(backend));
        let scheduler_task = SchedulerTask::new(main_tx, Arc::clone(&scheduler)).await?;

        tokio::select! {
            _ = ipc_server.listen(Arc::clone(&scheduler)) => {},
            _ = scheduler_task.run() => {},
        }

        Ok(())
    }
}
