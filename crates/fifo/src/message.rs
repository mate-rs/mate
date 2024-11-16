use serde::{Deserialize, Serialize};

use mate_proto::{Job, PushJobDto, Task};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Text(String),
    MainReply(MainReply),
    SchedulerRequest(SchedulerRequest),
    ExecutorRequest(ExecutorRequest),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MainReply {
    JobCreated(String),
    JobPopped(Vec<Job>),
    ListJobs(Vec<Job>),
    TaskCreated(String),
    TasksList(Vec<Task>),
    Error(String),
    SchedulerExited,
    ExecutorExited,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SchedulerRequest {
    PushJob(PushJobDto),
    PopJob,
    ListJobs,
    Exit,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ExecutorRequest {
    ListTasks,
    CreateTask(Task),
    ExecuteJobs(Vec<Job>),
    Exit,
}
