use std::process;
use std::sync::Arc;

use anyhow::Result;
use mate_fifo::proto::Message;
use tracing::info;

use mate_fifo::NPipeHandle;

use crate::client::Command as MateCommand;

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
                    .send(&Message::Text("Hello".into()))
                    .await
                {
                    Ok(_) => {
                        let message = self.main_pipe.recv().await?;
                        println!(">> {message:?}");
                    }
                    Err(err) => {
                        eprintln!("Failed to list jobs: {}", err);
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
    }
}
