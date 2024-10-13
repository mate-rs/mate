use std::process;
use std::sync::Arc;

use anyhow::Result;
use tracing::info;

use crate::client::{Client, Command};

pub struct Repl {
    pub(crate) client: Arc<Client>,
}

impl Repl {
    pub fn new(client: Arc<Client>) -> Self {
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
                    // "eq" => {
                    //     let job = Job {
                    //         data: args[1].to_string(),
                    //     };

                    //     match client.enqueue(job).await {
                    //         Ok(_) => {
                    //             println!("Job enqueued with success!");
                    //         }
                    //         Err(err) => {
                    //             eprintln!("Failed to enqueue job: {:?}", err);
                    //         }
                    //     }
                    // }
                    "list" => match client.send(Command::List).await {
                        Ok(_) => {
                            info!("List went OK.");
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
