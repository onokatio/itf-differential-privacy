## Compile

for apple silicon, use `arch -x86_64` before compiler command.


### for WASM

```
$ path/to/wasi-sdk-16.0/bin/clang -O3 -o main.wasm ./main.c
```

### with out-of-box Clang

alternatively, you can compile it with ordinary clang + wasi-libc + libclang_rt.builtins-wasm32.a by

```
$ path/to/ordinary/clang --target=wasm32-unknown-wasi --sysroot <where-you-install-wasi-libc> -O3 -o main.wasm ./main.c
```

* I am following [this page](https://depth-first.com/articles/2019/10/16/compiling-c-to-webassembly-and-running-it-without-emscripten/) to do this

* install wasi-libc 
```
git clone https://github.com/WebAssembly/wasi-libc
cd wasi-libc
make install INSTALL_DIR=<where-you-install-wasi-libc>
```

* just installing wasi-libc, clang still fails with missing libclang_rt.builtins-wasm32.a (see https://depth-first.com/articles/2019/10/16/compiling-c-to-webassembly-and-running-it-without-emscripten/ ), so download it and copy libclang_rt.builtins-wasm32.a to the path clang complained with

```
git clone https://github.com/jedisct1/libclang_rt.builtins-wasm32.a
cp libclang_rt.builtins-wasm32.a/precompiled/libclang_rt.builtins-wasm32.a <the-path-clang-complained-about>
```


