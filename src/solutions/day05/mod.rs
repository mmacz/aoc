mod input;
use crate::intcode::*;
use crate::solver::Solver;

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

