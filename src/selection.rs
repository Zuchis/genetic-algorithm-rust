extern crate rand;

use self::rand::Rng;
use self::rand::distributions::{IndependentSample, Range};

use population::Population;

pub fn wheel<T>(pop: &mut Population<T>) -> usize {
    let sum: f64 = pop.fit_array.iter().fold(0.0, |a, &b| a + b);
    let roulete_position = Range::new(0.0, sum).ind_sample(&mut rand::thread_rng());
    let mut accumulator: f64 = 0.0;
    let mut chosen: usize = 0;
    for i in 0..pop.fit_array.len() {
        accumulator += pop.fit_array[i as usize];
        if accumulator >= roulete_position {
            chosen = i as usize;
            break;
        }
    }
    chosen
}
