use wasmer::{Store, Imports, Exports, WasmPtr, MemorySize, Memory32, FunctionEnv, FunctionEnvMut};
use wasmer_wasi::WasiEnv;

pub fn import_object(store: &mut Store, env: &FunctionEnv<WasiEnv>) -> Imports {
    let mut import_object = Imports::new();
    import_object.register_namespace("wasi_snapshot_preview1", wasi_exports(store, env));
    import_object.register_namespace("wasi_dp_preview1", wasi_dp_exports(store, env));
    return import_object;
}

struct DenySyscall {
    name: String,
    argc: u32,
}

fn wasi_exports(store: &mut Store, env: &FunctionEnv<WasiEnv>) -> Exports {
    let mut wasi_snapshot_preview1 = wasmer_wasi::generate_import_object_from_env(store,env,wasmer_wasi::WasiVersion::Snapshot1).get_namespace_exports("wasi_snapshot_preview1").unwrap();
    let deny_syscall_list = vec![
        DenySyscall { name: String::from("hoge"), argc: 2 },
        //DenySyscall { name: String::from("random_get"), argc: 2 },
        //DenySyscall { name: String::from("fd_write"), argc: 4 },
    ];
    for s in deny_syscall_list.iter() {
        match s.argc {
            2 => wasi_snapshot_preview1.insert(&s.name, wasmer::Function::new_typed(store, deny_syscall_2)),
            4 => wasi_snapshot_preview1.insert(&s.name, wasmer::Function::new_typed(store, deny_syscall_4)),
            _ => panic!("Unexpected number of arguments to wasi_exports: {}", s.argc),
        }
    }
    return wasi_snapshot_preview1;
}

fn wasi_dp_exports(store: &mut Store, env: &FunctionEnv<WasiEnv>) -> Exports {
    let mut wasi_dp = Exports::new();
    wasi_dp.insert("privacy_out_array5", wasmer::Function::new_typed(store, privacy_out_array5::<Memory32>));
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
    iovs: WasmPtr<M::Offset, M>,
    iovs_len: M::Offset,
    nwritten: WasmPtr<i32, M>,
) -> wasmer_wasi::types::__wasi_errno_t {
    let env = ctx.data();
    eprintln!("[Runtime] privacy_out_vec({:?}, {:?})", iovs, iovs_len);

    let memory = env.memory_view(&ctx);
    let iovs = match iovs.slice(&memory, iovs_len) {
        Ok(iovs) => iovs,
        Err(e) => {
            eprintln!("address invalid {}", e);
            return wasmer_wasi::types::__WASI_EINVAL
        },
    };
    let nwritten_ref = nwritten.deref(&memory);
    let mut nwritten = 0;

    for i in iovs.iter() {
        match i.read() {
            Ok(i) => {
                println!("[Runtime] privacy_out_vec: iovs[] = {}",i);
                nwritten+=1;
            },
            Err(e) => {
                eprintln!("WasmRef read failed {}", e);
                return wasmer_wasi::types::__WASI_EINVAL
            },
        };
    }
    match nwritten_ref.write(nwritten) {
        Err(e) => {
            eprintln!("WasmRef write failed {}", e);
            return wasmer_wasi::types::__WASI_EINVAL
        },
        _ => {},
    };
    return wasmer_wasi::types::__WASI_ESUCCESS;
}

fn privacy_out_array5<M: MemorySize>(a: M::Native, b: M::Native, c: M::Native, d: M::Native, e: M::Native) -> i32 {
    eprintln!("[Runtime] privacy_out_array5({:?},{:?},{:?},{:?},{:?})",M::native_to_offset(a),M::native_to_offset(b),M::native_to_offset(c),M::native_to_offset(d),M::native_to_offset(e));
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