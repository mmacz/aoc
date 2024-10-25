mod input;
use crate::intcode::*;
use crate::solver::Solver;

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
    use crate::solutions::day09::*;

    const SANITY_INPUTS: [&str; 3] = [
        "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99",
        "1102,34915192,34915192,7,4,7,99,0",
        "104,1125899906842624,99",
    ];

    fn sanity_check(program: &str) -> i32 {
        let mut cpu = Cpu::new(program);
        cpu.run();
        0
    }

    #[test]
    fn test_verify_day9_sanity() {
        let results: Vec<i32> = SANITY_INPUTS
            .iter()
            .map(|input| sanity_check(&input))
            .collect();
        assert!(results.iter().all(|&res| res == 0));
    }
}
