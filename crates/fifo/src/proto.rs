use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Text(String),
    Ack,
    MainReply(MainReply),
    SchedulerRequest(SchedulerRequest),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MainReply {
    ListJobs(Vec<Job>),
    SchedulerExited,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SchedulerRequest {
    ListJobs,
    Exit,
}
