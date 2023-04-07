#!/bin/zsh

#../runtime/target/release/runtime ../samples/dp.rs/target/wasm32-wasi/release/samples.wasm
#

for i in {1..100};do
time ../runtime/target/release/runtime ../samples/tlc_trip_record/tlc_trip_record_orig/target/wasm32-wasi/release/tlc_trip_record_orig.wasm < ../samples/tlc_trip_record/data/yellow_tripdata_2022-10.csv > log/log_sandbox_$i.txt
done

#for i in {1..100};do
#time ../samples/tlc_trip_record/tlc_trip_record_orig/target/release/tlc_trip_record_orig < ../samples/tlc_trip_record/data/yellow_tripdata_2022-10.csv > log/log_raw_$i.txt
#done

#for i in {1..100};do
#time ../runtime/target/release/runtime ../samples/tlc_trip_record/tlc_trip_record/target/wasm32-wasi/release/tlc_trip_record.wasm < ../samples/tlc_trip_record/data/yellow_tripdata_2022-10.csv > log/log_$i.txt
#done

#for i in {1..100};do
#time ../runtime/target/release/runtime_nonoise ../samples/tlc_trip_record/tlc_trip_record/target/wasm32-wasi/release/tlc_trip_record.wasm < ../samples/tlc_trip_record/data/yellow_tripdata_2022-10.csv > log/log_nonoise_$i.txt
#done
