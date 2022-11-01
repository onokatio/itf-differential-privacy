# cpp linpack

## Compile

for apple silicon, use `arch -x86_64` before compiler command.

### for WASM

```
$ path/to/wasi-sdk-16.0/bin/clang++ ./linpack_bench.cpp -O3 -lm -Wno-all -D_WASI_EMULATED_PROCESS_CLOCKS -lwasi-emulated-process-clocks -o linpack.wasm
```

### for native

```
$ g++ -O3 ./linpack_bench.cpp -lm -o linpack.out
```
