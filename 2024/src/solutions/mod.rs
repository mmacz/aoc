use crate::solver::Solver;
use std::time::{SystemTime, UNIX_EPOCH};
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

pub fn run_solutions() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    day01::Problem {}.solve(1);
    day02::Problem {}.solve(2);
    day03::Problem {}.solve(3);
    day04::Problem {}.solve(4);
    day05::Problem {}.solve(5);
    day06::Problem {}.solve(6);
    day06::Problem {}.solve(7);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!(" ******** AOC solutions time: {:#?} ******** ", end - start);
}
