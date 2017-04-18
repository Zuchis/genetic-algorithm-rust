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
    ) {
    let mut pop = Population::<i64>::new(fit_function,mut_function,cross_function,select_function);
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
    // let args: Vec<String> = helpers::parse_arguments();
    // println!("Arguments given:");
    // for i in args {
    //     println!("{}",i);
    // }

    // println!("A display of the one-point crossover");

    // let mut vec1: Vec<i64> = [1,2,3,4,5].to_vec();
    // let mut vec2: Vec<i64> = [6,7,8,9,10].to_vec();
    // crossover::one_point_crossover(&mut vec1, &mut vec2);

    // helpers::print_vector(&vec1);
    // helpers::print_vector(&vec2);

    // println!("Testing the simple step range");
    // for i in helpers::SimpleStepRange(0,10,2) {
    //     print!("{} ", i);
    // }
    // println!("");

    let ind: Vec<bool> = [true,false,true].to_vec();
    let i: i64 = helpers::bin_to_int(&ind);
    println!("{}",i);
}
