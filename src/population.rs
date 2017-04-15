#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

extern crate rand;

use population::rand::Rng;
use population::rand::distributions::{IndependentSample, Range};
use population::rand::distributions::normal::StandardNormal;

use std::process;

use ::fitness::HasFitness;
use ::helpers;
use ::crossover;
use ::selection;

use helpers::SimpleStepRange;

// #[derive(Debug, Clone)]
pub struct Population<T>
    where T: Clone {
    pub pop_size: u64,
    pub ind_size: u64,
    pub individuals: Vec<Vec<T>>,
    pub fit_array: Vec<f64>,
    pub lower_bound: f64,
    pub upper_bound: f64,
    pub cross_chance: f64,
    pub mut_chance: f64,
    pub has_elitism: bool,
    pub num_of_generations: u64,
    pub tournament_size: u64,
    pub marked_for_evaluation: Vec<bool>,
    pub crossover_function: fn(&mut Vec<T>, &mut Vec<T>),
    pub selection_function: fn(&mut Population<T>) -> usize,
    pub fitness_function: fn(&Vec<T>) -> f64,
    pub mutation_function: fn(&mut Population<T>, usize, usize)
}

#[allow(dead_code)]
impl<T> Population<T>
    where T: Clone {

    pub fn new(fit_function: fn(&Vec<T>) -> f64, mut_function: fn(&mut Population<T>, usize, usize)) -> Population<T> {
        let args: Vec<String> = helpers::parse_arguments();


        let p_size: u64 = args[1].trim().parse()
            .expect("Not a valid number");

        let i_size: u64 = args[2].trim().parse()
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


        let cross_function = match args[10].to_uppercase().as_ref() {
            "ONE_POINT_CROSSOVER" => crossover::one_point_crossover,
            _ => {
                println!("Not a valid crossover function");
                process::exit(1);
            }
        };

        let select_function = match args[8].to_uppercase().as_ref() {
            "WHEEL" => selection::wheel,
            "TOURNAMENT" => selection::tournament,
            _ => {
                println!("Not a valid selection function");
                process::exit(1);
            }
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
            marked_for_evaluation: vec![true; p_size as usize],
            tournament_size: t_size,
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

        for i in SimpleStepRange(0usize, self.pop_size as usize, 2) {
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

        self.individuals.clone_from(&new_pop);
        self.fit_array.clone_from(&new_fitnesses);

        self.mutate_all();

        if self.has_elitism {
            self.individuals[0] = best_individual;
            self.fit_array[0] = best_fitness;
        }
        self.evaluate_all();
    }
}


#[allow(dead_code)]
impl Population<i64> {

    pub fn initialize(&mut self) {
        for _ in 0..self.pop_size {
            self.individuals.push(vec![0; self.ind_size as usize]);
        }
        let between = Range::new(self.lower_bound.floor() as i64, self.upper_bound.floor() as i64);
        let mut rng = rand::thread_rng();

        for i in 0..self.pop_size {
            for j in 0..self.ind_size {
                let value: i64 = between.ind_sample(&mut rng);
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
