pub mod proto;

use std::fmt::Display;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

use anyhow::Result;
use libc::{c_char, mkfifo};
use tempfile::{tempdir, TempDir};
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::unix::pipe::{OpenOptions, Receiver, Sender};
use tokio::sync::Mutex;
use tracing::info;

use crate::proto::Message;

const RW_FS_PERMISSIONS: u16 = 0o644;

/// Named Pipe (FIFO) abstraction for IPC
pub struct NPipe {
    /// Path to the FIFO file
    path: PathBuf,
    /// Temporary Dir containing FIFO files, will be deleted
    /// when [`Fifo`] is dropped
    _dir: TempDir,
}

impl NPipe {
    pub fn new<S: Into<String> + Clone + Display>(name: S) -> Result<Self> {
        let _dir = tempdir()?;
        let path = _dir.path().join(name.clone().into());

        unsafe {
            Self::make_fifo(path.clone())?;
        };

        info!(%name, ?path, "Created FIFO");
        Ok(Self { path, _dir })
    }

    #[inline]
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub async fn open(&self) -> Result<NPipeHandle> {
        let rx = OpenOptions::new().open_receiver(&self.path)?;
        let tx = OpenOptions::new().open_sender(&self.path)?;
        Ok(NPipeHandle {
            rx: Mutex::new(rx),
            tx: Mutex::new(tx),
        })
    }

    // FIXME: This is not working on Windows
    unsafe fn make_fifo(path: PathBuf) -> Result<()> {
        let os_str = path.into_os_string();
        let slice = os_str.as_bytes();

        // The extra space is for a null terminator (0),
        // C-Strings are null-terminated strings.
        let mut bytes = Vec::with_capacity(slice.len() + 1);
        bytes.extend_from_slice(slice);
        bytes.push(0x0);

        let path_ptr: *const c_char = (&bytes[0]) as *const u8 as *const c_char;

        if mkfifo(path_ptr, RW_FS_PERMISSIONS) != 0 {
            anyhow::bail!("Failed to create FIFO file");
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct NPipeHandle {
    rx: Mutex<Receiver>,
    tx: Mutex<Sender>,
}

impl NPipeHandle {
    pub async fn new(path: &PathBuf) -> Result<NPipeHandle> {
        let rx = OpenOptions::new().open_receiver(path)?;
        let tx = OpenOptions::new().open_sender(path)?;

        Ok(Self {
            rx: Mutex::new(rx),
            tx: Mutex::new(tx),
        })
    }

    pub async fn send(&self, msg: &Message) -> Result<()> {
        let msg = bincode::serialize(msg).expect("Serialization failed");
        let mut tx = self.tx.lock().await;

        tx.write_all(&usize::to_ne_bytes(msg.len())).await?;
        tx.write_all(&msg[..]).await?;
        tx.flush().await.map_err(|e| {
            tracing::error!("Failed to flush write pipe: {:?}", e);
            anyhow::anyhow!("The write pipe is broken")
        })
    }

    pub async fn recv(&self) -> Result<Message> {
        let mut rx = self.rx.lock().await;
        let mut len_bytes = [0u8; std::mem::size_of::<usize>()];
        rx.read_exact(&mut len_bytes).await?;
        let len = usize::from_ne_bytes(len_bytes);
        let mut buf = vec![0; len];
        rx.read_exact(&mut buf[..]).await?;
        Ok(bincode::deserialize(&buf[..]).expect("Deserialization failed"))
    }
}

#[cfg(test)]
mod tests {
    use super::NPipe;

    #[test]
    fn creates_a_named_pipe_instance() {
        let npipe_name = "my-fifo";
        let npipe = NPipe::new(npipe_name).unwrap();

        assert!(npipe.path().exists());
        assert!(npipe.path().ends_with(npipe_name));
    }

    #[test]
    fn droping_npipe_deletes_file() {
        let npipe_name = "my-fifo";
        let npipe = NPipe::new(npipe_name).unwrap();
        let path = npipe.path().clone();

        assert!(path.exists());

        drop(npipe);

        assert!(!path.exists());
    }
}
