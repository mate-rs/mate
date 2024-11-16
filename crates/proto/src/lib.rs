use serde::{Deserialize, Serialize};

pub type JobId = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub id: JobId,
    pub task: String,
}

/// Represent a `Task` that can be executed by the `Executor`
/// when referred by a `Job`.
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub wasm: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PushJobDto {
    pub task: String,
}
