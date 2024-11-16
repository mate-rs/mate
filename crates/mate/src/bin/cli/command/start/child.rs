use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Stdio;

use anyhow::{Context, Result};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

pub fn spawn(binary_path: &PathBuf, args: HashMap<String, String>) -> Result<()> {
    let mut cmd = Command::new(binary_path);
    let process_name = binary_path
        .file_name()
        .context("Failed to retrieve file name for process")?
        .to_str()
        .context("Failed to retrieve utf-8 string for file name")?;

    let mut logs = File::create_new(format!("{}_log.log", process_name))?;
    let mut errs = File::create_new(format!("{}_err.log", process_name))?;

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

            if let Err(err) = logs.write_all(line.as_bytes()) {
                eprintln!("Failed to write to log file. {err}");
            }
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

            if let Err(err) = errs.write_all(line.as_bytes()) {
                eprintln!("Failed to write to log file. {err}");
            }
        }
    });

    Ok(())
}
