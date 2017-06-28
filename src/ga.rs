#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_upper_case_globals)]

use population::Population;

use ::helpers;
use ::crossover;
use ::fitness;

use std::fs::OpenOptions;
use std::io::prelude::*;

static num_of_intervals: u64 = 10;

pub fn int_loop(
    fit_function: fn(&Vec<i64>) -> f64,
    mut_function: fn(&mut Population<i64>, usize, usize),
    cross_function: fn(&mut Vec<i64>, &mut Vec<i64>),
    select_function: fn(&mut Population<i64>) -> usize,
    perm: bool,
    ) {
    let num_of_executions = 10usize;
    let generations: usize = 10000;

    let mut best_vec: Vec<f64> = vec![0.0;generations];
    let mut avg_vec: Vec<f64> = vec![0.0;generations];
    let mut var_vec: Vec<f32> = vec![0.0;generations];

    let mut best_of_gens: Vec<f64> = vec![0.0;num_of_executions];
    let mut opt_of_gens: Vec<u64> = vec![0;num_of_executions];

    for j in 0 .. num_of_executions {
        let mut pop = Population::<i64>::new(fit_function,mut_function,cross_function,select_function);
        pop.initialize(perm);
        pop.evaluate_all();
        pop.print();

        let interval = pop.num_of_generations / num_of_intervals;
        let mut found_opt = false;
        let mut opt_gen: u64;

        println!("\n");
        for i in 0 .. pop.num_of_generations {
            pop.evolve_population();
            // pop.write_statistics(i);
            // pop.genetic_variability(i);
            best_vec[i as usize] += pop.fit_array[pop.get_best_individual()] * (1.0 / num_of_executions as f64);
            avg_vec[i as usize] += pop.get_average_fitness() * (1.0 / num_of_executions as f64);
            var_vec[i as usize] += pop.genetic_variability(i) as f32 * (1.0 / num_of_executions as f32);
            if i % interval == 0 {
                pop.print();
                println!("");
            }
            if pop.fit_array[0] >= 1.0 && found_opt == false {
                found_opt = true;
                opt_of_gens[j] = i;
            }
        }
        best_of_gens[j] = pop.fit_array[0];
        println!("\n");
    }
    println!("Best of each execution:");
    for i in 0 .. num_of_executions {
        println!("{}", best_of_gens[i]);
    }

    println!("Iteration that the optimum was found at each execution:");
    for i in 0 .. num_of_executions {
        println!("{}", opt_of_gens[i]);
    }

    let mut variability_file =
        OpenOptions::new()
        .write(true)
        .append(true)
        .open("variability.log")
        .unwrap();

    let mut best_file =
        OpenOptions::new()
        .write(true)
        .append(true)
        .open("best.log")
        .unwrap();

    let mut average_file =
        OpenOptions::new()
        .write(true)
        .append(true)
        .open("average.log")
        .unwrap();

    for i in 0 .. generations {
        if let Err(e) = writeln!(best_file, "{}\t{}",i,best_vec[i]) {
            println!("{}",e);
        }

        if let Err(e) = writeln!(average_file, "{}\t{}",i,avg_vec[i]) {
            println!("{}",e);
        }

        if let Err(e) = writeln!(variability_file, "{}\t{}",i,var_vec[i]) {
            println!("{}",e);
        }
    }
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
        // pop.write_statistics(i);
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
    let num_of_executions = 10usize;
    let generations: usize = 1;

    let mut best_vec: Vec<f64> = vec![0.0;generations];
    let mut avg_vec: Vec<f64> = vec![0.0;generations];
    let mut var_vec: Vec<f32> = vec![0.0;generations];

    let mut best_of_gens: Vec<f64> = vec![0.0;num_of_executions];
    let mut opt_of_gens: Vec<u64> = vec![0;num_of_executions];

    for j in 0 .. num_of_executions {
        let mut pop = Population::<bool>::new(fit_function,mut_function,cross_function,select_function);
        pop.initialize();
        pop.evaluate_all();
        pop.print();

        let interval = pop.num_of_generations / num_of_intervals;
        let mut found_opt = false;
        let mut opt_gen: u64;

        println!("\n");
        for i in 0 .. pop.num_of_generations {
            pop.evolve_population();
            // pop.write_statistics(i);
            // pop.genetic_variability(i);
            best_vec[i as usize] += pop.fit_array[pop.get_best_individual()] * (1.0 / num_of_executions as f64);
            avg_vec[i as usize] += pop.get_average_fitness() * (1.0 / num_of_executions as f64);
            var_vec[i as usize] += pop.genetic_variability(i) as f32 * (1.0 / num_of_executions as f32);
            if i % interval == 0 {
                pop.print();
                println!("");
            }
            if pop.fit_array[0] >= 1.0 && found_opt == false {
                found_opt = true;
                opt_of_gens[j] = i;
            }
        }
        best_of_gens[j] = pop.fit_array[0];
        println!("\n");
    }
    println!("Best of each execution:");
    for i in 0 .. num_of_executions {
        println!("{}", best_of_gens[i]);
    }

    println!("Iteration that the optimum was found at each execution:");
    for i in 0 .. num_of_executions {
        println!("{}", opt_of_gens[i]);
    }

    let mut variability_file =
        OpenOptions::new()
        .write(true)
        .append(true)
        .open("variability.log")
        .unwrap();

    let mut best_file =
        OpenOptions::new()
        .write(true)
        .append(true)
        .open("best.log")
        .unwrap();

    let mut average_file =
        OpenOptions::new()
        .write(true)
        .append(true)
        .open("average.log")
        .unwrap();

    for i in 0 .. generations {
        if let Err(e) = writeln!(best_file, "{}\t{}",i,best_vec[i]) {
            println!("{}",e);
        }

        if let Err(e) = writeln!(average_file, "{}\t{}",i,avg_vec[i]) {
            println!("{}",e);
        }

        if let Err(e) = writeln!(variability_file, "{}\t{}",i,var_vec[i]) {
            println!("{}",e);
        }
    }
}

pub fn testing () {
    let lul: Vec<bool> = vec![false;64];
    let fit = fitness::deceptive_n(&lul);
    println!("{}",fit);
}
