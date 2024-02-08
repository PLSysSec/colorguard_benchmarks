use std::path::Path;
use std::sync::Arc;
use wasmtime::*;
use wasmtime_wasi::{sync::WasiCtxBuilder, WasiCtx};

fn get_config(mpk: bool, is_async: bool) -> Config {
    let mut pool = PoolingAllocationConfig::default();
    // pool.total_core_instances(10_000);
    // pool.total_memories(10_000);
    // pool.total_tables(10_000);
    let enabled = if mpk {
        MpkEnabled::Enable
    } else {
        MpkEnabled::Disable
    };
    pool.memory_protection_keys(enabled);
    let strategy = InstanceAllocationStrategy::Pooling(pool);
    let mut config = Config::default();
    if is_async {
        config.async_support(true);
        config.epoch_interruption(true);
    }
    config.allocation_strategy(strategy.clone());
    config
}

pub fn get_engine(mpk: bool, is_async: bool) -> Engine {
    let config = get_config(mpk, is_async);
    Engine::new(&config).expect("failed to create engine")
}

/// set timeslice to 1 epoch
pub fn get_store(engine: &Engine, is_async: bool) -> Store<WasiCtx> {
    let wasi = WasiCtxBuilder::new().build();
    // wasi.inherit_stdout();
    let mut store = Store::new(engine, wasi);
    if is_async {
        store.set_epoch_deadline(1);
        store.epoch_deadline_async_yield_and_update(1);
    }
    store
}

pub fn get_preinstance(engine: Engine, path: &Path) -> Arc<InstancePre<WasiCtx>> {
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |cx| cx).unwrap();
    let module = Module::from_file(&engine, path).expect("failed to load WASI example module");

    Arc::new(
        linker
            .instantiate_pre(&module)
            .expect("failed to pre-instantiate"),
    )
}

// pub fn get_stores(engine: Engine, num_stores: usize, is_async: bool) -> Vec<Store<WasiCtx>> {
//     let mut stores = Vec::new();

//     for _ in 0..num_stores {
//         let store = get_store(&engine, is_async);
//         stores.push(store);
//     }
//     stores
// }
