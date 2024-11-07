use serde::{Deserialize, Serialize};

pub type JobId = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub id: JobId,
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PushJobDto {
    pub data: String,
}
