mod ga;
mod population;
mod fitness;
mod helpers;
mod crossover;
mod mutation;
mod selection;

use std::process;

fn main() {
    let args: Vec<String> = helpers::parse_arguments();

    match args[0].to_uppercase().as_ref() {
        "INT"   =>  {
            let fit_function = match args[11].to_uppercase().as_ref() {
                "INT_PARITY_ALTERNATE" => fitness::int_parity_alternate,
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
            ga::int_loop(fit_function,mut_function);
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
                       _         => {
                    println!("Not a valid mutation function");
                    process::exit(1);
                }
            };
            ga::float_loop(fit_function, mut_function);
        }
        "BIN"  =>   {
            let fit_function = match args[11].to_uppercase().as_ref() {
                "BINARY_ALTERNATE" => fitness::binary_alternate,
                "BIN_PARPS_FUNCTION" => fitness::bin_parps_function,
                "BIN_RADIO_FACTORY" => fitness::bin_radio_factory,
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
            ga::bin_loop(fit_function, mut_function);
        }
          _     => ga::testing(),
     };
}
