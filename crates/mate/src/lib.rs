pub mod client;
pub mod executor;
pub mod ipc;
pub mod job;
pub mod repl;
pub mod scheduler;

use anyhow::Result;

pub const EXECUTOR_PORT: u16 = 62_833;
pub const SCHEDULER_PORT: u16 = 62_837;

pub struct Mate {}

impl Mate {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
}
