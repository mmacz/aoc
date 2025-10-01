use crate::solver::Solver;
use std::io::{self, Read};
use std::env;
use std::fs::{File, metadata};
use std::path::Path;
use regex::Regex;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

fn read_file_to_string(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn run_solutions() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file_name>", args[0]);
        std::process::exit(1);
    }
    let filepath = &args[1];
    if metadata(filepath).is_err() {
        eprintln!("Error: File '{}' does not exist.", filepath);
        std::process::exit(1);
    }
    let filename = Path::new(filepath).file_name().unwrap().to_str().unwrap();
    let mut day: usize = 1;
    let re = Regex::new(r"input(\d+)\.txt").unwrap();
    if let Some(captures) = re.captures(filename) {
        if let Some(x_str) = captures.get(1) {
            let x: usize = x_str.as_str().parse().unwrap();
            day = x;
        }
    }

    let input = read_file_to_string(filepath).unwrap();

    match day {
        1 => day01::Problem{}.solve(1, &input),
        2 => day02::Problem{}.solve(2, &input),
        3 => day03::Problem{}.solve(3, &input),
        4 => day04::Problem{}.solve(4, &input),
        5 => day05::Problem{}.solve(5, &input),
        6 => day06::Problem{}.solve(6, &input),
        7 => day07::Problem{}.solve(7, &input),
        8 => day08::Problem{}.solve(8, &input),
        9 => day09::Problem{}.solve(9, &input),
        _ => todo!()
    }

}
