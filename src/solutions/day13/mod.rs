use crate::solver::Solver;
use crate::intcode::*;

mod input;

pub struct Problem;
impl Solver for Problem {
    type Ans1 = usize;
    type Ans2 = usize;

    fn solution1(&self) -> Self::Ans1 {
        0
    }

    fn solution2(&self) -> Self::Ans2 {
        0
    }
}

