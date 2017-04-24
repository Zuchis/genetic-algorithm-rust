#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_upper_case_globals)]

use population::Population;

use ::helpers;
use ::crossover;

static num_of_intervals: u64 = 10;

pub fn int_loop(
    fit_function: fn(&Vec<i64>) -> f64,
    mut_function: fn(&mut Population<i64>, usize, usize),
    cross_function: fn(&mut Vec<i64>, &mut Vec<i64>),
    select_function: fn(&mut Population<i64>) -> usize,
    perm: bool,
    ) {
    let mut pop = Population::<i64>::new(fit_function,mut_function,cross_function,select_function);
    pop.initialize(perm);
    pop.evaluate_all();
    pop.print();

    let interval = pop.num_of_generations / num_of_intervals;

    println!("\n");
    for i in 1 .. pop.num_of_generations {
        pop.evolve_population();
        pop.write_statistics(i);
        if i % interval == 0 {
            pop.print();
            println!("");
        }
    }
    pop.print();
}

pub fn float_loop(
    fit_function: fn(&Vec<f64>) -> f64,
    mut_function: fn(&mut Population<f64>, usize, usize),
    cross_function: fn(&mut Vec<f64>, &mut Vec<f64>),
    select_function: fn(&mut Population<f64>) -> usize,
    ) {
    let mut pop = Population::<f64>::new(fit_function,mut_function,cross_function,select_function);
    pop.initialize();
    pop.evaluate_all();
    pop.print();

    let interval = pop.num_of_generations / num_of_intervals;

    println!("\n");
    for i in 1 .. pop.num_of_generations {
        pop.evolve_population();
        pop.write_statistics(i);
        if i % interval == 0 {
            pop.print();
            println!("");
        }
    }
    pop.print();
}

pub fn bin_loop(
    fit_function: fn(&Vec<bool>) -> f64,
    mut_function: fn(&mut Population<bool>, usize, usize),
    cross_function: fn(&mut Vec<bool>, &mut Vec<bool>),
    select_function: fn(&mut Population<bool>) -> usize,
    ) {
    let mut pop = Population::<bool>::new(fit_function,mut_function,cross_function,select_function);
    pop.initialize();
    pop.evaluate_all();
    pop.print();

    let interval = pop.num_of_generations / num_of_intervals;

    println!("\n");
    for i in 1 .. pop.num_of_generations {
        pop.evolve_population();
        pop.write_statistics(i);
        if i % interval == 0 {
            pop.print();
            println!("");
        }
    }
    println!("\n");
}

pub fn testing () {
    let (is_in,pos) = helpers::is_in(&5,&[1,2,3,4,5].to_vec());
    if is_in {
        println!("the number 5 is in position {}",pos);
    }
    let mut vec1 = [1,2,3,4,5,6].to_vec();
    let mut vec2 = [2,5,3,4,1,6].to_vec();
    helpers::print_vector(&vec1);
    helpers::print_vector(&vec2);
    crossover::pmx_crossover(&mut vec1, &mut vec2);
    helpers::print_vector(&vec1);
    helpers::print_vector(&vec2);
}
