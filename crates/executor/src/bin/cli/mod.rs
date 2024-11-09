mod ipc;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

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
        let (ipc_server, main_tx) = IpcServer::new(&self.main_pipe, &self.executor_pipe).await?;

        tokio::select! {
            _ = ipc_server.listen() => {},
        }

        Ok(())
    }
}
