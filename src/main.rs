use std::env;
use std::process;

mod population;

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


    let mut pop = population::Population::<i64>::new(pop_size_,ind_size_,l_bound_,u_bound_);

    pop.initialize();
    pop.print();

    // let mut population = match args[1].to_uppercase().as_ref() {
    //     "INT"   => Population::<i64>::new(pop_size_,ind_size_,l_bound_,u_bound_),
    //     "FLOAT" => Population::<f64>::new(pop_size_,ind_size_,l_bound_,u_bound_),
    //       _     => Population::<bool>::new(pop_size_,ind_size_,l_bound_,u_bound_),
    // };
}
