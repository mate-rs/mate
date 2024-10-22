use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

use anyhow::Result;
use libc::{c_char, mkfifo};
use tempfile::{tempdir, TempDir};

const RW_FS_PERMISSIONS: u16 = 0o644;

/// Named Pipe (FIFO) abstraction for IPC
pub struct NPipe {
    /// Path to the FIFO file
    path: PathBuf,
    /// Temporary Dir containing FIFO files, will be deleted
    /// when [`Fifo`] is dropped
    dir: TempDir,
}

impl NPipe {
    pub fn new<S: Into<String>>(name: S) -> Result<Self> {
        let dir = tempdir()?;
        let path = dir.path().join(name.into());

        unsafe {
            Self::make_fifo(path.clone())?;
        };

        Ok(Self { path, dir })
    }

    #[inline]
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

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
