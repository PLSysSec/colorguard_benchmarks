mod config;
mod exec;

use exec::*;
use std::env;
use std::path::Path;
use std::time::Instant;

fn bench_mpk_pooling(path: &Path, num_engines: usize, tasks_per_engine: usize, mpk: bool) {
    let start = Instant::now();

    let mgrs = TaskManager::build_n(path, num_engines, mpk);
    let post_instantiation = Instant::now();

    exec_all(&mgrs, tasks_per_engine);

    let end = Instant::now();

    println!("Setup time: {:?}", post_instantiation - start);
    println!("Execution time: {:?}", end - post_instantiation);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        println!(
            "Usage: cargo run <path> <num_engine> <tasks per store> <mpk or no>"
        );
        std::process::exit(1);
    }
    let path = Path::new(&args[1]);

    let num_engines = (args[2]).parse::<usize>().unwrap();
    let tasks_per_engine = (args[3]).parse::<usize>().unwrap();
    let mpk = &args[4] == "mpk";

    println!(
        "mpk_pooling: invoking {:?} across {num_engines} with {tasks_per_engine} tasks per engine\nmpk: {mpk}", path
    );

    bench_mpk_pooling(path, num_engines, tasks_per_engine, mpk);
    println!("Done!");
}
