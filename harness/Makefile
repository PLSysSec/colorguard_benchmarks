.PHONY: regex_no_mpk load_balancing_no_mpk templating_no_mpk regex_mpk load_balancing_mpk templating_mpk build run_all

build:
	cd ../examples/regex_example && cargo build --release --target=wasm32-wasi
	cd ../examples/load_balancing && cargo build --release --target=wasm32-wasi
	cd ../examples/template_price && cargo build --release --target=wasm32-wasi

regex_no_mpk: build
	bash run_no_mpk_parallel.sh ../examples/regex_example/target/wasm32-wasi/release/regex_example.wasm
	mv out regex_no_mpk

load_balancing_no_mpk: build
	bash run_no_mpk_parallel.sh ../examples/load_balancing/target/wasm32-wasi/release/load_balancing.wasm
	mv out load_balancing_no_mpk

template_no_mpk: build
	bash run_no_mpk_parallel.sh ../examples/template_price/target/wasm32-wasi/release/template_price.wasm
	mv out template_no_mpk

regex_mpk: build
	bash run_mpk.sh ../examples/regex_example/target/wasm32-wasi/release/regex_example.wasm
	mv out regex_mpk

load_balancing_mpk: build
	bash run_mpk.sh ../examples/load_balancing/target/wasm32-wasi/release/load_balancing.wasm
	mv out load_balancing_mpk

template_mpk: build
	bash run_mpk.sh ../examples/template_price/target/wasm32-wasi/release/template_price.wasm
	mv out template_mpk

run_all: regex_no_mpk load_balancing_no_mpk templating_no_mpk regex_mpk load_balancing_mpk templating_mpk
