use population::Population;

use ::fitness;

pub fn int_loop(p_size: u64, i_size: u64, lb: f64, ub: f64) {
    let mut pop = Population::<i64>::new(p_size,i_size,lb,ub);
    pop.initialize();
    pop.evaluate_all(&fitness::int_parity_alternate);
    pop.print();
}

pub fn float_loop(p_size: u64, i_size: u64, lb: f64, ub: f64) {
    let mut pop = Population::<f64>::new(p_size,i_size,lb,ub);
    pop.initialize();
    pop.evaluate_all(&fitness::float_quadratic_min);
    pop.print();
}

pub fn bin_loop(p_size: u64, i_size: u64) {
    let mut pop = Population::<bool>::new(p_size,i_size,0.0,0.0);
    pop.initialize();
    // pop.evaluate_all(&fitness::binary_alternate);
    // pop.evaluate_all(&fitness::bin_parps_function);
    pop.evaluate_all(&fitness::bin_radio_factory);
    pop.print();
}
