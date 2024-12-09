mod input;
use crate::solver::Solver;
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

#[cfg(test)]
mod test {
    use crate::solutions::day07::*;

    #[test]
    fn test_day_07_sum_of_correct() {
    }

    const TEST_INPUT_1: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
}

