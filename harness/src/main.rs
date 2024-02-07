mod config;
mod exec;

use exec::*;
use std::env;
use std::path::Path;
use std::time::Instant;

// TODO: smart choosing of engine number and num sandboxes?

fn bench_mpk_pooling(
    path: &Path,
    num_engines: usize,
    tasks_per_store: usize,
    mpk: bool,
    stores_per_engine: usize,
    is_async: bool,
) {
    let start = Instant::now();

    let mgrs = TaskManager::build_n(path, num_engines, stores_per_engine, mpk, is_async);
    let post_instantiation = Instant::now();

    exec_all(mgrs, tasks_per_store, is_async);

    // if is_async {
    //     let epoch_thread = spawn_epoch_thread(mgr.engine.clone(), 10); // awaken every 10 microseconds
    //     block_on(mgr.do_task_n_async(tasks_per_store)).unwrap();
    //     epoch_thread.send(()).unwrap(); // kill epoch thread
    // } else {
    //     mgr.do_task_n_sync(tasks_per_store).unwrap();
    // }
    let end = Instant::now();

    println!("Setup time: {:?}", post_instantiation - start);
    println!("Execution time: {:?}", end - post_instantiation);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if !args.len() == 5 {
        println!("Usage: cargo run <path> <engine> <stores per engine> <tasks per store> <mpk or no> <async or no>");
        std::process::exit(1);
    }
    // ../benches/instantiation/toobig.wat
    let p = &args[1];
    // 8
    let path = Path::new(p);

    let num_engines = (args[2]).parse::<usize>().unwrap();
    let stores_per_engine = (args[3]).parse::<usize>().unwrap();
    let tasks_per_store = (args[4]).parse::<usize>().unwrap();

    let mpk = &args[5] == "mpk";
    let is_async = &args[6] == "async";

    println!(
        "mpk_pooling: invoking {:?} across {num_engines} with {stores_per_engine} stores per engine, with {tasks_per_store} tasks per store\nmpk: {mpk}, is_async: {is_async}", path
    );

    bench_mpk_pooling(
        path,
        num_engines,
        tasks_per_store,
        mpk,
        stores_per_engine,
        is_async,
    );
    println!("Done!");
}
