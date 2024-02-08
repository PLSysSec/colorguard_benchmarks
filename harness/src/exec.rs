use anyhow::{anyhow, Result};
use futures::executor::block_on;
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

pub fn spawn_epoch_thread(engines: Vec<Engine>, num_millisec: u64) -> Sender<()> {
    let epoch_t = Duration::from_millis(num_millisec);
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
            for engine in engines.iter() {
                engine.increment_epoch();
            }
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
    pub fn build(path: &Path, stores_per_engine: usize, mpk: bool, is_async: bool) -> Self {
        let engine = get_engine(mpk, is_async);
        let pre = get_preinstance(engine.clone(), path);
        let stores = get_stores(engine.clone(), stores_per_engine, is_async);
        Self {
            is_async,
            mpk,
            engine,
            pre,
            stores,
        }
    }

    pub fn build_n(
        path: &Path,
        num_engines: usize,
        stores_per_engine: usize,
        mpk: bool,
        is_async: bool,
    ) -> Vec<Self> {
        let mut mgrs = Vec::new();
        for _ in 0..num_engines {
            let mgr = Self::build(path, stores_per_engine, mpk, is_async);
            mgrs.push(mgr);
        }
        mgrs
    }

    pub async fn do_task_n_async(self, tasks_per_store: usize) -> Result<()> {
        let tasks = self
            .stores
            .into_iter()
            .map(|s| do_tasks_async(&self.pre, s, tasks_per_store));
        let results = futures::future::join_all(tasks).await;
        if results.iter().any(|r| { r.as_ref().unwrap(); r.is_err() }) {
            Err(anyhow!("async task failed"))
        } else {
            Ok(())
        }
    }

    pub fn do_task_n_sync(mut self, tasks_per_store: usize) -> Result<()> {
        let num_tasks = tasks_per_store * self.stores.len();
        for idx in 0..num_tasks {
            let s_idx = idx % self.stores.len();
            let instance = self.pre.instantiate(&mut self.stores[s_idx])?;
            let f = instance.get_typed_func::<(), ()>(&mut self.stores[s_idx], "_start")?;
            f.call(&mut self.stores[s_idx], ())?;
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

pub async fn exec_all_async(mgrs: Vec<TaskManager>, tasks_per_store: usize) -> Result<()> {
    let mut tasks = Vec::new();
    for mgr in mgrs.into_iter() {
        let task = mgr.do_task_n_async(tasks_per_store);
        tasks.push(task);
    }
    let results = futures::future::join_all(tasks).await;
    if results.iter().any(|r| { r.is_err() }) {
        Err(anyhow!("async task failed"))
    } else {
        Ok(())
    }
}

pub fn exec_all_sync(mgrs: Vec<TaskManager>, tasks_per_store: usize) -> Result<()> {
    for mgr in mgrs.into_iter() {
        mgr.do_task_n_sync(tasks_per_store)?;
    }
    Ok(())
}

pub fn exec_all(mgrs: Vec<TaskManager>, tasks_per_store: usize, is_async: bool) {
    if is_async {
        let engines = mgrs.iter().map(|mgr| mgr.engine.clone()).collect();
        let epoch_thread = spawn_epoch_thread(engines, 10); // awaken every 10 microseconds
        block_on(exec_all_async(mgrs, tasks_per_store)).unwrap();
        epoch_thread.send(()).unwrap(); // kill epoch thread
    } else {
        exec_all_sync(mgrs, tasks_per_store).unwrap();
        // mgr.do_task_n_sync(tasks_per_store).unwrap();
    }
}
