extern crate rand;

use rand::Rng;
use rand::distributions::{IndependentSample, Range};
use std::env;
use std::process;

#[derive(Debug, Clone)]
struct Population<T> {
    pop_size: u64,
    ind_size: u64,
    individuals: Vec<T>,
    lower_bound: f64,
    upper_bound: f64,
}

#[allow(dead_code)]
impl<T> Population<T> {

    fn new(pop_size_: u64, ind_size_: u64, lb: f64, ub: f64) -> Population<T> {
        Population::<T> {
            pop_size: pop_size_,
            ind_size: ind_size_,
            lower_bound: lb,
            upper_bound: ub,
            individuals: Vec::new(),
        }
    }

    fn access(&self, i: u64, j: u64) -> &T {
        &self.individuals[(i + (j * self.ind_size)) as usize]
    }

    fn set(&mut self, i: u64, j: u64, value: T) {
        self.individuals[(i + (j * self.ind_size)) as usize] = value;
    }
}

#[allow(dead_code)]
impl Population<i64> {

    fn initialize(&mut self) {
        self.individuals = vec![0; (self.pop_size * self.ind_size) as usize];
        let between = Range::new(self.lower_bound.floor() as i64, self.upper_bound.floor() as i64);
        let mut rng = rand::thread_rng();

        for i in 0..self.pop_size {
            for j in 0..self.ind_size {
                let value: i64 = between.ind_sample(&mut rng);
                self.set(i, j, value);
            }
        }
    }

    fn print(&self) {
        for i in 0..self.pop_size {
            for j in 0..self.ind_size {
                print!("{} ", self.access(i, j));
            }
            println!("");
        }
    }
}

#[allow(dead_code)]
impl Population<f64> {
    fn initialize(&mut self) {
        self.individuals = vec![0.0; (self.pop_size * self.ind_size) as usize];
        let between = Range::new(self.lower_bound, self.upper_bound);
        let mut rng = rand::thread_rng();
        for i in 0..self.pop_size {
            for j in 0..self.ind_size {
                let value: f64 = between.ind_sample(&mut rng);
                self.set(i, j, value);
            }
        }
    }

    fn print(&self) {
        for i in 0..self.pop_size {
            for j in 0..self.ind_size {
                print!("{} ", self.access(i, j));
            }
            println!("");
        }
    }
}

#[allow(dead_code)]
impl Population<bool> {
    fn initialize(&mut self) {
        self.individuals = vec![false; (self.pop_size * self.ind_size) as usize];
        let mut rng = rand::thread_rng();
        for i in 0..self.pop_size {
            for j in 0..self.ind_size {
                let value: bool = rng.gen::<bool>();
                self.set(i, j, value);
            }
        }
    }

    fn print(&self) {
        for i in 0..self.pop_size {
            for j in 0..self.ind_size {
                print!("{} ", self.access(i, j));
            }
            println!("");
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();

    match args.len() {
        1 ... 3 => {
            println!("Too few arguments!");
            println!("Usage: cargo run [options] -- (type of individual: float, int or bin)
                     (size of the population) (size of each individual)
                     (range of the domain, for int or float types)");
            process::exit(0);

        }
        4 | 6 => {
        }
        5 => {
            println!("You have to pass both the lower bound and the upper bound");
            process::exit(0);
        }
        _ => {
            println!("Passed too many arguments!");
            process::exit(0);
        }
    }

    let pop_size_: u64 = args[2].trim().parse()
        .expect("Not a valid number");

    let ind_size_: u64 = args[3].trim().parse()
        .expect("Not a valid number");

    let l_bound_: f64 = if args.len() == 6 {
        args[4].trim().parse()
            .expect("lul")
    } else { 0.0 as f64 };

    let u_bound_: f64 = if args.len() == 6 {
        args[5].trim().parse()
            .expect("lul")
    } else { 0.0 as f64 };

    // let mut population = Population::<i64>::new(pop_size_,ind_size_,l_bound_,u_bound_);
    // let mut population = Population::<f64>::new(pop_size_,ind_size_,l_bound_,u_bound_);
    let mut population = Population::<bool>::new(pop_size_,ind_size_,l_bound_,u_bound_);

    population.initialize();
    population.print();

    // let mut population = match args[1].to_uppercase().as_ref() {
    //     "INT"   => Population::<i64>::new(pop_size_,ind_size_,l_bound_,u_bound_),
    //     "FLOAT" => Population::<f64>::new(pop_size_,ind_size_,l_bound_,u_bound_),
    //       _     => Population::<bool>::new(pop_size_,ind_size_,l_bound_,u_bound_),
    // };
}
