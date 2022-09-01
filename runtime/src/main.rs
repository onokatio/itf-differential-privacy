use std::env;
use wasmer::{Store, Module, Instance, ImportObject, Exports};
use wasmer_wasi;
//use anyhow;

fn privacy_out_array5(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {
    eprintln!("[Runtime] privacy_out_array5({:?},{:?},{:?},{:?},{:?})",a,b,c,d,e);
    return 0;
}
fn privacy_out_vec(nums: i32, len: i32) -> i32 {
    eprintln!("[Runtime] privacy_out_vec({:?}, {:?})", nums, len);
    return 0;
}

fn deny_syscall_2(_: i32, _: i32) -> i32 {
    println!("deny_syscall");
    return 0;
}
fn deny_syscall_4(_: i32, _: i32, _: i32, _: i32) -> i32 {
    println!("deny_syscall");
    return 0;
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
    eprintln!("[Runtime] Loading {}", path);
    let wasm_bytes = std::fs::read(path)?;
    let store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;
    let mut state_builder = wasmer_wasi::WasiState::new("wasi-prog-name");
    let state = state_builder.build()?;
    let env = wasmer_wasi::WasiEnv::new(state);
    let wasmer_import_object = wasmer_wasi::generate_import_object_from_env(&store,env,wasmer_wasi::WasiVersion::Snapshot1);
    let mut wasi_snapshot_preview1 = wasmer_import_object.get_namespace_exports("wasi_snapshot_preview1").unwrap();
    //wasi_snapshot_preview1.insert("random_get", wasmer::Function::new_native(&store, deny_syscall_2));
    //wasi_snapshot_preview1.insert("fd_write", wasmer::Function::new_native(&store, deny_syscall_4));
    let mut wasi_dp_preview1 = Exports::new();
    wasi_dp_preview1.insert("privacy_out_array5", wasmer::Function::new_native(&store, privacy_out_array5));
    wasi_dp_preview1.insert("privacy_out_vec", wasmer::Function::new_native(&store, privacy_out_vec));
    let mut import_object = ImportObject::new();
    import_object.register("wasi_snapshot_preview1", wasi_snapshot_preview1);
    import_object.register("wasi_dp_preview1", wasi_dp_preview1);
    eprintln!("[Runtime] Start {}", path);
    let instance = Instance::new(&module, &import_object)?;
    match instance.exports.get_function("_start") {
        Ok(f) => {
            match f.call(&[]) {
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
