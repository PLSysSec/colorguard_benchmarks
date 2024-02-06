use anyhow::{anyhow, Result};
use std::path::Path;
use std::sync::Arc;
use wasmtime::*;
use wasmtime_wasi::{sync::WasiCtxBuilder, WasiCtx};

use crate::config::get_engine;

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

/// set timeslice to 1 epoch
fn store(engine: &Engine, is_async: bool) -> Store<WasiCtx> {
    let wasi = WasiCtxBuilder::new().build();
    let mut store = Store::new(engine, wasi);
    if is_async {
        store.set_epoch_deadline(1);
        store.epoch_deadline_async_yield_and_update(1);
    }
    store
}

// TODO: 1 engine with n stores each with m instances???

fn get_preinstance(engine: Engine, path: &Path) -> Arc<InstancePre<WasiCtx>> {
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |cx| cx).unwrap();
    let module = Module::from_file(&engine, path).expect("failed to load WASI example module");

    Arc::new(
        linker
            .instantiate_pre(&module)
            .expect("failed to pre-instantiate"),
    )
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
