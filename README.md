# Build harness

Maybe necessary:
    `cd wasmtime && cargo build --rlease && cd -`

`cd harness && cargo build --release && cd -`

# Run benchmarks

`cd harness && make run_all`

# Known issue

* Harness requires rust 1.76.0 while examples are build for 1.73.0 (like the remaining segue/colorguard repo). May need to change defaults in between.