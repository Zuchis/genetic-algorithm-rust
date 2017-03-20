use population::Population;

#[allow(dead_code)]
pub fn binary_alternate (pop: &Population<bool>, ind: u64) -> f64 {
    let index = ind * pop.ind_size;
    let mut fit: f64 = 0.0;
    for j in 0 .. (pop.ind_size - 1) {
        if pop.individuals[(index + j) as usize] != pop.individuals[(index + j + 1) as usize]{
            fit = fit + 1.0;
        }
    }
    fit
}

#[allow(dead_code)]
pub fn int_parity_alternate (pop: &Population<i64>, ind: u64) -> f64 {
    let index = ind * pop.ind_size;
    let mut fit: f64 = 0.0;
    for j in 0 .. (pop.ind_size - 1) {
        let current: usize = (index + j) as usize;
        let next: usize = (index + j + 1) as usize;
        if ((pop.individuals[current] % 2 == 0) && (pop.individuals[next] % 2 != 0)) || ((pop.individuals[current] % 2 != 0) && (pop.individuals[next] % 2 == 0)){
            fit = fit + 1.0;
        }
    }
    fit
}

#[allow(dead_code)]
pub fn float_quadratic_min (pop: &Population<f64>, ind: u64) -> f64 {
    let index = ind * pop.ind_size;
    let mut fit: f64 = 0.0;
    for j in 0 .. (pop.ind_size - 1) {
        let value = pop.individuals[(index + j) as usize];
        fit = fit + (value * value);
    }
    1.0 / fit
}
