extern crate rand;

use population::rand::Rng;
use population::rand::distributions::{IndependentSample, Range};

#[derive(Debug, Clone)]
pub struct Population<T> {
    pub pop_size: u64,
    pub ind_size: u64,
    pub individuals: Vec<Vec<T>>,
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

    pub fn evaluate_individual(&mut self, ind: u64, f: &Fn(&Vec<T>) -> f64) {
        self.fit_array[ind as usize] = f(&self.individuals[ind as usize]);
    }

    pub fn evaluate_all(&mut self, f: &Fn(&Vec<T>) -> f64) {
        for i in 0 .. self.pop_size {
            self.fit_array[i as usize] = f(&self.individuals[i as usize]);
        }
    }

    pub fn wheel(&mut self) -> &Vec<T> {
        let sum: f64 = self.fit_array.iter().fold(0.0, |a, &b| a + b);
        let roulete_position = Range::new(0.0, sum).ind_sample(&mut rand::thread_rng());
        let mut accumulator: f64 = 0.0;
        let mut chosen: usize = 0;
        for i in 0..self.fit_array.len() {
            accumulator += self.fit_array[i as usize];
            if accumulator >= roulete_position {
                chosen = i as usize;
                break;
            }
        }
        &self.individuals[chosen]
    }

    pub fn crossover(&mut self, parent1: usize, parent2: usize)
    where T: Clone {
        let cut_position: usize = Range::new(0usize, (self.ind_size - 1) as usize).ind_sample(&mut rand::thread_rng());
        let split_1 = self.individuals[parent1].split_off(cut_position);
        let split_2 = self.individuals[parent2].split_off(cut_position);

        self.individuals[parent1].extend_from_slice(&split_2);
        self.individuals[parent2].extend_from_slice(&split_1);
    }

    // pub fn mutate_individual(&mut self, ind: usize) {
    // }
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
