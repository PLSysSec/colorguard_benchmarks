use std::sync::mpsc::{self, Sender, TryRecvError};
use std::thread;
use std::time::Duration;
use wasmtime::*;

pub fn spawn_epoch_thread(engine: Engine, num_usec: u64) -> Sender<()> {
    let epoch_t = Duration::from_micros(num_usec);
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        // match self.rx
        loop {
            // poll for termination flag
            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    println!("Terminating epoch thread");
                    break;
                }
                Err(TryRecvError::Empty) => {}
            };

            // increment epoch
            engine.increment_epoch();
            // go back to sleep
            thread::sleep(epoch_t);
        }
    });
    tx
}
