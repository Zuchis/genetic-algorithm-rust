#![allow(unused_variables)]

mod ga;
mod population;
mod fitness;
mod helpers;
mod crossover;
mod mutation;
mod selection;

use std::process;
use std::fs::File;

fn main() {
    let args: Vec<String> = helpers::parse_arguments();

    let f1 = File::create("best.log");
    let f2 = File::create("average.log");

    match args[0].to_uppercase().as_ref() {
        "INT" | "INT_PERM"   =>  {
            let fit_function = match args[11].to_uppercase().as_ref() {
                "INT_PARITY_ALTERNATE" => fitness::int_parity_alternate,
                "N_QUEENS" => fitness::n_queens,
                "LABIRINTH_MINIMUM_PATH" => fitness::labirinth_minimum_path,
                _          => {
                    println!("Not a valid fitness function");
                    process::exit(1);
                }
            };
            let mut_function = match args[12].to_uppercase().as_ref() {
                "INT_DELTA_MUTATION" => mutation::int_delta_mutation,
                "SWAP_POSITION"      => mutation::swap_position,
                _              => {
                    println!("Not a valid mutation function");
                    process::exit(1);
                }
            };

            let cross_function = match args[10].to_uppercase().as_ref() {
                "ONE_POINT_CROSSOVER" => crossover::one_point_crossover,
                "UNIFORM_CROSSOVER" => crossover::uniform_crossover,
                "PMX_CROSSOVER" => crossover::pmx_crossover,
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
            let perm = match args[0].to_uppercase().as_ref() {
                "INT" => true,
                  _   => false,
            };
            ga::int_loop(fit_function,mut_function,cross_function,select_function,perm);
        }
        "FLOAT" =>  {
            let fit_function = match args[11].to_uppercase().as_ref() {
                "FLOAT_QUADRATIC_MIN" => fitness::float_quadratic_min,
                _          => {
                    println!("Not a valid fitness function");
                    process::exit(1);
                }
            };
            let mut_function = match args[12].to_uppercase().as_ref() {
                "DELTA_MUTATION" => mutation::delta_mutation,
                "GAUSSIAN_MUTATION" => mutation::gaussian_mutation,
                       _         => {
                    println!("Not a valid mutation function");
                    process::exit(1);
                }
            };

            let cross_function = match args[10].to_uppercase().as_ref() {
                "ONE_POINT_CROSSOVER" => crossover::one_point_crossover,
                "UNIFORM_CROSSOVER"   => crossover::uniform_crossover,
                "BLX_CROSSOVER"       => crossover::blx_crossover,
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
            ga::float_loop(fit_function, mut_function,cross_function,select_function);
        }
        "BIN"  =>   {
            let fit_function = match args[11].to_uppercase().as_ref() {
                "BINARY_ALTERNATE" => fitness::binary_alternate,
                "BIN_PARPS_FUNCTION" => fitness::bin_parps_function,
                "BIN_RADIO_FACTORY" => fitness::bin_radio_factory,
                "BIN_PATTERN_RECOGNITION" => fitness::bin_pattern_recognition,
                _          => {
                    println!("Not a valid fitness function");
                    process::exit(1);
                }
            };
            let mut_function = match args[12].to_uppercase().as_ref() {
                "BIT_FLIP" => mutation::bit_flip,
                _              => {
                    println!("Not a valid mutation function");
                    process::exit(1);
                }
            };

            let cross_function = match args[10].to_uppercase().as_ref() {
                "ONE_POINT_CROSSOVER" => crossover::one_point_crossover,
                "UNIFORM_CROSSOVER" => crossover::uniform_crossover,
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
            ga::bin_loop(fit_function, mut_function,cross_function,select_function);
        }
          _     => ga::testing(),
     };
}
