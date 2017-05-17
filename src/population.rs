#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

extern crate rand;
extern crate num;

use population::rand::Rng;
use population::rand::distributions::{IndependentSample, Range};
use population::rand::distributions::normal::StandardNormal;
use population::num::Float;

use std::process;
use std::fs::OpenOptions;
use std::io::prelude::*;

use ::fitness::HasFitness;
use ::helpers;
use ::crossover;
use ::selection;

use helpers::SimpleStepRange;

// #[derive(Debug, Clone)]
pub struct Population<T>
    where T: Clone {
    pub pop_size: usize,
    pub ind_size: usize,
    pub individuals: Vec<Vec<T>>,
    pub fit_array: Vec<f64>,
    pub lower_bound: f64,
    pub upper_bound: f64,
    pub cross_chance: f64,
    pub mut_chance: f64,
    pub has_elitism: bool,
    pub num_of_generations: u64,
    pub current_generation: u64,
    pub tournament_size: u64,
    pub marked_for_evaluation: Vec<bool>,
    pub linear_scaling: bool,
    pub generation_gap: bool,
    pub crossover_function: fn(&mut Vec<T>, &mut Vec<T>),
    pub selection_function: fn(&mut Population<T>) -> usize,
    pub fitness_function: fn(&Vec<T>) -> f64,
    pub mutation_function: fn(&mut Population<T>, usize, usize)
}

#[allow(dead_code)]
impl<T> Population<T>
    where T: Clone {

    pub fn new(
        fit_function: fn(&Vec<T>) -> f64,
        mut_function: fn(&mut Population<T>, usize, usize),
        cross_function: fn(&mut Vec<T>, &mut Vec<T>),
        select_function: fn(&mut Population<T>) -> usize,
    ) -> Population<T> {
        let args: Vec<String> = helpers::parse_arguments();


        let p_size: usize = args[1].trim().parse()
            .expect("Not a valid number");

        let i_size: usize = args[2].trim().parse()
            .expect("Not a valid number");

        let lb: f64 = args[3].trim().parse()
            .expect("lul");

        let ub: f64 = args[4].trim().parse()
            .expect("lul");

        let c_chance: f64 = args[5].trim().parse()
            .expect("not a valid number");

        let m_chance: f64 = args[6].trim().parse()
            .expect("not a valid number");

        let n_gens: u64 = args[7].trim().parse()
            .expect("not a valid number");

        let t_size: u64 = args[9].trim().parse()
            .expect("not a valid number");

        let elitism_option = match args[13].to_uppercase().as_ref() {
            "TRUE" => true,
            _ => false,
        };

        let linear_scaling_option = match args[14].to_uppercase().as_ref() {
            "TRUE" => true,
            _ => false,
        };

        let generation_gap_option = match args[15].to_uppercase().as_ref() {
            "TRUE" => true,
            _ => false,
        };

        let pop: Population<T> = Population::<T> {
            pop_size: p_size,
            ind_size: i_size,
            individuals: Vec::new(),
            fit_array: vec![0.0; p_size as usize],
            lower_bound: lb,
            upper_bound: ub,
            cross_chance: c_chance,
            mut_chance: m_chance,
            has_elitism: elitism_option,
            num_of_generations: n_gens,
            current_generation: 0,
            marked_for_evaluation: vec![true; p_size as usize],
            tournament_size: t_size,
            linear_scaling: linear_scaling_option,
            generation_gap: generation_gap_option,
            crossover_function: cross_function,
            selection_function: select_function,
            fitness_function: fit_function,
            mutation_function: mut_function,
        };
        pop
    }

    pub fn evaluate_individual(&mut self, ind: usize) {
        self.fit_array[ind] = self.individuals[ind].fitness(&self.fitness_function);
        self.marked_for_evaluation[ind] = false;
    }

    pub fn evaluate_all(&mut self) {
        for i in 0usize .. (self.pop_size as usize) {
            // if self.marked_for_evaluation[i] == true {
                self.fit_array[i] = self.individuals[i].fitness(&self.fitness_function);
                self.marked_for_evaluation[i] = false;
            // }
        }
    }

    pub fn mutate_all(&mut self) {
        let range = Range::new(0.0, 100.0);
        let mut rng = rand::thread_rng();

        for i in 0usize .. self.pop_size as usize {
            for j in 0usize..self.ind_size as usize {
                let chance = range.ind_sample(&mut rng);
                if chance <= self.mut_chance {
                    (&self.mutation_function)(self,i, j);
                    self.marked_for_evaluation[i] = true;
                }
            }
        }
    }

    pub fn get_best_individual(&self) -> usize {
        let mut best_index: usize = 0;

        for i in 1usize .. self.pop_size as usize {
            if self.fit_array[i] > self.fit_array[best_index] {
                best_index = i;
            }
        }

        best_index
    }

    pub fn get_average_fitness(&self) -> f64 {
        let sum: f64 = self.fit_array.iter().fold(0.0, |a, &b| a + b);
        sum / self.pop_size as f64
    }

    pub fn get_best_and_worst_individual(&self) -> (usize,usize) {
        let mut best_index: usize = 0;
        let mut worst_index: usize = 0;

        for i in 1usize .. self.pop_size as usize {
            if self.fit_array[i] > self.fit_array[best_index] {
                best_index = i;
            }
            if self.fit_array[i] < self.fit_array[worst_index] {
                worst_index = i;
            }

        }
        (best_index,worst_index)
    }

    pub fn fitness_scaling(&self) -> Vec<f64> {
       let (best,worst) = self.get_best_and_worst_individual();
        let fmin = self.fit_array[worst];
        let fmax = self.fit_array[best];
        let favg = self.fit_array.iter().fold(0.0, |a, &b| a + b) / self.fit_array.len() as f64;
        let c: f64 = 1.2 * (2.0 / 1.2).powf(self.current_generation as f64 / self.num_of_generations as f64);
        let alpha: f64;
        let beta: f64;
        let mut scaled_fitness_array: Vec<f64> = Vec::new();
        match fmin > (c * favg - fmax) / (c - 1.0) {
            true => {
                alpha = (favg * (c - 1.0)) / (fmax - favg);
                beta = (favg * (fmax - c * favg)) / (fmax - favg);
            }
            false => {
                alpha = favg / (favg - fmin);
                beta = (-fmin * favg) / (favg - fmin);
            }
        }
        for i in 0usize .. self.fit_array.len() {
            scaled_fitness_array.push(self.fit_array[i] * alpha + beta);
        }
        scaled_fitness_array
    }

    pub fn evolve_population(&mut self) {
        // let (best_ind_index, worst_ind_index) = self.get_best_and_worst_individual();
        let best_ind_index = self.get_best_individual();
        let best_individual = self.individuals[best_ind_index].clone();
        let best_fitness = self.fit_array[best_ind_index];
        let mut new_pop: Vec<Vec<T>> = Vec::new();
        let mut new_fitnesses: Vec<f64> = Vec::new();
        let mut last_chosen: usize;
        let mut new_chosen: usize;
        let between = Range::new(0.0, 100.0);
        let mut rng = rand::thread_rng();
        // new_fitnesses.push(self.fit_array[last_chosen]);

        let mut new_percentage = 0.4;

        let new_range = if self.generation_gap == true {(new_percentage * self.pop_size as f32).floor() as usize} else {self.pop_size};
        if new_range % 2 != 0 {
            new_range = new_range - 1;
        }

        for i in SimpleStepRange(0usize, new_range, 2) {
            last_chosen = (&self.selection_function)(self);
            loop {
                new_chosen = (&self.selection_function)(self);
                if new_chosen != last_chosen {
                    break;
                }
            }

            let mut parent1 = self.individuals[last_chosen].clone();
            let mut parent2 = self.individuals[new_chosen].clone();

            let prob = between.ind_sample(&mut rng);
            // Check if the crossover is going to happen
            if prob <= self.cross_chance {
                (&self.crossover_function)(&mut parent1, &mut parent2);
                self.marked_for_evaluation[i] = true;
                self.marked_for_evaluation[i + 1usize] = true;
            }
            new_pop.push(parent1);
            new_fitnesses.push(self.fit_array[last_chosen]);
            new_pop.push(parent2);
            new_fitnesses.push(self.fit_array[new_chosen]);
        }

        if self.generation_gap == true {
            self.individuals.clone_from(&new_pop);
            self.fit_array.clone_from(&new_fitnesses);
        } else {
            let pos_range = Range::new(0usize, self.pop_size);
            for i in 0usize .. new_range {
                let random_pos = pos_range.ind_sample(&mut rng);
                self.individuals[random_pos] = new_pop.pop().unwrap();
            }
        }

        self.mutate_all();

        if self.has_elitism {
            self.individuals[0] = best_individual;
            self.fit_array[0] = best_fitness;
        }
        self.evaluate_all();
    }

    pub fn write_statistics(&self, iter: u64) {
        let best: usize = self.get_best_individual();
        let best_fitness: f64 = self.fit_array[best];
        let average_fitness: f64 = self.get_average_fitness();
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

        if let Err(e) = writeln!(best_file, "{}\t{}",iter,best_fitness) {
            println!("{}",e);
        }

        if let Err(e) = writeln!(average_file, "{}\t{}",iter,average_fitness) {
            println!("{}",e);
        }
    }
}


#[allow(dead_code)]
impl Population<i64> {

    pub fn initialize(&mut self, perm: bool) {
        for _ in 0..self.pop_size {
            self.individuals.push(vec![0; self.ind_size as usize]);
        }
        let between = Range::new(self.lower_bound.floor() as i64, self.upper_bound.floor() as i64);
        let mut rng = rand::thread_rng();

        if perm == true {
            for i in 0usize..self.pop_size as usize {
                for j in 0usize..self.ind_size as usize {
                    let value: i64 = between.ind_sample(&mut rng);
                    self.individuals[i][j] = value;
                }
            }
        } else {
            for i in 0usize..self.pop_size as usize {
                for j in 0usize..self.ind_size as usize {
                    self.individuals[i][j] = j as i64;
                }
                rng.shuffle(&mut self.individuals[i]);
            }
        }
    }

    pub fn print(&self) {
        for i in 0..self.pop_size {
            for j in 0..self.ind_size {
                print!("{} ", self.individuals[i as usize][j as usize]);
            }
            println!("");
        }

        println!["Population's fitness:"];
        for i in 0 .. self.pop_size {
            print!(" {} ",self.fit_array[i as usize]);
        }
    }

    pub fn genetic_variability(&self, iter: u64) {
        let mut variability: i64 = 0;
        for i in 0usize .. ((self.ind_size - 1) as usize) {
            for j in i .. self.ind_size as usize {
                variability = variability + helpers::euclidean_distance(&self.individuals[i],&self.individuals[j]);
            }
        }

        let mut variability_file =
            OpenOptions::new()
            .write(true)
            .append(true)
            .open("variability.log")
            .unwrap();

        if let Err(e) = writeln!(variability_file, "{}\t{}",iter,variability) {
            println!("{}",e);
        }
    }

}

#[allow(dead_code)]
impl Population<f64> {
    pub fn initialize(&mut self) {
        for _ in 0..self.pop_size {
            self.individuals.push(vec![0.0; self.ind_size as usize]);
        }
        let between = Range::new(self.lower_bound, self.upper_bound);
        let mut rng = rand::thread_rng();

        for i in 0..self.pop_size {
            for j in 0..self.ind_size {
                let value: f64 = between.ind_sample(&mut rng);
                self.individuals[i as usize][j as usize] = value;
            }
        }
    }

    pub fn print(&self) {
        for i in 0..self.pop_size {
            for j in 0..self.ind_size {
                print!("{} ", self.individuals[i as usize][j as usize]);
            }
            println!("");
        }
        println!["Population's fitness:"];
        for i in 0 .. self.pop_size {
            print!(" {} ",self.fit_array[i as usize]);
        }
    }

}


#[allow(dead_code)]
impl Population<bool> {
    pub fn initialize(&mut self) {
        for _ in 0..self.pop_size {
            self.individuals.push(vec![false; self.ind_size as usize]);
        }
        let mut rng = rand::thread_rng();
        for i in 0..self.pop_size {
            for j in 0..self.ind_size {
                let value: bool = rng.gen::<bool>();
                self.individuals[i as usize][j as usize] = value;
            }
        }
    }

    pub fn print(&self) {
        for i in 0..self.pop_size {
            for j in 0..self.ind_size {
                print!("{} ", self.individuals[i as usize][j as usize]);
            }
            println!("");
        }

        println!["Population's fitness:"];
        for i in 0 .. self.pop_size {
            print!(" {} ",self.fit_array[i as usize]);
        }
    }
}
