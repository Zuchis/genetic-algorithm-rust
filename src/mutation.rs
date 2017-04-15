#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

extern crate rand;

use self::rand::Rng;
use self::rand::distributions::{IndependentSample, Range};
use self::rand::distributions::normal::StandardNormal;

use ::helpers;

use population::Population;

pub fn bit_flip(pop: &mut Population<bool>, ind: usize, index: usize) {
    pop.individuals[ind][index] = !pop.individuals[ind][index];
}

pub fn int_delta_mutation(pop: &mut Population<i64>, ind: usize, index: usize) {
    let mut rng = rand::thread_rng();
    let lb = pop.lower_bound.floor() as i64;
    let ub = pop.upper_bound.floor() as i64;
    let interval = Range::new(lb, ub);
    let fifty_fifty: bool = rng.gen::<bool>();
    let sign = if fifty_fifty {1i64} else {-1i64};
    let ten_percent = interval.ind_sample(&mut rng) / 10;
    pop.individuals[ind][index] = helpers::clamp(pop.individuals[ind][index] + (ten_percent * sign),lb,ub);
}

pub fn swap_position(pop: &mut Population<i64>, ind: usize, index: usize) {
    let mut rng = rand::thread_rng();
    let between = Range::new(0usize, pop.ind_size as usize);
    let temp: i64;
    let mut swaped: usize;
    loop {
        swaped = between.ind_sample(&mut rng);
        if swaped != index {
            break;
        }
    }
    temp = pop.individuals[ind][index];
    pop.individuals[ind][index] = pop.individuals[ind][swaped];
    pop.individuals[ind][swaped] = temp;
}

pub fn delta_mutation(pop: &mut Population<f64>, ind: usize, index: usize) {
    let interval = Range::new(pop.lower_bound, pop.upper_bound);
    let mut rng = rand::thread_rng();
    let fifty_fifty: bool = rng.gen::<bool>();
    let sign = if fifty_fifty {1.0f64} else {-1.0f64};
    let ten_percent = interval.ind_sample(&mut rng) / 10.0f64;
    pop.individuals[ind][index] = helpers::clampf(pop.individuals[ind][index] + (ten_percent * sign),pop.lower_bound,pop.upper_bound);
}
