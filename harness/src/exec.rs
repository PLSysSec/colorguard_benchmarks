use crate::config::WasiHostCtx;
use anyhow::{anyhow, Result};
use futures::future::join_all;
use futures::prelude::*;
use std::path::Path;
use std::sync::mpsc::{self, Sender, TryRecvError};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::time::sleep;
use wasmtime::*;

use crate::config::*;

pub const EPOCH_T_IN_MILLIS: u64 = 1;
// pub const STORES_PER_ENGINE: usize = 1000;

pub fn spawn_epoch_thread(engine: Engine, num_millisec: u64) -> Sender<()> {
    let epoch_t = Duration::from_millis(num_millisec);
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        loop {
            // poll for termination flag
            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    break;
                }
                Err(TryRecvError::Empty) => {}
            };

            // increment epoch
            engine.increment_epoch();
            // go back to sleep
            thread::sleep(epoch_t);
        }
    });
    tx
}

pub struct TaskManager {
    pub engine: Engine,
    pub pre: Arc<InstancePre<WasiHostCtx>>,
}

impl TaskManager {
    pub fn new(path: &Path, mpk: bool) -> Self {
        let engine = get_engine(mpk);
        let pre = get_preinstance(engine.clone(), path);
        Self { engine, pre }
    }

    /// execute loaded module once
    async fn do_task(&self) -> Result<()> {
        let mut store = get_store(&self.engine);
        let instance = self.pre.instantiate_async(&mut store).await?;
        let f = instance.get_typed_func::<(), ()>(&mut store, "_start")?;
        f.call_async(&mut store, ()).await
    }

    pub async fn do_task_n(&self, num_tasks: usize, delay_interval: u64) -> Result<()> {
        let mut handles = Vec::new();

        for _ in 0..num_tasks {
            let handle = self.do_task();
            handles.push(handle);
        }
        // execute tasks with up to STORES_PER_ENGINE concurrent tasks
        /*
                let stream = futures::stream::iter(handles).buffer_unordered(num_tasks);
                let results = stream
                    .then(|x| async {
                        sleep(Duration::from_nanos(delay_interval)).await;
                        x
                    })
                    .collect::<Vec<_>>()
                    .await;
        */
        let results = join_all(handles).await;

        for r in results {
            if r.is_err() {
                return Err(anyhow!("async task failed: {:?}", r));
            }
        }

        Ok(())
    }
}

pub fn exec_all(mgr: &TaskManager, num_tasks: usize, delay: u64) {
    let epoch_thread = spawn_epoch_thread(mgr.engine.clone(), EPOCH_T_IN_MILLIS);
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(mgr.do_task_n(num_tasks, delay)).unwrap();
    epoch_thread.send(()).unwrap(); // kill epoch thread
}
