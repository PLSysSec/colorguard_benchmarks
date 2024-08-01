mkdir -p ./out

rm out/mpk_1.txt
taskset -c 2 target/release/harness $1  14000 1000 mpk | tee -a ./out/mpk_1.txt
rm out/mpk_2.txt
taskset -c 2 target/release/harness $1  28000 1000 mpk | tee -a ./out/mpk_2.txt
rm out/mpk_3.txt
taskset -c 2 target/release/harness $1  42000 1000 mpk | tee -a ./out/mpk_3.txt
rm out/mpk_4.txt
taskset -c 2 target/release/harness $1  56000 1000 mpk | tee -a ./out/mpk_4.txt
rm out/mpk_5.txt
taskset -c 2 target/release/harness $1  70000 1000 mpk | tee -a ./out/mpk_5.txt
rm out/mpk_6.txt
taskset -c 2 target/release/harness $1  84000 1000 mpk | tee -a ./out/mpk_6.txt
rm out/mpk_7.txt
taskset -c 2 target/release/harness $1  98000 1000 mpk | tee -a ./out/mpk_7.txt
rm out/mpk_8.txt
taskset -c 2 target/release/harness $1  112000 1000 mpk | tee -a ./out/mpk_8.txt
rm out/mpk_9.txt
taskset -c 2 target/release/harness $1  126000 1000 mpk | tee -a ./out/mpk_9.txt
rm out/mpk_10.txt
taskset -c 2 target/release/harness $1  140000 1000 mpk | tee -a ./out/mpk_10.txt
rm out/mpk_11.txt
taskset -c 2 target/release/harness $1  154000 1000 mpk | tee -a ./out/mpk_11.txt
rm out/mpk_12.txt
taskset -c 2 target/release/harness $1  168000 1000 mpk | tee -a ./out/mpk_12.txt
rm out/mpk_13.txt
taskset -c 2 target/release/harness $1  182000 1000 mpk | tee -a ./out/mpk_13.txt
rm out/mpk_14.txt
taskset -c 2 target/release/harness $1  196000 1000 mpk | tee -a ./out/mpk_14.txt
rm out/mpk_15.txt
taskset -c 2 target/release/harness $1  210000 1000 mpk | tee -a ./out/mpk_15.txt

