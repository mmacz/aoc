mod input;
use crate::solver::Solver;
pub struct Problem;

use std::collections::HashSet;

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
    use crate::solutions::day06::*;

    #[test]
    fn test_day_06_unique_positions() {
    }

    const TEST_INPUT_1: &str = "....#.....
....^....#
..........
..#.......
.......#..
..........
.#........
........#.
#.........
......#...";
}

