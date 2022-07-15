//#![feature(start)]
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Score {
    name: String,
    point: u32,
}

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

fn filter1(scores: Vec<Score>) -> Vec<Score> {
    let mut ret_scores: Vec<Score> = Vec::new();
    for index in 0..5 {
        ret_scores.push(Score {
            name: scores[index].name.clone(),
            point: scores[index].point.clone() * 2,
        })
    }
    return ret_scores;
}

fn privacy_output_vec(scores: Vec<Score>) -> Vec<u32> {
    let mut ret: Vec<u32> = Vec::new();
    for index in 0..5 {
        ret.push(scores[index].point);
    }
    return ret;
}

//#[start]
//fn main(argc: isize, argv: *const *const u8) -> isize {
fn main(){
    let private_score: Vec<Score> = vec![
        Score {
            name: "Alice".to_string(),
            point: 100,
        },
        Score {
            name: "Bob".to_string(),
            point: 200,
        },
        Score {
            name: "Charlie".to_string(),
            point: 300,
        },
        Score {
            name: "David".to_string(),
            point: 400,
        },
        Score {
            name: "Eve".to_string(),
            point: 500,
        },
    ];
    /*
    let output1: Output<&Vec<Score>> = Output {
        output_type: OutputType::Vec2Sum,
        value: &private_score,
    };
    let json_string = serde_json::to_string(&output1);
    println!("{:?}\n", json_string.unwrap());
    */

    let filtered_score = filter1(private_score);
    let privacy_vec = privacy_output_vec(filtered_score);
    let output2: Output<&Vec<u32>> = Output {
        output_type: OutputType::Vec2Sum,
        value: &privacy_vec,
    };
    let json_string2 = serde_json::to_string(&output2);
    println!("{}",json_string2.unwrap());

    //return Ok(());
    //return 0;
}
