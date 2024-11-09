use std::process;
use std::sync::Arc;

use anyhow::Result;

use mate_fifo::message::{ExecutorRequest, MainReply, Message, SchedulerRequest};
use mate_fifo::NPipeHandle;
use mate_proto::PushJobDto;

pub struct Repl {
    main_pipe: Arc<NPipeHandle>,
    scheduler_pipe: Arc<NPipeHandle>,
    executor_pipe: Arc<NPipeHandle>,
}

impl Repl {
    pub fn new(
        main_pipe: NPipeHandle,
        scheduler_pipe: NPipeHandle,
        executor_pipe: NPipeHandle,
    ) -> Self {
        Self {
            main_pipe: Arc::new(main_pipe),
            scheduler_pipe: Arc::new(scheduler_pipe),
            executor_pipe: Arc::new(executor_pipe),
        }
    }

    pub async fn start(&self) -> Result<()> {
        println!("mate‎ v0.0.0. Run 'help' to see available commands, 'exit' to quit");

        loop {
            eprint!("𐲖 ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let args = input.split_whitespace().collect::<Vec<&str>>();

            match args[0].trim() {
                "push" => match self
                    .scheduler_pipe
                    .send(&Message::SchedulerRequest(SchedulerRequest::PushJob(
                        PushJobDto {
                            data: String::from("data"),
                        },
                    )))
                    .await
                {
                    Ok(_) => match self.main_pipe.recv().await? {
                        Message::MainReply(MainReply::JobCreated(job_id)) => {
                            println!(">> Created job with ID: {}", job_id);
                        }
                        Message::MainReply(MainReply::Error(msg)) => {
                            eprintln!("Failed to create job: {:?}", msg);
                        }
                        _ => {}
                    },
                    Err(err) => {
                        eprintln!("Failed to push job: {}", err);
                    }
                },
                "pop" => match self
                    .scheduler_pipe
                    .send(&Message::SchedulerRequest(SchedulerRequest::PopJob))
                    .await
                {
                    Ok(_) => match self.main_pipe.recv().await? {
                        Message::MainReply(MainReply::JobPopped(jobs)) => {
                            println!(">> {:#?}", jobs);
                        }
                        Message::MainReply(MainReply::Error(msg)) => {
                            eprintln!("Failed to pop job: {:?}", msg);
                        }
                        _ => {}
                    },
                    Err(err) => {
                        eprintln!("Failed to create job: {}", err);
                    }
                },
                "list" => match self
                    .scheduler_pipe
                    .send(&Message::SchedulerRequest(SchedulerRequest::ListJobs))
                    .await
                {
                    Ok(_) => match self.main_pipe.recv().await? {
                        Message::MainReply(MainReply::ListJobs(jobs)) => {
                            println!(">> {jobs:?}");
                        }
                        Message::MainReply(MainReply::Error(msg)) => {
                            eprintln!("Failed to list jobs: {:?}", msg);
                        }
                        _ => {}
                    },
                    Err(err) => {
                        eprintln!("Failed to list jobs: {}", err);
                    }
                },
                "exit" => {
                    match self
                        .scheduler_pipe
                        .send(&Message::SchedulerRequest(SchedulerRequest::Exit))
                        .await
                    {
                        Ok(_) => {
                            let msg = self.main_pipe.recv().await?;

                            if let Message::MainReply(MainReply::SchedulerExited) = msg {
                                println!("Scheduler exited");
                            } else {
                                eprintln!("Failed to exit scheduler: {:?}", msg);
                            }
                        }
                        Err(err) => {
                            eprintln!("Failed to list jobs: {}", err);
                        }
                    }

                    match self
                        .executor_pipe
                        .send(&Message::ExecutorRequest(ExecutorRequest::Exit))
                        .await
                    {
                        Ok(_) => {
                            let msg = self.main_pipe.recv().await?;

                            if let Message::MainReply(MainReply::ExecutorExited) = msg {
                                println!("Executor exited")
                            } else {
                                eprintln!("Failed to exit executor: {:?}", msg);
                            }
                        }
                        Err(err) => {
                            eprintln!("Failed to list jobs: {}", err);
                        }
                    }

                    process::exit(0);
                }
                _ => println!(
                    "Unknown command: \"{}\", use \"help\" to learn more about commands",
                    input.trim()
                ),
            }
        }
    }
}
