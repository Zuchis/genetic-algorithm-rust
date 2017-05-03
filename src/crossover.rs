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
use std::cmp;

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

pub fn pmx_crossover(parent1: &mut Vec<i64>, parent2: &mut Vec<i64>) {
    let interval = Range::new(1usize, (parent1.len() -1));
    let mut rng = rand::thread_rng();
    let cut_position1: usize = interval.ind_sample(&mut rng);
    let mut cut_position2: usize;
    loop {
        cut_position2 = interval.ind_sample(&mut rng);
        if cut_position2 != cut_position1 {
            break;
        }
    }
    let beg = cmp::min(cut_position1,cut_position2);
    let end = cmp::max(cut_position1,cut_position2);
    let mut slice1: Vec<i64> = Vec::new();
    let mut slice2: Vec<i64> = Vec::new();

    let mut matched_section_index: usize = 0;

    for i in beg .. end {
        slice1.push(parent1[i].clone());
        slice2.push(parent2[i].clone());

        parent1[i] = slice2[matched_section_index].clone();
        parent2[i] = slice1[matched_section_index].clone();
        matched_section_index += 1;
    }

    let mut slice_end: usize = slice1.len();
    let mut i: usize = 0;

    while i != slice_end {
        let (is_in, position) = helpers::is_in(&slice1[i],&slice2);
        if is_in {
            slice2.remove(position);
            slice1.remove(i);
            slice_end -= 1;
        } else {
            i += 1;
        }
    }

    for i in 0usize .. beg {
        let (is_in, position) = helpers::is_in(&parent1[i],&slice2);
        if is_in {
            parent1[i] = slice1[position].clone();
        }
        let (is_in2,position2) = helpers::is_in(&parent2[i],&slice1);
        if is_in2 {
            parent2[i] = slice2[position2].clone();
        }
    }

    for i in end .. parent1.len() {
        let (is_in, position) = helpers::is_in(&parent1[i],&slice2);
        if is_in {
            parent1[i] = slice1[position].clone();
        }
        let (is_in2,position2) = helpers::is_in(&parent2[i],&slice1);
        if is_in2 {
            parent2[i] = slice2[position2].clone();
        }
    }
    // println!("The chosen cut points were:{} {}",beg,end);
    // if helpers::has_repeated_elements(parent1) || helpers::has_repeated_elements(parent2) {
    //     helpers::print_vector(parent1);
    //     helpers::print_vector(parent2);
    //     panic!("There are repeated elements!");
    // }
    // println!("");
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
