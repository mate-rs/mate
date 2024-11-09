use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Stdio;

use anyhow::Result;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

pub fn spawn(binary_path: &PathBuf, args: HashMap<String, String>) -> Result<()> {
    let mut cmd = Command::new(binary_path);

    for (flag, value) in args.iter() {
        cmd.arg(flag);
        cmd.arg(value);
    }

    let mut child = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;

    let stdout = child.stdout.take().expect("Expected a Stdout Handle");
    let stderr = child.stderr.take().expect("Expected a Stderr Handle");
    let mut stdout_reader = BufReader::new(stdout);
    let mut stderr_reader = BufReader::new(stderr);

    tokio::spawn(async move {
        let status = child
            .wait()
            .await
            .expect("child process encountered an error");

        println!("child status was: {}", status);
    });

    tokio::spawn(async move {
        let mut line = String::new();
        loop {
            line.clear();
            let n = stdout_reader.read_line(&mut line).await.unwrap();
            if n == 0 {
                break;
            }
            println!("scheduler: {}", line);
        }
    });

    tokio::spawn(async move {
        let mut line = String::new();
        loop {
            line.clear();
            let n = stderr_reader.read_line(&mut line).await.unwrap();
            if n == 0 {
                break;
            }
            println!("scheduler: {}", line);
        }
    });

    Ok(())
}
