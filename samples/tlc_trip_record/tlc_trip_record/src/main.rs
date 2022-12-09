use std::error::Error;
use std::process;
use std::io::Write;
use serde::Deserialize;

mod dp;

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
fn process_csv() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(std::io::stdin());
    let mut count_chan = dp::CntChan::new(0.1);
    let mut count = 0;
    for row in rdr.deserialize() {
        let row: YellowTripData = row?;
        if row.trip_distance > 1000.0 {
            count_chan.add();
            count += 1;
        }
    }
    //let mut wp = std::fs::File::create("count.txt")?;
    //writeln!(&mut wp, "count = {}", count)?;
    count_chan.output();
    Ok(())
}

fn main() {
    if let Err(err) = process_csv() {
        println!("error running example: {}", err);
        process::exit(1);
    }    
}
