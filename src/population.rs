extern crate rand;

use population::rand::Rng;
use population::rand::distributions::{IndependentSample, Range};

#[derive(Debug, Clone)]
pub struct Population<T> {
    pub pop_size: u64,
    pub ind_size: u64,
    pub individuals: Vec<T>,
    pub fit_array: Vec<f64>,
    pub lower_bound: f64,
    pub upper_bound: f64,
}

#[allow(dead_code)]
impl<T> Population<T> {

    pub fn new(pop_size_: u64, ind_size_: u64, lb: f64, ub: f64) -> Population<T> {
        Population::<T> {
            pop_size: pop_size_,
            ind_size: ind_size_,
            lower_bound: lb,
            upper_bound: ub,
            individuals: Vec::new(),
            fit_array: vec![0.0; pop_size_ as usize],
        }
    }

    pub fn access(&self, i: u64, j: u64) -> &T {
        &self.individuals[((i * self.ind_size) + j) as usize]
    }

    pub fn set(&mut self, i: u64, j: u64, value: T) {
        self.individuals[((i * self.ind_size) + j) as usize] = value;
    }

    pub fn evaluate_individual(&mut self, ind: u64, f: &Fn(&Population<T>,u64) -> f64) {
        self.fit_array[ind as usize] = f(&self, ind);
    }

    pub fn evaluate_all(&mut self, f: &Fn(&Population<T>,u64) -> f64) {
        for i in 0 .. self.pop_size {
            self.fit_array[i as usize] = f(&self,i);
        }
    }
}


#[allow(dead_code)]
impl Population<i64> {

    pub fn initialize(&mut self) {
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

    pub fn print(&self) {
        for i in 0..self.pop_size {
            for j in 0..self.ind_size {
                print!("{} ", self.access(i, j));
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

    pub fn print(&self) {
        for i in 0..self.pop_size {
            for j in 0..self.ind_size {
                print!("{} ", self.access(i, j));
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
        self.individuals = vec![false; (self.pop_size * self.ind_size) as usize];
        let mut rng = rand::thread_rng();
        for i in 0..self.pop_size {
            for j in 0..self.ind_size {
                let value: bool = rng.gen::<bool>();
                self.set(i, j, value);
            }
        }
    }

    pub fn print(&self) {
        for i in 0..self.pop_size {
            for j in 0..self.ind_size {
                print!("{} ", self.access(i, j));
            }
            println!("");
        }

        println!["Population's fitness:"];
        for i in 0 .. self.pop_size {
            print!(" {} ",self.fit_array[i as usize]);
        }
    }
}
