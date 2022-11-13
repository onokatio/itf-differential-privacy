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
cd data
./parquet_to_csv.py ???.parquet 
```

will output ???.csv and ???.sqlite

# Build

```
cargo build --target wasm32-wasi
```

# Run

* at this point there are two issues
1. the runtime cannot pass arguments to the program, so I must hardcode the data file name
2. opening the csv file fails
```
tau@xps13:runtime$ ./target/debug/runtime ../samples/tlc_trip_record/tlc_trip_record/target/wasm32-wasi/debug/tlc_trip_record.wasm
error running example: No such file or directory (os error 44)
Error: RuntimeError: WASI exited with code: 1

Caused by:
    WASI exited with code: 1
```


# Processing sqlite files

* I thought processing sqlite with a query specified at command line is more convenient
* ./parquet_to_csv.py already produces an sqlite file
* The code that processes sqlite is already in main.rs but it is currently commented out, as targetting --wasm32-wasi fails when it tries to compile a C program included in sqlite3 module, complaining the lack of stdio.h

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
