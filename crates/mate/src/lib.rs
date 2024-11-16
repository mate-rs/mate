pub mod client;
pub mod job;
pub mod repl;

use anyhow::Result;

pub struct Mate {}

impl Mate {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
}
