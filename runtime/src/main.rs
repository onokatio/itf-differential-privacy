use wasmer::{Store, Module, Instance, Function, imports};
use wasmer_wasi;

fn fd_write(env: &wasmer_wasi::WasiEnv, a: i32, b: i32, c: i32, d: i32) -> i32 {
    println!("fd_write(fd={:?},{:?},{:?},{:?})",a,b,c,d);
    return 0;
}
fn environ_get(a: i32, b: i32) -> i32 {
    println!("enciron_get {:?},{:?}",a,b);
    return 0;
}

fn environ_sizes_get(a: i32, b: i32) -> i32 {
    println!("environ_sizes_get {:?},{:?}",a,b);
    return 0;
}

fn proc_exit(a: i32) {
    println!("proc_exit {:?}",a);
}

fn privacy_out_vec(a: i32, b: i32) -> i32 {
    println!("privacy_out_vec {:?},{:?}",a,b);
    return 0;
}

fn main() -> anyhow::Result<()>{
    let path = "../samples/target/wasm32-wasi/debug/samples.wasm";
    //let path = "../samples/target/wasm32-unknown-unknown/debug/samples.wasm";
    eprintln!("[Runtime] Loading {}", path);
    let wasm_bytes = std::fs::read(path)?;
    let store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;
    let mut state_builder = wasmer_wasi::WasiState::new("wasi-prog-name");
    let state = state_builder.build()?;
    let env = wasmer_wasi::WasiEnv::new(state);
    let wasmer_import_object = wasmer_wasi::generate_import_object_from_env(&store,env,wasmer_wasi::WasiVersion::Snapshot1);
    let externs = wasmer_import_object.externs_vec();
    //externs["wasi_snapshot_preview1"]["fd_write"];
    /*
    let import_object = imports! {
        "wasi_snapshot_preview1" => {
            //."fd_write" => Function::new_native_with_env(&store, env.clone(), wasmer_wasi::syscalls::wasi::fd_write),
            //"fd_write" => Function::new_native_with_env(&store, env, fd_write),
            "environ_get" => Function::new_native(&store, environ_get),
            "environ_sizes_get" => Function::new_native(&store, environ_sizes_get),
            "proc_exit" => Function::new_native(&store, proc_exit),
            "privacy_out_vec" => Function::new_native(&store, privacy_out_vec),
        }
    };
    */
    let import_object = wasmer_import_object;
    eprintln!("[Runtime] Start {}", path);
    //let instance = Instance::new(&module, &import_object)?;
    let instance = Instance::new(&module, &import_object)?;
    match instance.exports.get_function("_start") {
        Ok(f) => {
            f.call(&[]);
        }
        ExportError => {
            println!("error");
        }
    }

    //let add_one = instance.exports.get_function("add_one")?;
    //let result = add_one.call(&[Value::I32(42)])?;
    assert_eq!(1,1);
    Ok(())
}
