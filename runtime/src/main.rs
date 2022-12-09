#[macro_use]
extern crate lazy_static;

use std::io::prelude::*;
use std::process::{Command, Stdio};

use std::env;
use std::sync::{Arc, Mutex};

use wasmer::{Instance, Memory32, MemorySize, Module, Store};
use wasmer_wasi::{WasiState}; // WasiEnv, 
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
    let dp_request = 
        match *imports::OUTPUTTYPE.lock().unwrap() {
            imports::OutputType::Vec2Sum => imports::sum(*imports::EPS.lock().unwrap(), dp_buff.to_vec()),
            imports::OutputType::Vec2Avg => imports::avg(*imports::EPS.lock().unwrap(), *imports::CLIP.lock().unwrap(), dp_buff.to_vec()),
            imports::OutputType::Vec2Cnt => imports::cnt(*imports::EPS.lock().unwrap(), *imports::CLIP.lock().unwrap(), dp_buff.to_vec())
        };
    //imports::sum(*imports::EPS.lock().unwrap(), dp_buff.to_vec());
    //let dp_request = imports::sum(*imports::EPS.lock().unwrap(), dp_buff.to_vec());
    let dp_request_str = serde_json::to_string(&dp_request).unwrap();
    println!("[runtime] sends {:?} -> privacy gateway", &dp_request_str);
    
    let p = Command::new("../plugin/target/release/plugin")
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();

    let _res = p.stdin.unwrap().write_all(dp_request_str.as_bytes());
    
    // .write_all("{\"output_type\": \"Vec2Sum\", \"value\": [0,1]}".as_bytes());
    //let result = add_one.call(&[Value::I32(42)])?;
    Ok(())
}
