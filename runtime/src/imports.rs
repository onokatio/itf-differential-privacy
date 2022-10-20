use wasmer::{Store, Imports, Exports, WasmPtr, MemorySize, Memory32, FunctionEnv, FunctionEnvMut};
use wasmer_wasi::WasiEnv;

pub fn import_object(store: &mut Store, env: &FunctionEnv<WasiEnv>) -> Imports {
    let mut import_object = Imports::new();
    import_object.register_namespace("wasi_snapshot_preview1", wasi_exports(store, env));
    import_object.register_namespace("wasi_dp_preview1", wasi_dp_exports(store, env));
    return import_object;
}

fn wasi_exports(store: &mut Store, env: &FunctionEnv<WasiEnv>) -> Exports {
    let mut wasi_snapshot_preview1 = wasmer_wasi::generate_import_object_from_env(store,env,wasmer_wasi::WasiVersion::Snapshot1).get_namespace_exports("wasi_snapshot_preview1").unwrap();
    //wasi_snapshot_preview1.insert("random_get", wasmer::Function::new_native(&store, deny_syscall_2));
    //wasi_snapshot_preview1.insert("fd_write", wasmer::Function::new_native(&store, deny_syscall_4));
    return wasi_snapshot_preview1;
}

fn wasi_dp_exports(store: &mut Store, env: &FunctionEnv<WasiEnv>) -> Exports {
    let mut wasi_dp = Exports::new();
    wasi_dp.insert("privacy_out_array5", wasmer::Function::new_typed(store, privacy_out_array5));
    wasi_dp.insert("privacy_out_vec", wasmer::Function::new_typed_with_env(store, env, privacy_out_vec::<Memory32>));
    return wasi_dp;
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
    ctx: FunctionEnvMut<'_, wasmer_wasi::WasiEnv>,
    iovs: WasmPtr<wasmer_wasi::types::__wasi_iovec_t<M>, M>,
    iovs_len: M::Offset,
    nwritten: WasmPtr<M::Offset, M>,
) -> wasmer_wasi::types::__wasi_errno_t {
    let env = ctx.data();
    eprintln!("[Runtime] privacy_out_vec({:?}, {:?})", iovs, iovs_len);

    let memory = env.memory();
    let mut state = env.state();
    let iovs = match iovs.slice(&env.memory_view(&ctx), iovs_len) {
        Ok(iovs) => iovs,
        Err(e) => panic!("address invalid {}", e),
    };
    let nwritten = nwritten.deref(&env.memory_view(&ctx));
    return 0;
}

fn privacy_out_array5(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {
    eprintln!("[Runtime] privacy_out_array5({:?},{:?},{:?},{:?},{:?})",a,b,c,d,e);
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