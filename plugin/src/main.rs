use std::io;
use std::f64::consts;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum OutputType {
    Vec2Sum,
    Vec2Avg,
}

#[derive(Debug, Serialize, Deserialize)]
struct Output<T> {
    output_type: OutputType,
    value: T,
}


fn main() {
    let mut input_json_string = String::new();
    io::stdin().read_line(&mut input_json_string);
    let input_json: Output<Vec<u32>> = serde_json::from_str(&input_json_string).unwrap();
    //println!("{:?}", input_json_string);
    println!("{:?}", input_json);
    let y = Laplace(0.5, 0.0, 2.0);
    println!("{:?}", y);
}

// avg: データの平均値
// scale: 2*scale*scaleが分散になるような値
fn Laplace(x: f64, avg: f64, scale: f64) -> f64 {
    return 1.0/(2.0*scale) * consts::E.powf(-((x-avg)/scale));
}