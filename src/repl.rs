use std::process;
use std::sync::Arc;

use anyhow::Result;

use crate::client::Client;
use crate::job::Job;
use crate::scheduler::SchedulerBackend;

pub struct Repl<SB: SchedulerBackend> {
    pub(crate) client: Arc<Client<SB>>,
}

impl<SB: SchedulerBackend> Repl<SB> {
    pub fn new(client: Arc<Client<SB>>) -> Self {
        Self { client }
    }

    pub async fn start(&self) -> Result<()> {
        let client = Arc::clone(&self.client);

        tokio::spawn(async move {
            println!("mate‎ v0.0.0. Run 'help' to see available commands, 'exit' to quit");

            loop {
                eprint!("𐲖 ");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let args = input.split_whitespace().collect::<Vec<&str>>();

                match args[0].trim() {
                    "eq" => {
                        let job = Job {
                            data: args[1].to_string(),
                        };

                        match client.enqueue(job).await {
                            Ok(_) => {
                                println!("Job enqueued with success!");
                            }
                            Err(err) => {
                                eprintln!("Failed to enqueue job: {:?}", err);
                            }
                        }
                    }
                    "list" => match client.list().await {
                        Ok(jobs) => {
                            if jobs.is_empty() {
                                println!("No jobs found");
                                continue;
                            }

                            for job in jobs {
                                println!("{:#?}", job);
                            }
                        }
                        Err(err) => {
                            eprintln!("Failed to list jobs: {:?}", err);
                        }
                    },
                    "exit" => {
                        process::exit(0);
                    }
                    _ => println!(
                        "Unknown command: \"{}\", use \"help\" to learn more about commands",
                        input.trim()
                    ),
                }
            }
        })
        .await?;

        Ok(())
    }
}
