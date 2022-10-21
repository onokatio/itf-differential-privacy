all: linpack.wasm linpack.out

linpack.wasm: linpack_bench.cpp
	../c/wasi-sdk-16.0/bin/clang++ ./linpack_bench.cpp -O3 -lm -Wno-all -D_WASI_EMULATED_PROCESS_CLOCKS -lwasi-emulated-process-clocks -o linpack.wasm

linpack.out: linpack_bench.cpp
	g++ -O3 ./linpack_bench.cpp -lm -o linpack.out
