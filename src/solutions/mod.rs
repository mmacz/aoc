use std::time::{SystemTime, UNIX_EPOCH};
use crate::solver::Solver;
mod day01;
mod day02;

pub fn run_solutions() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    day01::Problem {}.solve(1);
    day02::Problem {}.solve(2);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!(" ******** AOC solutions time: {:#?} ******** ", end - start);
}
