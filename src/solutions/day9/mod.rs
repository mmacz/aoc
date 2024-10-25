mod input;
use std::str::FromStr;
use crate::solver::Solver;
use crate::intcode::*;

fn sanity_check(program: &str, idx: usize) -> i32 {
    let mut cpu = Cpu::new(program);
    cpu.run();
    0
}

fn solution(program: &str, program_input: i64) -> i64 {
    let mut cpu = Cpu::new(program);
    cpu.push_input(program_input);
    cpu.run()
}

pub struct Problem;
impl Solver for Problem {
    type Ans1 = i64;
    type Ans2 = i64;

    fn solution1(&self) -> Self::Ans1 {
        solution(input::INPUT, 1)
    }

    fn solution2(&self) -> Self::Ans2 {
        solution(input::INPUT, 2)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::day9::*;

    #[test]
    fn test_verify_day9_sanity() {
        let results: Vec<i32> = input::SANITY_INPUTS
            .iter()
            .enumerate()
            .map(|(idx, input)| sanity_check(&input, idx))
            .collect();
        assert!(results.iter().all(|&res| res == 0));
    }
}

