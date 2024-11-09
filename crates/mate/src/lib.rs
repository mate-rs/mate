pub mod client;
pub mod job;
pub mod repl;

use anyhow::Result;

pub const EXECUTOR_PORT: u16 = 62_833;
pub const SCHEDULER_PORT: u16 = 62_837;

pub struct Mate {}

impl Mate {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
}
