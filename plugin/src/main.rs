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

    let avg = dp_avg(input_json.value,1000.0);
    println!("avg with privacy: {:?}", avg);
}

fn dp_avg(data: Vec<u32>, max: f64) -> f64 {
    let lap = Laplace::new(0.0,max/(data.len() as f64 * 10.0)).unwrap();
    let noise = lap.inverse_cdf(thread_rng().gen::<f64>());
    let avg: f64 = data.iter().sum::<u32>() as f64 / data.len() as f64;
    return avg + noise;
}