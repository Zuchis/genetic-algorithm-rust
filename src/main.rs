extern crate rand;

// use rand::Rng;
use rand::distributions::{IndependentSample, Range};
use std::env;
use std::process;

#[derive(Debug, Clone)]
struct Population<T> {
    pop_size: u64,
    ind_size: u64,
    individuals: Vec<T>,
    lower_bound: T,
    upper_bound: T,
}

#[allow(dead_code)]
impl<T> Population<T> {
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
        let between = Range::new(self.lower_bound, self.upper_bound);
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
impl Population<u8> {
    fn initialize(&mut self) {
        let between = Range::new(self.lower_bound, self.upper_bound);
        let mut rng = rand::thread_rng();
        for i in 0..self.pop_size {
            for j in 0..self.ind_size {
                let value: u8 = between.ind_sample(&mut rng);
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
        4 | 6 => { // If you got in here, you are on the right neighborhood m8
            let pop_size_: u64 = args[2].trim().parse()
                .expect("Not a valid number");

            let ind_size_: u64 = args[3].trim().parse()
                .expect("Not a valid number");

            match args[1].to_uppercase().as_ref() {
                "INT" => {
                    let l_bound: i64 = args[4].trim().parse()
                        .expect("Not a valid number");
                    let u_bound: i64 = args[5].trim().parse()
                        .expect("Not a valid number");

                    let mut population: Population<i64> = Population {pop_size: pop_size_, ind_size: ind_size_, individuals: vec![0; (pop_size_ * ind_size_) as usize], lower_bound: l_bound, upper_bound: u_bound};
                    population.initialize();
                    population.print();
                }
                "FLOAT" => {
                    let l_bound: f64 = args[4].trim().parse()
                        .expect("Not a valid number");
                    let u_bound: f64 = args[5].trim().parse()
                        .expect("Not a valid number");
                    let mut population: Population<f64> = Population {pop_size: pop_size_, ind_size: ind_size_, individuals: vec![0.0; (pop_size_ * ind_size_) as usize], lower_bound: l_bound, upper_bound: u_bound};
                    population.initialize();
                    population.print();
                }
                "BIN" => {
                    let mut population: Population<u8> = Population {pop_size: pop_size_, ind_size: ind_size_, individuals: vec![0; (pop_size_ * ind_size_) as usize], lower_bound: 0, upper_bound: 2};
                    population.initialize();
                    population.print();
                }
                _ => {
                    println!("Not a valid gene type");
                    process::exit(0);
                }

            }
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
}
