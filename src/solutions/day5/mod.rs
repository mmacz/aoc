mod input;
use crate::intcode::*;
use crate::solver::Solver;
use std::str::FromStr;

fn get_program(program: &str) -> Vec<i64> {
    program
        .split(",")
        .filter_map(|c| i64::from_str(c).ok())
        .collect()
}

pub struct Problem;

impl Solver for Problem {
    type Ans1 = i64;
    type Ans2 = i64;

    fn solution1(&self) -> i64 {
        let mut cpu = Cpu::new(input::INPUT);
        cpu.push_input(1);
        cpu.run()
    }

    fn solution2(&self) -> i64 {
        let mut cpu = Cpu::new(input::INPUT);
        cpu.push_input(5);
        cpu.run()
    }
}

