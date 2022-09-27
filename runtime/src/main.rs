use std::env;
use wasmer::{Store, Module, Instance, Imports, Exports, FunctionEnvMut, WasmPtr, MemorySize};
use wasmer_wasi;
//use anyhow;

fn privacy_out_array5(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {
    eprintln!("[Runtime] privacy_out_array5({:?},{:?},{:?},{:?},{:?})",a,b,c,d,e);
    return 0;
}

/// ### `privacy_out_vec()`
/// Inputs:
/// - `const __wasi_iovec_t *iovs`
///     Vectors where data will be stored
/// - `size_t iovs_len`
///     Number of vectors
/// - `size_t nwrite`
///     Number of vectors write
fn privacy_out_vec<M: MemorySize>(
    ctx: &FunctionEnvMut<'_, wasmer_wasi::WasiEnv>,
    iovs: WasmPtr<wasmer_wasi::types::__wasi_iovec_t<M>, M>,
    iovs_len: M::Offset,
    nwrite: WasmPtr<M::Offset, M>,
) -> wasmer_wasi::types::__wasi_errno_t {
    let env = ctx.data();
    eprintln!("[Runtime] privacy_out_vec({:?}, {:?})", iovs, iovs_len);

    let (memory, mut state, inodes) = env.get_memory_and_wasi_state_and_inodes(&ctx, 0);
    let iovs = wasmer_wasi::macros::wasi_try_mem_ok!(iovs.slice(&memory, iovs_len));
    let privacy_vec = iovs.deref(&memory);
    return 0;
}

#[allow(dead_code)]
fn deny_syscall_2(_: i32, _: i32) -> i32 {
    eprintln!("deny_syscall");
    return 0;
}

#[allow(dead_code)]
fn deny_syscall_4(_: i32, _: i32, _: i32, _: i32) -> i32 {
    eprintln!("deny_syscall");
    return 12;
}

fn help(){
    println!("usage:
    runtime <wasm path>");
}

fn main() -> anyhow::Result<()>{
    let args: Vec<String> = env::args().collect();
    //let path = "../samples/target/wasm32-wasi/debug/samples.wasm";
    //let path = "../samples/target/wasm32-unknown-unknown/debug/samples.wasm";
    if args.len() != 2 {
        help();
        return Err(anyhow::anyhow!("wasm file path is required"));
    }
    let path = &args[1];
    eprintln!("[Runtime] Creating environment(store)...");
    let mut store = Store::default();
    eprintln!("[Runtime] Loading {}...", path);
    let wasm_bytes = std::fs::read(path)?;
    let module = Module::new(&store, wasm_bytes)?;
    eprintln!("[Runtime] Creating environment(state)...");
    let mut state_builder = wasmer_wasi::WasiState::new("wasi-prog-name");
    let state = state_builder.build()?;
    eprintln!("[Runtime] Creating environment(env)...");
    let env = wasmer_wasi::WasiFunctionEnv::new(&mut store, wasmer_wasi::WasiEnv::new(state)).env;
    eprintln!("[Runtime] Loading syscall implemention...");
    let mut wasix_32v1 = wasmer_wasi::generate_import_object_from_env(&mut store,&env,wasmer_wasi::WasiVersion::Wasix32v1).get_namespace_exports("wasix_32v1").unwrap();
    let mut wasix_64v1 = wasmer_wasi::generate_import_object_from_env(&mut store,&env,wasmer_wasi::WasiVersion::Wasix64v1).get_namespace_exports("wasix_64v1").unwrap();
    let mut wasi_snapshot_preview1 = wasmer_wasi::generate_import_object_from_env(&mut store,&env,wasmer_wasi::WasiVersion::Snapshot1).get_namespace_exports("wasi_snapshot_preview1").unwrap();
    //wasi_snapshot_preview1.insert("random_get", wasmer::Function::new_native(&store, deny_syscall_2));
    //wasi_snapshot_preview1.insert("fd_write", wasmer::Function::new_native(&store, deny_syscall_4));
    let mut wasi_dp_preview1 = Exports::new();
    wasi_dp_preview1.insert("privacy_out_array5", wasmer::Function::new_typed(&mut store, privacy_out_array5));
    wasi_dp_preview1.insert("privacy_out_vec", wasmer::Function::new_typed(&mut store, privacy_out_vec));
    let mut import_object = Imports::new();
    //import another function
    //wasi_snapshot_preview1.insert("sock_accept", wasix_32v1.get_function("sock_accept").unwrap().clone());
    import_object.register_namespace("wasi_snapshot_preview1", wasi_snapshot_preview1);
    import_object.register_namespace("wasix_64v1", wasix_64v1);
    import_object.register_namespace("wasix_32v1", wasix_32v1);
    import_object.register_namespace("wasi_dp_preview1", wasi_dp_preview1);
    eprintln!("[Runtime] Start {}", path);
    let instance = Instance::new(&mut store,&module, &import_object)?;
    match instance.exports.get_function("_start") {
        Ok(f) => {
            match f.call(&mut store,&[]) {
                Ok(_) => {
                    eprintln!("[Runtime] Done {}", path);
                },
                Err(e) => {
                    eprintln!("[Runtime] Error {}", e);
                }
            };
        }
        //ExportError => {
        _ => {
            eprintln!("error");
        }
    }

    //let add_one = instance.exports.get_function("add_one")?;
    //let result = add_one.call(&[Value::I32(42)])?;
    Ok(())
}
