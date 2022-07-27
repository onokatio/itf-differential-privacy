use std::io;
use std::f64::consts;
use serde_derive::{Deserialize, Serialize};
use statrs::distribution::{Laplace, ContinuousCDF};
use rand::{thread_rng, Rng};

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
    println!("{:?}", input_json);

    let avg: u32 = input_json.value.iter().sum::<u32>() / input_json.value.len() as u32;
    println!("avg: {:?}", avg);
    let noise = Laplace::new(0.0,1000.0/(input_json.value.len() as f64 * 10.0)).unwrap().inverse_cdf(thread_rng().gen::<f64>());
    println!("avg with privacy: {:?}", avg as f64 + noise);
}