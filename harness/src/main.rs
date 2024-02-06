mod config;
mod exec;
mod instantiation;

use exec::spawn_epoch_thread;
use futures::executor::block_on;
use instantiation::TaskManager;
use std::env;
use std::path::Path;
use std::time::Instant;

// TODO: smart choosing of engine number and num sandboxes?

fn bench_mpk_pooling(
    path: &Path,
    tasks_per_store: usize,
    mpk: bool,
    num_stores: usize,
    is_async: bool,
) {
    let start = Instant::now();

    let mgr = TaskManager::build(path, num_stores, mpk, is_async);
    let post_instantiation = Instant::now();

    if is_async {
        let epoch_thread = spawn_epoch_thread(mgr.engine.clone(), 10); // awaken every 10 microseconds
        block_on(mgr.do_task_n_async(tasks_per_store)).unwrap();
        epoch_thread.send(()).unwrap(); // kill epoch thread
    } else {
        mgr.do_task_n_sync(tasks_per_store).unwrap();
    }
    let end = Instant::now();

    println!("Setup time: {:?}", post_instantiation - start);
    println!("Execution time: {:?}", end - post_instantiation);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if !args.len() == 5 {
        println!("Usage: cargo run <path> <num tasks> <num stores> <mpk or no> <async or no>");
        std::process::exit(1);
    }
    // ../benches/instantiation/toobig.wat
    let p = &args[1];
    // 8
    let tasks_per_store = (args[2]).parse::<usize>().unwrap();
    let path = Path::new(p);
    let num_stores = (args[3]).parse::<usize>().unwrap();
    let mpk = &args[4] == "mpk";
    let is_async = &args[5] == "async";

    println!(
        "mpk_pooling: invoking {:?} across {num_stores} with {tasks_per_store} tasks per store\n mpk: {mpk}, is_async: {is_async}", path
    );

    bench_mpk_pooling(path, tasks_per_store, mpk, num_stores, is_async);
    println!("Done!");
}
