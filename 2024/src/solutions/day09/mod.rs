use crate::solver::Solver;

pub struct Problem;
impl Solver for Problem {
    type Ans1 = usize;
    type Ans2 = usize;

    fn solution1(&self, input: &str) -> Self::Ans1 {
        0
    }

    fn solution2(&self, input: &str) -> Self::Ans1 {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::day09::*;

    #[test]
    fn test_day_08_filesystem_checksum() {
    }

    const TEST_INPUT_1: &str = "2333133121414131402";
}

