use anyhow::{anyhow, Result};
use futures::prelude::*;
use std::path::Path;
use std::sync::mpsc::{self, Sender, TryRecvError};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::time::sleep;
use wasmtime::*;
// use wasmtime_wasi::WasiCtx;
// use tokio::time::Sleep;
use crate::config::WasiHostCtx;

use crate::config::*;

pub const EPOCH_T_IN_MILLIS: u64 = 1;
pub const STORES_PER_ENGINE: usize = 1000;

pub fn spawn_epoch_thread(engine: Engine, num_millisec: u64) -> Sender<()> {
    let epoch_t = Duration::from_millis(num_millisec);
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        loop {
            // poll for termination flag
            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    //println!("Terminating epoch thread");
                    break;
                }
                Err(TryRecvError::Empty) => {}
            };

            // increment epoch
            engine.increment_epoch();
            // for engine in engines.iter() {
            //     engine.increment_epoch();
            // }
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

    // pub fn build_n(path: &Path, num_engines: usize, mpk: bool) -> Vec<Self> {
    //     let mut mgrs = Vec::new();
    //     for _ in 0..num_engines {
    //         let mgr = Self::build(path, mpk);
    //         mgrs.push(mgr);
    //     }
    //     mgrs
    // }

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
            // sleep(Duration::from_micros(delay_interval)).await;
            // handles.insert(handle, idx * delay_interval);
        }
        // execute tasks with up to STORES_PER_ENGINE concurrent tasks
        // let stream =
        let stream = futures::stream::iter(handles).buffer_unordered(STORES_PER_ENGINE);
        let results = stream
            .then(|x| async {
                sleep(Duration::from_nanos(delay_interval)).await;
                x
            })
            .collect::<Vec<_>>()
            .await;

        if results.iter().any(|r| r.is_err()) {
            Err(anyhow!("async task failed"))
        } else {
            Ok(())
        }
    }
}

// TODO: add timed arrival queue here
// enginers per epoch?
// all engines exec all their assigned tasks
// pub async fn exec_all_async(
//     mgrs: &[TaskManager],
//     tasks_per_engine: usize,
//     delay: u64,
// ) -> Result<()> {
//     let handles = mgrs
//         .iter()
//         .map(|mgr| mgr.do_task_n(tasks_per_engine, delay));

//     let results = futures::future::join_all(handles).await;
//     if results.iter().any(|r| r.is_err()) {
//         Err(anyhow!("async task failed"))
//     } else {
//         Ok(())
//     }
// }

pub fn exec_all(mgr: &TaskManager, num_tasks: usize, delay: u64) {
    // let engines = mgrs.iter().map(|mgr| mgr.engine.clone()).collect();
    let epoch_thread = spawn_epoch_thread(mgr.engine.clone(), EPOCH_T_IN_MILLIS);
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(mgr.do_task_n(num_tasks, delay)).unwrap();
    epoch_thread.send(()).unwrap(); // kill epoch thread
}
