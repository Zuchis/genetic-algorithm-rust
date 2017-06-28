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

pub fn fully_deceptive_f3 (ind: &Vec<bool>) -> f64 {
    let mut fit: f64 = 0.0;

    let values: Vec<f64> = vec![28.0, 26.0, 22.0, 0.0, 14.0, 0.0, 0.0, 30.0];

    let mut i: usize = 0;

    loop {
        let mut block: Vec<bool> = Vec::new();
        for j in 0usize .. 3 {
            block.push(ind[i]);
            i += 1;
        }
        let index: usize = helpers::bin_to_int(&block) as usize;
        fit += values[index];
        if i == (ind.len()) {
            break;
        }
    }

    fit / ((ind.len() / 3) as f64 * 30.0)
}

pub fn fully_deceptive_f3s (ind: &Vec<bool>) -> f64 {
    let mut fit: f64 = 0.0;

    let values: Vec<f64> = vec![28.0, 26.0, 22.0, 0.0, 14.0, 0.0, 0.0, 30.0];

    let mut i: usize = 0;

    let mut n: usize;

    loop {
        let mut block: Vec<bool> = Vec::new();
        n = 0;
        for j in 0usize .. 3 {
            block.push(ind[i + (n * 10)]);
            n += 1;
        }
        i += 1;
        let index: usize = helpers::bin_to_int(&block) as usize;
        fit += values[index];
        if i == (ind.len() / 3) {
            break;
        }
    }

    fit / ((ind.len() / 3) as f64 * 30.0)
}

pub fn deceptive_n (ind: &Vec<bool>) -> f64 {
    let mut fit: f64 = 0.0;

    let n: usize = 4;

    let mut i: usize = 0;

    loop {
        let mut ones: usize = 0;
        for j in 0usize .. n {
            if ind[i] == true {
                ones += 1;
            }
            i += 1;
        }
        match ones {
            0 => {
                fit += (n + 1) as f64;
            }
            _ => {
                fit += ones as f64;
            }
        }
        if i == (ind.len()) {
            break;
        }
    }

    fit / ((ind.len() / 4) * (n + 1)) as f64
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

pub fn n_queens (ind: &Vec<i64>) -> f64 {
    let mut colisions: f64 = 0.0;
    for i in 0usize .. (ind.len() - 1) {
        for j in (i + 1) .. ind.len() {
            if (ind[i] - ind[j]).abs() == (j - i) as i64 {
                colisions += 1.0;
            }
        }
    }
    1.0 - (colisions / (ind.len() as f64))
}

pub fn labirinth_minimum_path (ind: &Vec<i64>) -> f64 {
    let mut steps: usize = 0;
    let mut headbutts: usize = 0;
    let labirinth: Vec<Vec<u8>> = vec![
                                vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
                                vec![0,0,1,1,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,0,1,1,1,0,0],
                                vec![0,1,1,1,1,1,1,1,1,0,1,0,1,0,1,0,0,0,0,0,0,0,1,1,0],
                                vec![0,1,0,0,0,0,0,0,1,0,1,0,1,0,1,0,0,0,0,0,0,0,1,0,0],
                                vec![0,1,1,1,1,1,1,0,1,0,1,0,1,0,1,0,1,1,1,0,1,1,1,1,0],
                                vec![0,1,0,0,0,0,1,0,1,0,1,0,1,0,1,0,1,0,0,0,0,0,0,1,0],
                                vec![0,1,0,0,0,0,1,0,1,1,1,1,1,1,1,0,1,1,1,1,1,0,1,1,0],
                                vec![0,1,0,0,0,0,1,0,0,0,0,0,0,0,1,0,1,0,1,0,1,0,0,1,0],
                                vec![0,1,1,1,1,1,1,1,0,1,1,0,1,1,1,0,1,0,1,0,1,0,1,1,0],
                                vec![0,0,0,0,0,0,1,1,0,1,1,0,1,1,1,0,1,0,1,0,1,0,0,1,0],
                                vec![0,1,1,1,1,0,1,0,0,1,1,0,1,0,0,0,1,0,1,0,1,0,1,1,0],
                                vec![0,1,0,0,1,0,1,0,0,1,1,0,1,0,0,0,1,0,1,0,1,1,1,1,0],
                                vec![0,1,0,0,1,0,1,0,0,1,0,0,1,0,0,0,1,0,1,0,0,0,0,1,0],
                                vec![0,1,0,0,1,0,1,1,0,1,0,1,1,1,1,1,1,0,1,0,1,1,1,1,0],
                                vec![0,1,0,0,1,0,1,1,0,1,0,0,0,0,0,0,0,0,1,0,1,0,0,1,0],
                                vec![0,1,1,0,1,0,0,1,1,1,0,0,0,0,0,1,1,1,1,0,1,0,0,1,0],
                                vec![0,1,1,0,1,0,0,1,1,1,1,1,1,1,1,1,0,0,0,0,1,0,1,1,0],
                                vec![0,0,1,0,1,0,1,1,0,0,0,0,0,0,0,1,1,1,1,0,1,0,1,0,0],
                                vec![0,1,1,0,0,0,1,1,0,1,1,1,1,0,0,0,0,0,1,0,1,1,1,1,0],
                                vec![0,1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,0,1,0,0,1,0],
                                vec![0,0,0,0,1,0,0,0,0,1,1,0,1,1,1,0,1,0,1,0,1,1,0,1,0],
                                vec![0,1,1,1,1,0,1,1,1,1,1,0,1,0,1,0,1,0,1,0,0,1,0,1,0],
                                vec![0,1,1,0,1,0,1,0,0,0,1,0,1,0,1,0,1,0,1,0,1,1,0,1,0],
                                vec![0,1,1,0,1,0,1,0,0,0,1,0,1,0,1,0,1,0,1,0,1,0,0,1,0],
                                vec![0,1,1,0,1,0,1,0,0,0,1,0,1,0,1,0,1,0,1,0,1,1,0,1,0],
                                vec![0,1,1,0,1,0,1,0,0,0,1,0,1,0,1,0,1,0,0,0,0,1,0,1,0],
                                vec![0,0,0,0,1,0,1,1,1,1,1,1,1,1,1,0,1,0,0,1,1,1,1,1,0],
                                vec![0,1,1,1,1,0,1,0,0,0,0,0,0,0,0,0,1,0,0,1,0,0,1,0,0],
                                vec![0,1,1,0,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
                                vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
                                ];
    let mut x: usize = 1;
    let mut y: usize = 10;
    let final_x: usize = 20;
    let final_y: usize = 1;

    for i in 0usize .. ind.len() {
        match ind[i] {
            1 => {  // Left
                match labirinth[x-1][y] {
                    1 => {x = x - 1; steps += 1},
                    _ => {headbutts += 1},
                };
            }

            2 => { // Right
                match labirinth[x+1][y] {
                    1 => {x = x + 1; steps += 1},
                    _ => {headbutts += 1},
                };
            }
            3 => { // Down
                match labirinth[x][y+1] {
                    1 => {y = y + 1; steps += 1},
                    _ => {headbutts += 1},
                };
            }
            4 => { // Up
                match labirinth[x][y-1] {
                    1 => {y = y - 1; steps += 1},
                    _ => {headbutts += 1},
                };
            }
            _ => {
                panic!("Not a valid direction instruction!");
            }
        }
        steps += 1;
        if x == final_x && y == final_y {
            break;
        }
    }
    let total_steps: f64 = (steps + headbutts) as f64;
    (steps as f64 / total_steps) - (headbutts as f64 / total_steps)
    // 1.0 - (steps as f64 / 100.0)
    // ((final_x as i8 - x as i8).abs() + (final_y as i8 - y as i8).abs()) as f64
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
