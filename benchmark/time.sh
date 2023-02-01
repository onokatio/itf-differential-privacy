#!/bin/bash

#../runtime/target/release/runtime ../samples/dp.rs/target/wasm32-wasi/release/samples.wasm

for i in {1..10};do
time ../runtime/target/release/runtime ../samples/tlc_trip_record/tlc_trip_record/target/wasm32-wasi/release/tlc_trip_record.wasm < ../samples/tlc_trip_record/data/yellow_tripdata_2022-10.csv > log_$i.txt 2>log_$i_time.txt
done
