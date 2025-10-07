use crate::solver::Solver;
use std::time::{SystemTime, UNIX_EPOCH};
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;

pub fn run_solutions() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    day01::Problem {}.solve(1);
    day02::Problem {}.solve(2);
    day03::Problem {}.solve(3);
    day04::Problem {}.solve(4);
    day05::Problem {}.solve(5);
    day06::Problem {}.solve(6);
    day07::Problem {}.solve(7);
    day08::Problem {}.solve(8);
    day09::Problem {}.solve(9);
    day10::Problem {}.solve(10);
    day11::Problem {}.solve(11);
    day12::Problem {}.solve(12);
    day13::Problem {}.solve(13);
    day14::Problem {}.solve(14);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!(" ******** AOC solutions time: {:#?} ******** ", end - start);
}
