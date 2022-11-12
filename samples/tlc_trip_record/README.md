# FILES

data/ --- get from this page https://www.nyc.gov/site/tlc/about/tlc-trip-record-data.page
  parquet_to_csv.py --- convert parquet to csv and sqlite3
  
# Data

There seem four slightly different kinds of data. 
We'll see which one is most suitable.

* Yellow Taxi Trip Records (PARQUET)
* Green Taxi Trip Records (PARQUET)
* For-Hire Vehicle Trip Records (PARQUET)
* High Volume For-Hire Vehicle Trip Records (PARQUET)

* https://d37ci6vzurychx.cloudfront.net/trip-data/yellow_tripdata_2022-01.parquet
* https://d37ci6vzurychx.cloudfront.net/trip-data/green_tripdata_2022-01.parquet
* https://d37ci6vzurychx.cloudfront.net/trip-data/fhv_tripdata_2022-01.parquet
* https://d37ci6vzurychx.cloudfront.net/trip-data/fhvhv_tripdata_2022-01.parquet
  
# Converting parquet to csv and sqlite

```
./parquet_to_csv.py ???.parquet 
```

will output ???.csv and ???.sqlite

# Build and run

* for now I was only able to compile it for native target (not wasm) with the usual 
```
cargo build
```
# Sqlite

* I thought processing sqlite with a query specified at command line is more convenient
* The code is already in main.rs but it is currently commented out, as targetting --wasm32-wasi fails when it tries to compile a C program included in sqlite3 module, complaining the lack of stdio.h

```
warning: source/sqlite3.c:14111:10: fatal error: 'stdio.h' file not found
warning: #include <stdio.h>
warning:          ^~~~~~~~~
warning: 1 error generated.

 ...
 
  error occurred: Command "clang" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=wasm32-wasi" "-Wall" "-Wextra" "-o" "/home/tau/proj/secfs/sandbox/wasm_wasi/itf-differential-privacy/samples/tlc_trip_record/tlc_trip_record/target/wasm32-wasi/debug/build/sqlite3-src-1fb21f0852b0b77f/out/source/sqlite3.o" "-c" "source/sqlite3.c" with args "clang" did not execute successfully (status code exit status: 1). 
```

* we can solve this issue by giving --sysroot option to clang, but I don't know how
* or the error won't happen if you are using clang-wasi
