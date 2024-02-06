use wasmtime::*;

fn get_config(mpk: bool, is_async: bool) -> Config {
    let mut pool = PoolingAllocationConfig::default();
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
