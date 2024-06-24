mkdir -p ./out

rm out/mpk_1.txt
taskset -c 2 target/release/harness ../examples/load_balancing/target/wasm32-wasi/release/load_balancing.wasm  14000 1000 mpk | tee -a ./out/mpk_1.txt
rm out/mpk_2.txt
taskset -c 2 target/release/harness ../examples/load_balancing/target/wasm32-wasi/release/load_balancing.wasm  28000 1000 mpk | tee -a ./out/mpk_2.txt
rm out/mpk_3.txt
taskset -c 2 target/release/harness ../examples/load_balancing/target/wasm32-wasi/release/load_balancing.wasm  42000 1000 mpk | tee -a ./out/mpk_3.txt
rm out/mpk_4.txt
taskset -c 2 target/release/harness ../examples/load_balancing/target/wasm32-wasi/release/load_balancing.wasm  56000 1000 mpk | tee -a ./out/mpk_4.txt
rm out/mpk_5.txt
taskset -c 2 target/release/harness ../examples/load_balancing/target/wasm32-wasi/release/load_balancing.wasm  70000 1000 mpk | tee -a ./out/mpk_5.txt
rm out/mpk_6.txt
taskset -c 2 target/release/harness ../examples/load_balancing/target/wasm32-wasi/release/load_balancing.wasm  84000 1000 mpk | tee -a ./out/mpk_6.txt
rm out/mpk_7.txt
taskset -c 2 target/release/harness ../examples/load_balancing/target/wasm32-wasi/release/load_balancing.wasm  98000 1000 mpk | tee -a ./out/mpk_7.txt
rm out/mpk_8.txt
taskset -c 2 target/release/harness ../examples/load_balancing/target/wasm32-wasi/release/load_balancing.wasm  112000 1000 mpk | tee -a ./out/mpk_8.txt
rm out/mpk_9.txt
taskset -c 2 target/release/harness ../examples/load_balancing/target/wasm32-wasi/release/load_balancing.wasm  126000 1000 mpk | tee -a ./out/mpk_9.txt
rm out/mpk_10.txt
taskset -c 2 target/release/harness ../examples/load_balancing/target/wasm32-wasi/release/load_balancing.wasm  140000 1000 mpk | tee -a ./out/mpk_10.txt
rm out/mpk_11.txt
taskset -c 2 target/release/harness ../examples/load_balancing/target/wasm32-wasi/release/load_balancing.wasm  154000 1000 mpk | tee -a ./out/mpk_11.txt
rm out/mpk_12.txt
taskset -c 2 target/release/harness ../examples/load_balancing/target/wasm32-wasi/release/load_balancing.wasm  168000 1000 mpk | tee -a ./out/mpk_12.txt
rm out/mpk_13.txt
taskset -c 2 target/release/harness ../examples/load_balancing/target/wasm32-wasi/release/load_balancing.wasm  182000 1000 mpk | tee -a ./out/mpk_13.txt
rm out/mpk_14.txt
taskset -c 2 target/release/harness ../examples/load_balancing/target/wasm32-wasi/release/load_balancing.wasm  196000 1000 mpk | tee -a ./out/mpk_14.txt
rm out/mpk_15.txt
taskset -c 2 target/release/harness ../examples/load_balancing/target/wasm32-wasi/release/load_balancing.wasm  210000 1000 mpk | tee -a ./out/mpk_15.txt

