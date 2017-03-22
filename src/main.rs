use std::env;
use std::process;

mod ga;
mod population;
mod fitness;
mod helpers;

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

    let p_size: u64 = args[2].trim().parse()
        .expect("Not a valid number");

    let i_size: u64 = args[3].trim().parse()
        .expect("Not a valid number");

    let lb: f64 = if args.len() == 6 {
        args[4].trim().parse()
            .expect("lul")
    } else { 0.0 as f64 };

    let ub: f64 = if args.len() == 6 {
        args[5].trim().parse()
            .expect("lul")
    } else { 0.0 as f64 };

    match args[1].to_uppercase().as_ref() {
        "INT"   => ga::int_loop(p_size,i_size,lb,ub),
        "FLOAT" => ga::float_loop(p_size,i_size,lb,ub),
        _       => ga::bin_loop(p_size,i_size),
     };
}
