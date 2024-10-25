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
    use crate::solutions::day07::*;

    const SANITY1: &str = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
    const SANITY2: &str = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
    const SANITY3: &str = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
    const SANITY4: &str = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
    const SANITY5: &str = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";

    #[test]
    fn test_sequential_thrusters() {
        assert_eq!(43210, sequential_thrusters(SANITY1));
        assert_eq!(54321, sequential_thrusters(SANITY2));
        assert_eq!(65210, sequential_thrusters(SANITY3));
    }

    #[test]
    fn test_feedback_loop_thrusters() {
        assert_eq!(139629729, feedback_loop_thrusters(SANITY4));
        assert_eq!(18216, feedback_loop_thrusters(SANITY5));
    }
}

