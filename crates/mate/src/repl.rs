use std::process;
use std::sync::Arc;

use anyhow::Result;

use mate_fifo::message::{MainReply, Message, SchedulerRequest};
use mate_fifo::NPipeHandle;

pub struct Repl {
    main_pipe: Arc<NPipeHandle>,
    scheduler_pipe: Arc<NPipeHandle>,
}

impl Repl {
    pub fn new(main_pipe: NPipeHandle, scheduler_pipe: NPipeHandle) -> Self {
        Self {
            main_pipe: Arc::new(main_pipe),
            scheduler_pipe: Arc::new(scheduler_pipe),
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
                "list" => match self
                    .scheduler_pipe
                    .send(&Message::SchedulerRequest(SchedulerRequest::ListJobs))
                    .await
                {
                    Ok(_) => {
                        let msg = self.main_pipe.recv().await?;

                        if let Message::MainReply(MainReply::ListJobs(jobs)) = msg {
                            println!(">> {jobs:?}");
                        } else {
                            eprintln!("Failed to list jobs: {:?}", msg);
                        }
                    }
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
                                process::exit(0);
                            } else {
                                eprintln!("Failed to exit scheduler: {:?}", msg);
                            }
                        }
                        Err(err) => {
                            eprintln!("Failed to list jobs: {}", err);
                        }
                    }
                }
                _ => println!(
                    "Unknown command: \"{}\", use \"help\" to learn more about commands",
                    input.trim()
                ),
            }
        }
    }
}
