use anyhow::{anyhow, Result};
use std::path::Path;
use std::sync::mpsc::{self, Sender, TryRecvError};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use wasmtime::*;
use wasmtime_wasi::WasiCtx;

use crate::config::*;

pub async fn do_tasks_async(
    pre: &InstancePre<WasiCtx>,
    mut s: Store<WasiCtx>,
    num_tasks: usize,
) -> Result<()> {
    for _ in 0..num_tasks {
        let instance = pre.instantiate_async(&mut s).await?;
        instance
            .get_typed_func::<(), ()>(&mut s, "_start")?
            .call_async(&mut s, ())
            .await?;
    }
    Ok(())
}

pub fn spawn_epoch_thread(engine: Engine, num_usec: u64) -> Sender<()> {
    let epoch_t = Duration::from_micros(num_usec);
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        loop {
            // poll for termination flag
            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    println!("Terminating epoch thread");
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
    pub is_async: bool,
    pub mpk: bool,
    pub engine: Engine,
    pub stores: Vec<Store<WasiCtx>>,
    pub pre: Arc<InstancePre<WasiCtx>>,
}

impl TaskManager {
    pub fn build(path: &Path, num_stores: usize, mpk: bool, is_async: bool) -> Self {
        let engine = get_engine(mpk, is_async);
        let pre = get_preinstance(engine.clone(), path);
        let stores = get_stores(engine.clone(), num_stores, is_async);
        Self {
            is_async,
            mpk,
            engine,
            pre,
            stores,
        }
    }

    pub async fn do_task_n_async(self, tasks_per_store: usize) -> Result<()> {
        let tasks = self
            .stores
            .into_iter()
            .map(|s| do_tasks_async(&self.pre, s, tasks_per_store));
        let results = futures::future::join_all(tasks).await;
        if results.iter().any(|r| r.is_err()) {
            Ok(())
        } else {
            Err(anyhow!("async task failed"))
        }
    }

    pub fn do_task_n_sync(mut self, tasks_per_store: usize) -> Result<()> {
        let num_tasks = tasks_per_store * self.stores.len();
        for _ in 0..num_tasks {
            let instance = self.pre.instantiate(&mut self.stores[0])?;
            let f = instance.get_typed_func::<(), ()>(&mut self.stores[0], "_start")?;
            f.call(&mut self.stores[0], ())?;
        }
        Ok(())
    }
}

pub fn get_stores(engine: Engine, num_stores: usize, is_async: bool) -> Vec<Store<WasiCtx>> {
    let mut stores = Vec::new();

    for _ in 0..num_stores {
        let store = store(&engine, is_async);
        stores.push(store);
    }
    stores
}
