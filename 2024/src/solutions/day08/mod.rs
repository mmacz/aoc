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
mod test {
    use crate::solutions::day08::*;

    #[test]
    fn test_day_08_antinode_locations_count() {
        assert!(false);
    }

    const TEST_INPUT_1: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
}

