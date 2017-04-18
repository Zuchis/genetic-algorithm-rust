#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

extern crate rand;

use self::rand::Rng;
use self::rand::distributions::{IndependentSample, Range};

use ::helpers;

extern crate num;
use self::num::{Num, Zero, One, Signed};

use std::ops::{Add,Div,Sub,Mul};

pub fn one_point_crossover<T>(parent1: &mut Vec<T>, parent2: &mut Vec<T>)
    where T: Clone {
    let cut_position: usize = Range::new(1usize, (parent1.len() - 1) as usize).ind_sample(&mut rand::thread_rng());
    let split_1 = parent1.split_off(cut_position);
    let split_2 = parent2.split_off(cut_position);

    parent1.extend_from_slice(&split_2);
    parent2.extend_from_slice(&split_1);
}

pub fn uniform_crossover<T>(parent1: &mut Vec<T>, parent2: &mut Vec<T>)
    where T: Clone {
    let mut rng = rand::thread_rng();

    for i in 0usize .. parent1.len() {
        let fifty_fifty: bool = rng.gen::<bool>();
        if fifty_fifty {
            let value = parent1[i].clone();
            parent1[i] = parent2[i].clone();
            parent2[i] = value;
        }
    }
}

pub fn blx_crossover(parent1: &mut Vec<f64>, parent2: &mut Vec<f64>) {
    let mut rng = rand::thread_rng();

    let alpha = 0.5;

    for i in 0usize .. parent1.len() {
        let d = (parent1[i] - parent2[i]).abs();

        let a = helpers::minf(parent1[i],parent2[i]) - alpha * d;
        let b = helpers::maxf(parent1[i],parent2[i]) + alpha * d;

        if a != b {
            parent1[i] = rand::thread_rng().gen_range(a,b);
            parent2[i] = rand::thread_rng().gen_range(a,b);
        } else {
            parent1[i] = a;
            parent2[i] = a;
        }

    }

}
