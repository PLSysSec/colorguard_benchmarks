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

pub const STORES_PER_ENGINE: usize = 1000;

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
    pub pre: Arc<InstancePre<WasiCtx>>,
}

impl TaskManager {
    pub fn build(path: &Path, mpk: bool, is_async: bool) -> Self {
        let engine = get_engine(mpk, is_async);
        let pre = get_preinstance(engine.clone(), path);
        Self {
            is_async,
            mpk,
            engine,
            pre,
        }
    }

    pub fn build_n(path: &Path, num_engines: usize, mpk: bool, is_async: bool) -> Vec<Self> {
        let mut mgrs = Vec::new();
        for _ in 0..num_engines {
            let mgr = Self::build(path, mpk, is_async);
            mgrs.push(mgr);
        }
        mgrs
    }

    pub async fn do_task_n_async(&self, num_tasks: usize) -> Result<()> {
        let mut active_tasks = 0;
        for _ in 0..num_tasks {
            if active_tasks == STORES_PER_ENGINE {
                assert!(false); // TODO: figure out a way to wait
            }
            active_tasks += 1;
            let mut store = get_store(&self.engine, true);
            let instance = self.pre.instantiate_async(&mut store).await?;
            let f = instance.get_typed_func::<(), ()>(&mut store, "_start")?;
            f.call_async(&mut store, ()).await?;
            active_tasks -= 1;
        }
        Ok(())
    }

    // perform n tasks, generating stores for each
    pub fn do_task_n_sync(&self, num_tasks: usize) -> Result<()> {
        for _ in 0..num_tasks {
            let mut store = get_store(&self.engine, false);
            let instance = self.pre.instantiate(&mut store)?;
            let f = instance.get_typed_func::<(), ()>(&mut store, "_start")?;
            f.call(&mut store, ())?;
        }
        Ok(())
    }
}

// all engines exec all their assigned tasks
pub async fn exec_all_async(mgrs: &[TaskManager], tasks_per_engine: usize) -> Result<()> {
    let mut handles = Vec::new();
    for mgr in mgrs.iter() {
        let handle = mgr.do_task_n_async(tasks_per_engine);
        handles.push(handle);
    }

    let results = futures::future::join_all(handles).await;
    if results.iter().any(|r| r.is_err()) {
        Err(anyhow!("async task failed"))
    } else {
        Ok(())
    }
}

pub fn exec_all_sync(mgrs: &[TaskManager], tasks_per_store: usize) -> Result<()> {
    for mgr in mgrs.iter() {
        mgr.do_task_n_sync(tasks_per_store)?;
    }
    Ok(())
}

pub fn exec_all(mgrs: &[TaskManager], tasks_per_engine: usize, is_async: bool) {
    if is_async {
        let engines = mgrs.iter().map(|mgr| mgr.engine.clone()).collect();
        let epoch_thread = spawn_epoch_thread(engines, 10); // awaken every 10 microseconds
        block_on(exec_all_async(mgrs, tasks_per_engine)).unwrap();
        epoch_thread.send(()).unwrap(); // kill epoch thread
    } else {
        exec_all_sync(mgrs, tasks_per_engine).unwrap();
    }
}
