mod command;

use anyhow::Result;
use clap::Parser;

use self::command::Command;

#[derive(Debug, Parser)]
#[command(
    name = "mate",
    about = "Mate Job Queue",
    author = "Esteban Borai <estebanborai@gmail.com> (https://github.com/mate-rs/mate)",
    next_line_help = true
)]
pub struct MateCli {
    #[clap(subcommand)]
    pub command: Command,
}

impl MateCli {
    pub async fn exec(self) -> Result<()> {
        match self.command {
            Command::Start(opt) => opt.exec().await?,
        }

        Ok(())
    }
}
