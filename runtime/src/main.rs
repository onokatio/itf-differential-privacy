use std::env;
use wasmer::{Store, Module, Instance};
use wasmer_wasi;
mod imports;
//use anyhow;

fn help(){
    println!("usage:
    runtime <wasm path>");
}

fn main() -> anyhow::Result<()>{
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        help();
        return Err(anyhow::anyhow!("wasm file path is required"));
    }
    let path = &args[1];
    let mut store = Store::default();
    let module = Module::from_file(&store, path)?;
    let mut state_builder = wasmer_wasi::WasiState::new("wasi-prog-name");
    let wasi_env = state_builder.finalize(&mut store)?;

    let import_object = imports::import_object(&mut store, &wasi_env.env);
    let instance = Instance::new(&mut store,&module, &import_object)?;
    let memory = instance.exports.get_memory("memory")?;
    wasi_env.data_mut(&mut store).set_memory(memory.clone());

    let endpoint = "_start";
    let f = instance.exports.get_function(endpoint)?;
    f.call(&mut store,&[])?;

    //let result = add_one.call(&[Value::I32(42)])?;
    Ok(())
}
