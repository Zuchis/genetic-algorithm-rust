#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use ::helpers;

pub trait HasFitness<T> {
    fn fitness(&self, &Fn(&T) -> f64) -> f64;
}

impl<T> HasFitness<T> for T {
    fn fitness(&self, f: &Fn(&T) -> f64) -> f64 {
        f(&self)
    }
}

pub fn binary_alternate (ind: &Vec<bool>) -> f64 {
    let mut fit: f64 = 0.0;
    for i in 0 .. (ind.len() - 1) {
        if ind[i as usize] != ind[(i + 1) as usize]{
            fit = fit + 1.0;
        }
    }
    if fit == 0.0 {
        return 0.2
    }
    fit

}

pub fn bin_parps_function (ind: &Vec<bool>) -> f64 {
    let fit: f64;
    let float_value: f64;
    let converted: i64 = helpers::bin_to_int(ind);
    let bit_limitant = (2i64.pow(16) - 1) as f64;
    float_value = -2.0 + (4.0 / bit_limitant) * (converted as f64);
    fit = (float_value * 20.0).cos() - (float_value.abs() / 2.0) + (float_value.powi(3) / 4.0);
    fit + 4.0
}

pub fn bin_radio_factory (ind: &Vec<bool>) -> f64 {
    let fit: f64;
    let float_valueST: f64;
    let float_valueLX: f64;
    let (st_vec,lx_vec) = ind.split_at(5);
    let convertedST: i64 = helpers::bin_to_int(&st_vec.to_vec());
    let convertedLX: i64 = helpers::bin_to_int(&lx_vec.to_vec());
    let bit_limitant = (2i64.pow(5) - 1) as f64;
    float_valueST = (0.0 + (24.0 / bit_limitant) * (convertedST as f64)).ceil();
    float_valueLX = (0.0 + (16.0 / bit_limitant) * (convertedLX as f64)).ceil();
    // float_valueST = convertedST as f64;
    // float_valueLX = convertedLX as f64;
    fit = ((30.0 * float_valueST + 40.0 * float_valueLX) / 1360.0) - (helpers::maxf(0.0, (float_valueST + 2.0 * float_valueLX - 40.0) / 16.0));
    fit
}

pub fn bin_pattern_recognition (ind: &Vec<bool>) -> f64 {
    let fit: f64;
    let pattern: Vec<bool> = [false,true,false,false,false,false,
                              false,true,false,true,true,false,
                              false,true,false,true,false,false,
                              false,false,false,false,true,false,
                              false,true,true,true,false,false,
                              false,false,false,false,true,false].to_vec();
    fit = helpers::hamming_distance(ind,&pattern).into();
    36.0 - fit
}

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

pub fn float_quadratic_min (ind: &Vec<f64>) -> f64 {
    let mut fit: f64 = 0.0;
    static mut biggest: f64 = 0.0f64;
    let div: f64;

    for i in 0 .. ind.len() {
        let value = ind[i as usize];
        fit = fit + (value * value);
    }

    unsafe {
        biggest = helpers::maxf(biggest, fit);
        div = biggest;
    }

    1.0f64 - fit / div
}
