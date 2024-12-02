mod input;
use crate::solver::Solver;
pub struct Problem;

impl Solver for Problem {
    type Ans1 = i64;
    type Ans2 = i64;

    fn solution1(&self) -> Self::Ans1 {
        let vec = read_to_vec(input::INPUT);
        total_distance(&vec)
    }

    fn solution2(&self) -> Self::Ans2 {
        0
    }
}

fn read_to_vec(input: &str) -> Vec<(i64,i64)> {
    input.lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let l: i64 = iter.next().unwrap().parse::<i64>().unwrap();
            let r: i64 = iter.next().unwrap().parse::<i64>().unwrap();
            (l, r)
        })
        .collect()
}

fn total_distance(input: &Vec<(i64, i64)>) -> i64 {
    let (mut left, mut right): (Vec<i64>, Vec<i64>) = input
        .into_iter()
        .map(|(l, r)| (l, r))
        .collect();
    left.sort();
    right.sort();

    left
        .iter()
        .zip(right)
        .map(|(l, r)| (l - r).abs())
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::solutions::day01::*;

    #[test]
    fn test_distance_in_list() {
        let locations = read_to_vec(TEST_INPUT_1);
        assert_eq!(11, total_distance(&locations));
    }

const TEST_INPUT_1: &str = "3   4
4   3
2   5
1   3
3   9
3   3";
}

