mod ga;
mod population;
mod fitness;
mod helpers;

fn main() {
    let args: Vec<String> = helpers::parse_arguments();

    let p_size: u64 = args[1].trim().parse()
        .expect("Not a valid number");

    let i_size: u64 = args[2].trim().parse()
        .expect("Not a valid number");

    let lb: f64 = args[3].trim().parse()
            .expect("lul");

    let ub: f64 = args[4].trim().parse()
            .expect("lul");

    match args[0].to_uppercase().as_ref() {
        "INT"   => ga::int_loop(p_size,i_size,lb,ub),
        "FLOAT" => ga::float_loop(p_size,i_size,lb,ub),
        "BIN "  => ga::bin_loop(p_size,i_size),
          _     => ga::testing(),
     };
}
