use rand::{thread_rng, Rng};

struct Score {
    name: String,
    point: i32,
}

enum OutputType {
    Vec2Sum,
    Vec2Avg,
}

struct Output<T> {
    output_type: OutputType,
    value: T,
}

#[link(wasm_import_module = "wasi_dp_preview1")]
extern "C" {
    pub fn privacy_out_array5(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32;
    pub fn privacy_out_vec(vecs: *const i32, len: i32, nwritten: &i32) -> i32;
}

/*
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
*/

fn score2vec(scores: Vec<Score>) -> Vec<i32> {
    let mut ret: Vec<i32> = Vec::new();
    for index in 0..5 {
        ret.push(scores[index].point);
    }
    return ret;
}

//#[start]
//fn main(argc: isize, argv: *const *const u8) -> isize {
fn main(){
    println!("Hello World!");
    let mut rng = thread_rng();
    let private_score: Vec<Score> = vec![
        Score {
            name: "Alice".to_string(),
            point: rng.gen_range(0..100),
        },
        Score {
            name: "Bob".to_string(),
            point: rng.gen_range(0..100),
        },
        Score {
            name: "Charlie".to_string(),
            point: rng.gen_range(0..100),
        },
        Score {
            name: "David".to_string(),
            point: rng.gen_range(0..100),
        },
        Score {
            name: "Eve".to_string(),
            point: rng.gen_range(0..100),
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

    //let filtered_score = filter1(private_score);
    let privacy_vec = score2vec(private_score);
    let output2: Output<&Vec<i32>> = Output {
        output_type: OutputType::Vec2Sum,
        value: &privacy_vec,
    };
    let v = output2.value;
    let nwritten = 0;
    unsafe {
        privacy_out_array5(v[0], v[1], v[2], v[3], v[4]);
        privacy_out_vec(v.as_ptr(), v.len() as i32, &nwritten);
    }
    println!("{} elements written", nwritten);

    //return Ok(());
    //return;
}
