mkdir -p out

rm -f out/no_mpk_parallel_1.txt
dstat -C 2 --sys --cpu-adv -ipc --lock --vm --noheaders >  out/no_mpk_parallel_1_dstat.csv &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_1.txt
pkill dstat

rm -f out/no_mpk_parallel_2.txt
dstat -C 2 --sys --cpu-adv -ipc --lock --vm --noheaders >  out/no_mpk_parallel_2_dstat.csv &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_2.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_2.txt
pkill dstat

rm -f out/no_mpk_parallel_3.txt
dstat -C 2 --sys --cpu-adv -ipc --lock --vm --noheaders >  out/no_mpk_parallel_3_dstat.csv &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_3.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_3.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_3.txt
pkill dstat

rm -f out/no_mpk_parallel_4.txt
dstat -C 2 --sys --cpu-adv -ipc --lock --vm --noheaders >  out/no_mpk_parallel_4_dstat.csv &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_4.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_4.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_4.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_4.txt
pkill dstat

rm -f out/no_mpk_parallel_5.txt
dstat -C 2 --sys --cpu-adv -ipc --lock --vm --noheaders >  out/no_mpk_parallel_5_dstat.csv &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_5.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_5.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_5.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_5.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_5.txt
pkill dstat

rm -f out/no_mpk_parallel_6.txt
dstat -C 2 --sys --cpu-adv -ipc --lock --vm --noheaders >  out/no_mpk_parallel_6_dstat.csv &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_6.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_6.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_6.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_6.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_6.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_6.txt
pkill dstat

rm -f out/no_mpk_parallel_7.txt
dstat -C 2 --sys --cpu-adv -ipc --lock --vm --noheaders >  out/no_mpk_parallel_7_dstat.csv &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_7.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_7.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_7.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_7.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_7.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_7.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_7.txt
pkill dstat

rm -f out/no_mpk_parallel_8.txt
dstat -C 2 --sys --cpu-adv -ipc --lock --vm --noheaders >  out/no_mpk_parallel_8_dstat.csv &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_8.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_8.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_8.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_8.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_8.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_8.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_8.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_8.txt
pkill dstat

rm -f out/no_mpk_parallel_9.txt
dstat -C 2 --sys --cpu-adv -ipc --lock --vm --noheaders >  out/no_mpk_parallel_9_dstat.csv &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_9.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_9.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_9.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_9.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_9.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_9.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_9.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_9.txt &
taskset -c 2 target/release/harness $1  14000 1000 no | tee -a ./out/no_mpk_parallel_9.txt
pkill dstat

rm -f out/no_mpk_parallel_10.txt
dstat -C 2 --sys --cpu-adv -ipc --lock --vm --noheaders >  out/no_mpk_parallel_10_dstat.csv &
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
pkill dstat

rm -f out/no_mpk_parallel_11.txt
dstat -C 2 --sys --cpu-adv -ipc --lock --vm --noheaders >  out/no_mpk_parallel_11_dstat.csv &
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
pkill dstat


rm -f out/no_mpk_parallel_12.txt
dstat -C 2 --sys --cpu-adv -ipc --lock --vm --noheaders >  out/no_mpk_parallel_12_dstat.csv &
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
pkill dstat


rm -f out/no_mpk_parallel_13.txt
dstat -C 2 --sys --cpu-adv -ipc --lock --vm --noheaders >  out/no_mpk_parallel_13_dstat.csv &
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
pkill dstat


rm -f out/no_mpk_parallel_14.txt
dstat -C 2 --sys --cpu-adv -ipc --lock --vm --noheaders >  out/no_mpk_parallel_14_dstat.csv &
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
pkill dstat


rm -f out/no_mpk_parallel_15.txt
dstat -C 2 --sys --cpu-adv -ipc --lock --vm --noheaders >  out/no_mpk_parallel_15_dstat.csv &
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
pkill dstat





