use std::process;
use std::sync::Arc;

use anyhow::Result;
use tracing::info;

use crate::client::{Command as MateCommand, SocketClient};

pub struct Repl {
    sc: Arc<SocketClient>,
}

impl Repl {
    pub fn new(sc: SocketClient) -> Self {
        Self { sc: Arc::new(sc) }
    }

    pub async fn start(&self) -> Result<()> {
        println!("mate‎ v0.0.0. Run 'help' to see available commands, 'exit' to quit");

        loop {
            eprint!("𐲖 ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let args = input.split_whitespace().collect::<Vec<&str>>();

            match args[0].trim() {
                "list" => match self.sc.send(MateCommand::List).await {
                    Ok(_) => {
                        info!("List went OK.");
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
