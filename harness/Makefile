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

shielding_off:
	sudo cset shield --reset

disable_hyperthreading:
	sudo bash -c "echo off > /sys/devices/system/cpu/smt/control"

restore_hyperthreading:
	sudo bash -c "echo on > /sys/devices/system/cpu/smt/control"

disable_cpufreq:
	sudo cpufreq-set -c 2 -g performance
	sudo cpufreq-set -c 2 --min 2200MHz --max 2200MHz

restore_cpufreq:
	POLICYINFO=($$(cpufreq-info -c 0 -p)) && \
	sudo cpufreq-set -c 2 -g $${POLICYINFO[2]} && \
	sudo cpufreq-set -c 2 --min $${POLICYINFO[0]}MHz --max $${POLICYINFO[1]}MHz

benchmark_env_setup: disable_hyperthreading disable_cpufreq
	sudo cset shield -c 2 -k on
	(taskset -c 2 echo "testing shielded shell..." > /dev/null 2>&1 && echo "Shielded shell running!") || (echo "Not running in shielded shell. Run make shielded_shell first!" && sudo cset shield --reset && exit 1)

benchmark_env_close: restore_hyperthreading restore_cpufreq

