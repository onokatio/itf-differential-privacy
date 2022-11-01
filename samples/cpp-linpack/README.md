# cpp linpack

## Requirement

```
$ wget https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-16/wasi-sdk-16.0-macos.tar.gz
$ tar xvf wasi-sdk-16.0-macos.tar.gz
```

## Compile

for apple silicon, use `arch -x86_64` before compiler command.

### WASM

```
$ path/to/wasi-sdk-16.0/bin/clang++ ./linpack_bench.cpp -O3 -lm -Wno-all -D_WASI_EMULATED_PROCESS_CLOCKS -lwasi-emulated-process-clocks -o linpack.wasm
```

### native

```
$ g++ -O3 ./linpack_bench.cpp -lm -o linpack.out
```
