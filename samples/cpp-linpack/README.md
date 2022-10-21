# cpp linpack

## Compile

### WASM

```
$ path/to/wasi-sdk-16.0/bin/clang++ ./linpack_bench.cpp -O3 -lm -Wno-all -D_WASI_EMULATED_PROCESS_CLOCKS -lwasi-emulated-process-clocks -o linpack.wasm
```

for apple silicon
```
$ arch -x86_64 path/to/wasi-sdk-16.0/bin/clang++ -O3 ./linpack_bench.cpp -lm -Wno-all -D_WASI_EMULATED_PROCESS_CLOCKS -lwasi-emulated-process-clocks -o linpack.wasm
```

### native

```
$ g++ -O3 ./linpack_bench.cpp -lm -o linpack.out
```

for apple silicon
```
$ arch -x86_64 g++ -O3 ./linpack_bench.cpp -lm -o linpack.out
```