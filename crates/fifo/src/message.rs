use serde::{Deserialize, Serialize};

use mate_proto::Job;

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Text(String),
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
