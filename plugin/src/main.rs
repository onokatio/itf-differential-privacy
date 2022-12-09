use rand::{thread_rng, Rng};
use serde_derive::{Deserialize, Serialize};
use statrs::distribution::{ContinuousCDF, Laplace};
//use std::f64::consts;
use std::io;

//const SIGMA: usize = 10;

#[derive(Debug, Serialize, Deserialize)]
enum OutputType {
    Vec2Sum,
    Vec2Avg,
    Vec2Cnt,
}

#[derive(Debug, Serialize, Deserialize)]
struct Output<T> {
    output_type: OutputType,
    eps: f64,
    clip: f64,
    value: T,
}

fn main() {
    let mut input_json_string = String::new();
    io::stdin().read_line(&mut input_json_string).unwrap();
    let input_json = json2vec(input_json_string);

    match input_json.output_type {
        OutputType::Vec2Sum => {
            let avg = dp_sum(input_json.value, input_json.eps, input_json.clip);
            println!("sum + noise: {:?}", avg);
        },
        OutputType::Vec2Avg => {
            let avg = dp_avg(input_json.value, input_json.eps, input_json.clip);
            println!("avg + noise: {:?}", avg);
        }
        OutputType::Vec2Cnt => {
            let avg = dp_cnt(input_json.value, input_json.eps);
            println!("count + noise: {:?}", avg);
        }
    }
}

fn json2vec(input: String) -> Output<Vec<f64>> {
    println!("[privacy gateway] received {:?}", &input);
    let input_json: Output<Vec<f64>> = serde_json::from_str(&input).unwrap();
    return input_json;
}

fn laplace_noise(b: f64) -> f64 {
    let lap = Laplace::new(0.0, b).unwrap();
    lap.inverse_cdf(thread_rng().gen::<f64>())
}

fn dp_avg(data: Vec<f64>, eps: f64, clip: f64) -> f64 {
    let n = data.len() as f64;
    let b = clip / (eps * n);
    let noise = laplace_noise(b);
    let avg = data.iter().sum::<f64>() / n;
    println!("[privacy gateway] avg: {:?}", avg);
    return avg + noise;
}

fn dp_sum(data: Vec<f64>, eps: f64, clip: f64) -> f64 {
    let b = clip / eps;
    let noise = laplace_noise(b);
    let sum = data.iter().sum::<f64>();
    println!("[gateway] sum: {:?}", sum);
    return sum + noise;
}

fn dp_cnt(data: Vec<f64>, eps: f64) -> f64 {
    let b = 1.0 / eps;
    let noise = laplace_noise(b);
    let count = data[0];
    println!("[privacy gateway] count: {:?}", count);
    return count + noise;
}
