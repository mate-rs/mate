pub mod storage;
pub mod task;

use anyhow::Result;
use tracing::info;
use wasmtime::{Caller, Engine, Linker, Module, Store};

use mate_proto::{Job, Task};

pub struct Executor {}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

impl Executor {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn execute(&self, job: &Job, task: Task) -> Result<()> {
        let engine = Engine::default();
        let module = Module::new(&engine, task.wasm)?;
        let mut linker = Linker::new(&engine);

        linker.func_wrap(
            "host",
            "host_func",
            |caller: Caller<'_, u32>, param: i32| {
                info!("Got {} from WebAssembly", param);
                info!("my host state is: {}", caller.data());
            },
        )?;

        let mut store = Store::new(&engine, 4);
        let instance = linker.instantiate(&mut store, &module)?;
        let hello = instance.get_typed_func::<(), ()>(&mut store, "hello")?;

        hello.call(&mut store, ())?;

        Ok(())
    }
}
