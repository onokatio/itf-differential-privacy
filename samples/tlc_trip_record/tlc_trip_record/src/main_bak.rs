use std::error::Error;
use std::env;
//use std::fs;
use std::process;
use serde::Deserialize;

mod dp;
/* uncomment this and other sections below to process sqlite.
   however, 
   I don't know how to fix the compilation error while compiling sqlite module
   with --target wasm32-wasi (lack of stdio.h). see README.md */
// use sqlite;

// data structure for yellow_tripdata
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct YellowTripData {
    #[serde(rename = "VendorID")]
    vendor_id: i32, // 1
    tpep_pickup_datetime: String,  // 2022-01-01 00:35:40
    tpep_dropoff_datetime: String, // 2022-01-01 00:53:29
    passenger_count: Option<f32>, // 2.0
    trip_distance: f32, // 3.8
    #[serde(rename = "RatecodeID")]
    ratecode_id: Option<f32>, // 1.0,
    store_and_fwd_flag: Option<String>, // N
    #[serde(rename = "PULocationID")]
    pu_location_id: i32, // 142
    #[serde(rename = "DOLocationID")]
    do_location_id: i32, // 236
    payment_type: i32, // 1
    fare_amount: f32, // 14.5
    extra: f32, // 3.0
    mta_tax: f32, // 0.5
    tip_amount: f32, // 3.65
    tolls_amount: f32, // 0.0
    improvement_surcharge: f32, // 0.3
    total_amount: f32, // 21.95
    congestion_surcharge: Option<f32>, // 2.5
    airport_fee: Option<f32>, // 0.0
}

// process a csv file
fn process_csv(_trip_csv : &str) -> Result<(), Box<dyn Error>> {
    //let mut rdr = csv::Reader::from_reader(fs::File::open(trip_csv)?);
    let mut rdr = csv::Reader::from_reader(std::io::stdin());
    let mut count = dp::CntChan::new(1.0);
    for row in rdr.deserialize() {
        let _row: YellowTripData = row?;
        count.add();
        //println!("{:?}", row);
    }
    count.output();
    //println!("{:?} records", count);
    Ok(())
}

/* uncomment this to process sqlite file */
/*
fn process_sqlite(trip_sqlite : &str, query : &str) -> Result<(), Box<dyn Error>> {
    let conn = sqlite::open(trip_sqlite).unwrap();
    conn
        .iterate(query, |cell| {
            for &(key, val) in cell.iter() {
                println!("{} = {}", key, val.unwrap());
            }
            true
        })
        .unwrap();
    Ok(())
}
 */

fn main() {
    let args: Vec<String> = env::args().collect();
    let in_file = if args.len() > 1 { &args[1] } else { "../samples/tlc_trip_record/data/yellow_tripdata_2022-01.csv" };
    if in_file.ends_with(".csv") {
        if let Err(err) = process_csv(in_file) {
            println!("error running example: {}", err);
            process::exit(1);
        }    
    }
    /* uncomment this to process sqlite
    else if in_file.ends_with(".sqlite") {
        let query = &args[2];     // "select distinct dispatching_base_num from trip"
        if let Err(err) = process_sqlite(in_file, query) {
            println!("error running example: {}", err);
            process::exit(1);
        }    
    }
     */
}
