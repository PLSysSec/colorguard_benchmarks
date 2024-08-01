mkdir -p out

rm out/no_mpk_parallel_1.txt
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_1.txt

rm out/no_mpk_parallel_2.txt
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_2.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_2.txt

rm out/no_mpk_parallel_3.txt
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_3.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_3.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_3.txt 

rm out/no_mpk_parallel_4.txt
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_4.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_4.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_4.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_4.txt

rm out/no_mpk_parallel_5.txt
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_5.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_5.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_5.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_5.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_5.txt 

rm out/no_mpk_parallel_6.txt
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_6.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_6.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_6.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_6.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_6.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_6.txt

rm out/no_mpk_parallel_7.txt
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_7.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_7.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_7.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_7.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_7.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_7.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_7.txt 

rm out/no_mpk_parallel_8.txt
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_8.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_8.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_8.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_8.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_8.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_8.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_8.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_8.txt

rm out/no_mpk_parallel_9.txt
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_9.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_9.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_9.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_9.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_9.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_9.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_9.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_9.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_9.txt 

rm out/no_mpk_parallel_10.txt
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_10.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_10.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_10.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_10.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_10.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_10.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_10.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_10.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_10.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_10.txt

rm out/no_mpk_parallel_11.txt
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_11.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_11.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_11.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_11.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_11.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_11.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_11.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_11.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_11.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_11.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_11.txt 


rm out/no_mpk_parallel_12.txt
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_12.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_12.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_12.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_12.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_12.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_12.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_12.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_12.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_12.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_12.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_12.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_12.txt


rm out/no_mpk_parallel_13.txt
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_13.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_13.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_13.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_13.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_13.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_13.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_13.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_13.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_13.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_13.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_13.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_13.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_13.txt


rm out/no_mpk_parallel_14.txt
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_14.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_14.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_14.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_14.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_14.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_14.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_14.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_14.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_14.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_14.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_14.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_14.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_14.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_14.txt


rm out/no_mpk_parallel_15.txt
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_15.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_15.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_15.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_15.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_15.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_15.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_15.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_15.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_15.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_15.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_15.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_15.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_15.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_15.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_15.txt





