use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{bail, Result};
use tokio::sync::Mutex;

use mate_proto::Task;

pub type SharedStorage = Arc<Storage>;

pub struct Storage {
    inner: Mutex<HashMap<String, Vec<u8>>>,
}

impl Storage {
    pub async fn new() -> Result<SharedStorage> {
        let storage = Self {
            inner: Mutex::new(HashMap::new()),
        };

        Ok(Arc::new(storage))
    }

    pub async fn create_task(&self, task: &Task) -> Result<()> {
        let mut inner = self.inner.lock().await;
        inner.insert(task.name.clone(), task.wasm.clone());
        Ok(())
    }

    pub async fn list_tasks(&self) -> Result<Vec<Task>> {
        let inner = self.inner.lock().await;
        let tasks = inner
            .iter()
            .map(|(name, wasm)| Task {
                name: name.clone(),
                wasm: wasm.clone(),
            })
            .collect();

        Ok(tasks)
    }

    pub async fn get_task(&self, name: &str) -> Result<Task> {
        let inner = self.inner.lock().await;

        if let Some(task) = inner.get(name) {
            return Ok(Task {
                name: name.to_string(),
                wasm: task.clone(),
            });
        }

        bail!("Task not found")
    }
}
