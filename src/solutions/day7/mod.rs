use std::str::FromStr;
use itertools::Itertools;
mod input;
use crate::intcode::*;
use crate::solver::Solver;

fn sequential_thrusters(program: &str) -> i64 {
    [0, 1, 2, 3, 4]
        .into_iter()
        .permutations(5)
        .map(|phases| {
            phases.iter().fold(0, |prev, &i| {
                let mut amp = Cpu::new(program);
                amp.push_input(i);
                amp.push_input(prev);
                amp.run()
            })
        })
        .max()
        .unwrap()
}

fn feedback_loop_thrusters(program: &str) -> i64 {
    [5, 6, 7, 8, 9]
        .into_iter()
        .permutations(5)
        .map(|phases| {
            let mut amps = vec![Cpu::new(program); 5];
            for i in 0..5 {
                amps[i].push_input(phases[i]);
            }
            let mut input = 0;
            let mut last_output = 0;
            let mut running = true;
            while running {
                running = false;
                for i in 0..5 {
                    amps[i].push_input(input);
                    loop {
                        match amps[i].step() {
                            CpuStatus::Output(out) => {
                                input = out;
                                last_output = out;
                                running = true;
                                break;
                            },
                            CpuStatus::Finished => break,
                            CpuStatus::WaitForInput => {
                                running = true;
                                break;
                            },
                            CpuStatus::Running => continue,
                        }
                    }
                }
            }
            last_output
        })
        .max()
        .unwrap()
}

pub struct Problem;

impl Solver for Problem {
    type Ans1 = i64;
    type Ans2 = i64;

    fn solution1(&self) -> i64 {
        sequential_thrusters(input::INPUT)
    }
    
    fn solution2(&self) -> i64 {
        feedback_loop_thrusters(input::INPUT)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::day7::*;

    #[test]
    fn test_sequential_thrusters() {
        assert_eq!(43210, sequential_thrusters(input::SANITY1));
        assert_eq!(54321, sequential_thrusters(input::SANITY2));
        assert_eq!(65210, sequential_thrusters(input::SANITY3));
    }

    #[test]
    fn test_feedback_loop_thrusters() {
        assert_eq!(139629729, feedback_loop_thrusters(input::SANITY4));
        assert_eq!(18216, feedback_loop_thrusters(input::SANITY5));
    }
}

