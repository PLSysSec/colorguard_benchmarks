mod config;
mod exec;

use exec::*;
use std::env;
use std::path::Path;
use std::time::Instant;

// Note: very unsafe
// Returns (voluntary_context_switches, involuntatry_context_switches)
/*
fn get_context_switches() -> (i64, i64) {
    unsafe {
        let mut rusage: libc::rusage = std::mem::zeroed();
        libc::getrusage(libc::RUSAGE_SELF, &mut rusage);
        (rusage.ru_nvcsw, rusage.ru_nivcsw)
    }
}
*/

fn bench_mpk_pooling(path: &Path, num_tasks: usize, delay: u64, mpk: bool) {
    //let start = Instant::now();

    let mgr = TaskManager::new(path, mpk);
    let post_instantiation = Instant::now();
    //let (start_nvcsw,start_nivcsw) = get_context_switches();

    exec_all(&mgr, num_tasks, delay);

    let end = Instant::now();
    //let (end_nvcsw,end_nivcsw) = get_context_switches();

    //println!("{:?} voluntary context switches, {:?} involuntary context switches", end_nvcsw - start_nvcsw, end_nivcsw - start_nivcsw);

    //println!("Setup time: {:?}", post_instantiation - start);
    println!("{:?}", (end - post_instantiation).as_millis());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        println!("Usage: cargo run <path> <tasks> <arrival delay in ns> <mpk or no>");
        std::process::exit(1);
    }
    let path = Path::new(&args[1]);

    let num_tasks = (args[2]).parse::<usize>().unwrap();
    let delay = (args[3]).parse::<u64>().unwrap();
    let mpk = &args[4] == "mpk";

    // let num_engines = if mpk {
    //     200 // max engines we can fit in address space using colorguard
    // } else {
    //     14 // max engines we can fit in address space not using colorguard
    // };

    // let tasks_per_engine = num_tasks / num_engines;
    /*
    println!(
        "mpk_pooling: invoking {:?} across {num_tasks} tasks \n# of engines: {num_engines}\n arrival delay: {delay}ns\nmpk: {mpk}", path
    );
    */
    bench_mpk_pooling(path, num_tasks, delay, mpk);
    //println!("Done!");
}
