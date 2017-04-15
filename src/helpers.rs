use std::fmt;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use std::cmp;

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

pub fn clamp<T> (value: T, lb: T, ub: T) -> T
    where T: Ord {
    cmp::min(ub,cmp::max(lb, value))
}

pub fn clampf (value: f64, lb: f64, ub: f64) -> f64 {
    ub.min(lb.max(value))
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

#[allow(dead_code)]
pub fn print_vector<T>(vec: &Vec<T>)
    where T: fmt::Display {
    for i in 0 .. vec.len() {
        print!("{} ", vec[i]);
    }
    println!("");
}

// http://stackoverflow.com/questions/27893223/how-do-i-iterate-over-a-range-with-a-custom-step
#[allow(dead_code)]
pub struct SimpleStepRange(pub usize, pub usize, pub usize);  // start, end, and step

impl Iterator for SimpleStepRange {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<usize> {
    if self.0 < self.1 {
        let v = self.0;
        self.0 = v + self.2;
        Some(v)
    } else {
        None
    }
    }
}
