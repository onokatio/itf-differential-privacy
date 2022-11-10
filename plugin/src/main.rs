use rand::{thread_rng, Rng};
use serde_derive::{Deserialize, Serialize};
use statrs::distribution::{ContinuousCDF, Laplace};
use std::f64::consts;
use std::io;

const SIGMA: usize = 10;

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
    io::stdin().read_line(&mut input_json_string).unwrap();
    let input_json = json2vec(input_json_string);

    let avg = dp_avg(input_json.value, 100.0);
    println!("avg with privacy: {:?}", avg);
}

fn json2vec(input: String) -> Output<Vec<f64>> {
    let input_json: Output<Vec<f64>> = serde_json::from_str(&input).unwrap();
    println!("{:?}", input_json);
    return input_json;
}

fn dp_avg(data: Vec<f64>, max: f64) -> f64 {
    let lap = Laplace::new(0.0, max / (data.len() * SIGMA) as f64).unwrap();
    let noise = lap.inverse_cdf(thread_rng().gen::<f64>());
    let avg = data.iter().sum::<f64>() / data.len() as f64;
    println!("avg: {:?}", avg);
    return avg + noise;
}
