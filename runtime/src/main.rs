#[macro_use]
extern crate lazy_static;
use std::io::prelude::*;
use std::process::{Command, Stdio};

use std::env;
use std::sync::{Arc, Mutex};
use wasmer::{Instance, Memory32, MemorySize, Module, Store};
use wasmer_wasi::{WasiEnv, WasiState};
mod imports;
//use anyhow;

fn help() {
    println!(
        "usage:
    runtime <wasm path>"
    );
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        help();
        return Err(anyhow::anyhow!("wasm file path is required"));
    }
    let path = &args[1];
    let mut store = Store::default();
    let module = Module::from_file(&store, path)?;
    let mut state_builder = WasiState::new("wasi-prog-name");
    let wasi_env = state_builder.finalize(&mut store)?;

    let dp_buff: Vec<imports::Output<<Memory32 as MemorySize>::Offset>> = Vec::new();
    let dp_buff_am = Arc::new(Mutex::new(dp_buff));
    let import_object = imports::import_object::<Memory32>(&mut store, &wasi_env.env, &dp_buff_am);
    let instance = Instance::new(&mut store, &module, &import_object)?;
    let memory = instance.exports.get_memory("memory")?;
    wasi_env.data_mut(&mut store).set_memory(memory.clone());

    let endpoint = "_start";
    let f = instance.exports.get_function(endpoint)?;
    f.call(&mut store, &[])?;

    let dp_buff = imports::DP_BUFF.lock().unwrap();
    println!("{:?} ", dp_buff);

    let mut p = Command::new("../plugin/target/debug/plugin")
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    p.stdin
        .unwrap()
        .write_all("{\"output_type\": \"Vec2Sum\", \"value\": [0,1]}".as_bytes());
    //let result = add_one.call(&[Value::I32(42)])?;
    Ok(())
}
