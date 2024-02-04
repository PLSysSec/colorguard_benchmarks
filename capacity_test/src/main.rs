use std::env;
use std::path::Path;
use std::sync::Arc;
use wasmtime::*;
use wasmtime_wasi::{sync::WasiCtxBuilder, WasiCtx};

fn store(engine: &Engine) -> Store<WasiCtx> {
    let wasi = WasiCtxBuilder::new().build();
    Store::new(engine, wasi)
}

fn get_config(num_instances: usize, mpk: bool) -> Config {
    let mut pool = PoolingAllocationConfig::default();
    let enabled = if mpk {
        MpkEnabled::Enable
    } else {
        MpkEnabled::Disable
    };
    pool.total_core_instances(num_instances as u32);
    pool.memory_protection_keys(enabled);
    let strategy = InstanceAllocationStrategy::Pooling(pool);
    // let slot_size = if mpk { 1 << 29 } else { 1 << 32 };

    let mut config = Config::default();
    config.allocation_strategy(strategy.clone());
    // if mpk {
    //     config.static_memory_guard_size(0);
    //     config.guard_before_linear_memory(false);
    // } else {
    //     config.static_memory_guard_size(1 << 31);
    //     config.guard_before_linear_memory(true);
    // }
    // config.static_memory_maximum_size(slot_size);
    // config.static_memory_forced(true);
    config
}

fn get_instances(
    config: Config,
    path: &Path,
    num_engines: usize,
    instances_per_engine: usize,
) -> Vec<(Instance, Store<WasiCtx>)> {
    let mut instances = Vec::new();
    for idx in 0..num_engines {
        println!("count: {:?}", (idx + 1) * instances_per_engine);
        let engine = Engine::new(&config).expect("failed to create engine");
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |cx| cx).unwrap();
        let module = Module::from_file(&engine, path).expect("failed to load WASI example module");

        let pre = Arc::new(
            linker
                .instantiate_pre(&module)
                .expect("failed to pre-instantiate"),
        );
        for _ in 0..instances_per_engine {
            let mut store = store(&engine);
            let instance = pre.instantiate(&mut store).unwrap();
            instances.push((instance, store));
        }
    }
    instances
}

// fn exec(instance: Instance, mut store: Store<WasiCtx>) {
//     println!("Exec!");
//     let f = instance.get_func(&mut store, "_start").unwrap();
//     f.call(&mut store, &[], &mut []).unwrap();
// }

fn bench_mpk_pooling(path: &Path, instances_per_engine: usize, mpk: bool, num_engines: usize) {
    println!(
        "mpk_pooling: invoking {:?} across {} engines with {} instances per engine, mpk: {}",
        path, num_engines, instances_per_engine, mpk,
    );

    let config = get_config(instances_per_engine, mpk);
    let _instances = get_instances(config, path, num_engines, instances_per_engine);

    // instances
    //     .into_iter()
    //     .for_each(|(instance, s)| exec(instance, s))

    // TODO: parallel
    // TODO: join
    // TODO: smart choosing of engine number and num sandboxes?
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if !args.len() == 4 {
        println!("Usage: cargo run <path> <num_sandboxes per engine> <num engines> <mpk or no>");
        std::process::exit(1);
    }
    // ../benches/instantiation/toobig.wat
    let p = &args[1];
    // 8
    let instance_slot_count = (args[2]).parse::<usize>().unwrap();
    let path = Path::new(p);
    let num_engines = (args[3]).parse::<usize>().unwrap();
    let mpk = &args[4] == "mpk";
    // if mpk && !wasmtime::mpk::is_supported(){
    //     println!("Requested use of MPK, but MPK is not available on this machine");
    //     return;
    // }

    bench_mpk_pooling(path, instance_slot_count, mpk, num_engines);
    println!("Done!");
}
