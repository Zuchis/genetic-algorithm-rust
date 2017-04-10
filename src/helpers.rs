use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use std::process;

pub fn bin_to_int (bstr: &Vec<bool>) -> i64 {
    let mut converted: i64 = 0;
    let mut power = 0;
    for i in (0..bstr.len()).rev() {
        if bstr[i as usize]{
            converted += 2i64.pow(power);
        }
        power += 1;
    }
    converted
}

pub fn maxf (x:f64, y:f64) -> f64 {
    let max = if x >= y  {x} else {y};
    max
}

pub fn parse_arguments() -> Vec<String> {
    let file = File::open("ga.config").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let split = contents.split("\n");
    let mut args: Vec<String> = Vec::new();
    for s in split {
        let arg = s.split("=");
        args.push(arg.last().unwrap().to_string());
    }
    args
}
