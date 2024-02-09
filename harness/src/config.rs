use std::path::Path;
use std::sync::Arc;
use wasmtime::*;
use wasmtime_wasi::preview2;
use wasmtime_wasi::preview2::WasiCtxBuilder;

pub struct WasiHostCtx {
    preview2_ctx: preview2::WasiCtx,
    preview2_table: wasmtime::component::ResourceTable,
    preview1_adapter: preview2::preview1::WasiPreview1Adapter,
}

impl preview2::WasiView for WasiHostCtx {
    fn table(&mut self) -> &mut wasmtime::component::ResourceTable {
        &mut self.preview2_table
    }

    fn ctx(&mut self) -> &mut preview2::WasiCtx {
        &mut self.preview2_ctx
    }
}

impl preview2::preview1::WasiPreview1View for WasiHostCtx {
    fn adapter(&self) -> &preview2::preview1::WasiPreview1Adapter {
        &self.preview1_adapter
    }

    fn adapter_mut(&mut self) -> &mut preview2::preview1::WasiPreview1Adapter {
        &mut self.preview1_adapter
    }
}

fn get_config(mpk: bool) -> Config {
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
    config.async_support(true);
    config.epoch_interruption(true);
    config.allocation_strategy(strategy.clone());
    config
}

pub fn get_engine(mpk: bool) -> Engine {
    let config = get_config(mpk);
    Engine::new(&config).expect("failed to create engine")
}

/// set timeslice to 1 epoch
pub fn get_store(engine: &Engine) -> Store<WasiHostCtx> {
    let wasi = WasiCtxBuilder::new().inherit_stdout().build();
    // wasi.inherit_stdout();
    let host_ctx = WasiHostCtx {
        preview2_ctx: wasi,
        preview2_table: preview2::ResourceTable::new(),
        preview1_adapter: preview2::preview1::WasiPreview1Adapter::new(),
    };
    let mut store = Store::new(engine, host_ctx);
    store.set_epoch_deadline(1);
    store.epoch_deadline_async_yield_and_update(1);
    store
}

pub fn get_preinstance(engine: Engine, path: &Path) -> Arc<InstancePre<WasiHostCtx>> {
    let mut linker: Linker<WasiHostCtx> = Linker::new(&engine);
    preview2::preview1::add_to_linker_async(&mut linker).unwrap();
    // wasmtime_wasi::preview2::preview1::add_to_linker_sync(&mut linker, |cx| cx).unwrap(); // async
    let module = Module::from_file(&engine, path).expect("failed to load WASI example module");

    Arc::new(
        linker
            .instantiate_pre(&module)
            .expect("failed to pre-instantiate"),
    )
}
