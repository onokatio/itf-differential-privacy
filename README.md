# Diffirential Privacy WASM Runtime

## Build

1. Prepare environment
```
$ rustup target add wasm32-wasi
```

2. compile sample

```
$ cd samples/rust
$ cargo build --target wasm32-wasi --release
```

3. Launch runtime

```
$ cd runtime
$ cargo run ../samples/rust/target/wasm32-wasi/release/samples.wasm
# or
$ cargo build
$ ./target/debug/runtime ../samples/rust/target/wasm32-wasi/release/samples.wasm
```