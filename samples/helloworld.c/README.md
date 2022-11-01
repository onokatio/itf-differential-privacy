## Compile

for apple silicon, use `arch -x86_64` before compiler command.


### for WASM

```
$ path/to/wasi-sdk-16.0/bin/clang -O3 -o main.wasm ./main.c
```