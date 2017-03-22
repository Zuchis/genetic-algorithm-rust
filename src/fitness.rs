#[allow(dead_code)]
pub fn binary_alternate (ind: &Vec<bool>) -> f64 {
    let mut fit: f64 = 0.0;
    for i in 0 .. (ind.len() - 1) {
        if ind[i as usize] != ind[(i + 1) as usize]{
            fit = fit + 1.0;
        }
    }
    fit
}

#[allow(dead_code)]
pub fn int_parity_alternate (ind: &Vec<i64>) -> f64 {
    let mut fit: f64 = 0.0;
    for i in 0 .. (ind.len() - 1) {
        let current: usize = i as usize;
        let next: usize = i + 1 as usize;
        if ((ind[current] % 2 == 0) && (ind[next] % 2 != 0)) || ((ind[current] % 2 != 0) && (ind[next] % 2 == 0)){
            fit = fit + 1.0;
        }
    }
    fit
}

#[allow(dead_code)]
pub fn float_quadratic_min (ind: &Vec<f64>) -> f64 {
    let mut fit: f64 = 0.0;
    for i in 0 .. ind.len() {
        let value = ind[i as usize];
        fit = fit + (value * value);
    }
    1.0 / fit
}
