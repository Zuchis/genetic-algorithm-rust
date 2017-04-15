#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

extern crate rand;

use self::rand::Rng;
use self::rand::distributions::{IndependentSample, Range};


#[allow(dead_code)]
pub fn one_point_crossover<T>(parent1: &mut Vec<T>, parent2: &mut Vec<T>)
    where T: Clone {
    let cut_position: usize = Range::new(1usize, (parent1.len() - 1) as usize).ind_sample(&mut rand::thread_rng());
    let split_1 = parent1.split_off(cut_position);
    let split_2 = parent2.split_off(cut_position);

    parent1.extend_from_slice(&split_2);
    parent2.extend_from_slice(&split_1);
}

#[allow(dead_code)]
pub fn uniform_crossover<T>(parent1: &mut Vec<T>, parent2: &mut Vec<T>)
    where T: Clone {
    let mut rng = rand::thread_rng();

    for i in 0usize .. parent1.len() {
        let fifty_fifty: bool = rng.gen::<bool>();
        if fifty_fifty {
            let value = &parent1[i];
            parent1[i] = parent2[i];
            parent2[i] = value;
        }
    }
}
