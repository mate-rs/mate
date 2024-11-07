use serde::{Deserialize, Serialize};

use mate_proto::{Job, PushJobDto};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Text(String),
    MainReply(MainReply),
    SchedulerRequest(SchedulerRequest),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MainReply {
    JobCreated(String),
    JobPopped(Vec<Job>),
    ListJobs(Vec<Job>),
    Error(String),
    SchedulerExited,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SchedulerRequest {
    PushJob(PushJobDto),
    PopJob,
    ListJobs,
    Exit,
}
