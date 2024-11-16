mod ipc;

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use clap::Parser;

use mate_executor::{storage::Storage, task::ExecutorTask};

use self::ipc::IpcServer;

#[derive(Debug, Parser)]
#[command(
    name = "mate-executor",
    author = "Esteban Borai <estebanborai@gmail.com> (https://github.com/mate-rs/mate)",
    next_line_help = true
)]
pub struct MateExecutorCli {
    #[clap(long)]
    pub main_pipe: PathBuf,
    #[clap(long)]
    pub executor_pipe: PathBuf,
}

impl MateExecutorCli {
    pub async fn exec(self) -> Result<()> {
        let storage = Storage::new().await?;
        let (ipc_server, main_rx) =
            IpcServer::new(Arc::clone(&storage), &self.main_pipe, &self.executor_pipe).await?;
        let mut executor_task = ExecutorTask::new(Arc::clone(&storage), main_rx).await?;

        tokio::select! {
            _ = ipc_server.listen() => {},
            _ = executor_task.run() => {},
        }

        Ok(())
    }
}
