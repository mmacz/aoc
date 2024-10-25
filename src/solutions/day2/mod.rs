mod input;
use crate::solver::Solver;
use crate::intcode::*;

pub struct Problem;


impl Solver for Problem {
    type Ans1 = i64;
    type Ans2 = i64;

    fn solution1(&self) -> i64 {
        let mut cpu: Cpu = Cpu::new(input::INPUT);
        cpu.code[1] = 12;
        cpu.code[2] = 2;
        cpu.run();
        cpu.code[0]
    }

    fn solution2(&self) -> i64 {
        let (mut n, mut v) = (0, 0);
        for noun in 0..=99 {
            for verb in 0..=99 {
                let mut cpu: Cpu = Cpu::new(input::INPUT);
                cpu.code[1] = noun;
                cpu.code[2] = verb;
                cpu.run();
                if cpu.code[0] == 19690720 {
                    (n, v) = (noun, verb);
                }
            }
        }
        100 * n + v
    }
}

