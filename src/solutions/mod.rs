use std::time::{SystemTime, UNIX_EPOCH};
use crate::solver::Solver;
mod day01;

pub fn run_solutions() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    day01::Problem {}.solve(1);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!(" ******** AOC solutions time: {:#?} ******** ", end - start);
}
