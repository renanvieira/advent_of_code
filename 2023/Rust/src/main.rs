use std::{env, fs};


mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let content = fs::read_to_string(&args[1]).unwrap();

    day1::run(content.lines().map(|l| l.as_ref()).collect());
}
