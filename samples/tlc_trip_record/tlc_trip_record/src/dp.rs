#[link(wasm_import_module = "wasi_dp_preview1")]
extern "C" {
    pub fn privacy_out_vec(vecs: *const i32,
                           len: i32,
                           output_type: u32, // 0 : sum, 1 : avg, 2 : cnt
                           eps: f64,
                           clip: f64,
                           nwritten: &i32) -> i32;
}

#[allow(dead_code)]
pub struct SumChan {
    vals : Vec<i32>,
    eps : f64,
    clip : f64
}

#[allow(dead_code)]
impl SumChan {
    pub fn new(eps: f64, clip: f64) -> Self {
        Self { vals : Vec::<i32>::new(), eps: eps, clip: clip }
    }
    pub fn add(&mut self, x : i32) {
        self.vals.push(x)
    }
    pub fn output(&self) {
        let nwritten = 0;
        let vals = &self.vals;
        let eps = self.eps;
        let clip = self.clip;
        unsafe {
            privacy_out_vec(vals.as_ptr(), vals.len() as i32,
                            0, eps, clip, &nwritten);
        }
    }
}

#[allow(dead_code)]
pub struct AvgChan {
    vals : Vec<i32>,
    eps : f64,
    clip : f64
}

#[allow(dead_code)]
impl AvgChan {
    pub fn new(eps: f64, clip: f64) -> Self {
        Self { vals : Vec::<i32>::new(), eps: eps, clip: clip }
    }
    pub fn add(&mut self, x : i32) {
        self.vals.push(x)
    }
    pub fn output(&self) {
        let nwritten = 0;
        let vals = &self.vals;
        let eps = self.eps;
        let clip = self.clip;
        unsafe {
            privacy_out_vec(vals.as_ptr(), vals.len() as i32,
                            1, eps, clip, &nwritten);
        }
    }
}

#[allow(dead_code)]
pub struct CntChan {
    vals : Vec<i32>,
    eps : f64,
}

#[allow(dead_code)]
impl CntChan {
    pub fn new(eps : f64) -> Self {
        Self { vals : vec![1], eps : eps }
    }
    pub fn add(&mut self) {
        self.vals[0] += 1;
    }
    pub fn output(&self) {
        let nwritten = 0;
        let vals = &self.vals;
        let eps = self.eps;
        unsafe {
            privacy_out_vec(vals.as_ptr(), vals.len() as i32,
                            2, eps, 0.0, &nwritten);
        }
    }
}

