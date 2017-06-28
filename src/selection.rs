#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

extern crate rand;

use std::process;

use self::rand::Rng;
use self::rand::thread_rng;
use self::rand::distributions::{IndependentSample, Range};

use population::Population;

#[allow(dead_code)]
pub fn wheel<T>(pop: &mut Population<T>) -> usize
    where T: Clone {
    let fit_array = if pop.linear_scaling == false {pop.fit_array.clone()} else {pop.fitness_scaling()};
    let sum: f64 = fit_array.iter().fold(0.0, |a, &b| a + b);
    let roulete_position = Range::new(0.0, sum).ind_sample(&mut rand::thread_rng());
    let mut accumulator: f64 = 0.0;
    let mut chosen: usize = 0;
    for i in 0 .. fit_array.len() {
        accumulator += fit_array[i as usize];
        if accumulator >= roulete_position {
            chosen = i as usize;
            break;
        }
    }
    chosen
}

#[allow(dead_code)]
pub fn tournament<T>(pop: &mut Population<T>) -> usize
    where T: Clone {
    if pop.tournament_size > pop.pop_size as u64 {
        println!("Error: The tournament size is bigger than the size of the population");
        process::exit(1);
    }
    let mut indexes: Vec<usize> = (0usize .. pop.pop_size as usize).collect();
    thread_rng().shuffle(&mut indexes);

    let mut winner: usize = indexes.pop().unwrap();

    for _ in 1..pop.tournament_size {
        let next_challenger = indexes.pop().unwrap();
        if pop.fit_array[next_challenger] > pop.fit_array[winner] {
            winner = next_challenger;
        }
    }

    winner
}
