pub mod start;

use clap::Subcommand;

use self::start::StartOpt;

#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    /// Spawns a Mate Environment
    Start(StartOpt),
}
