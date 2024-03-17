use std::{env, fs};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let args: Vec<String> = env::args().collect();

    let content: String = fs::read_to_string(&args[1]).unwrap();
    let day = args[2].parse::<u32>().unwrap();

    let input_lines = content.lines().map(|l| l.as_ref()).collect();
    match day {
        1 => day1::run(input_lines),
        2 => day2::run(input_lines),
        3 => day3::run(input_lines),
        4 => day4::run(input_lines),
        5 => day5::run(input_lines),
        _ => todo!("Day {} not yet implemented", day),
    };
}
