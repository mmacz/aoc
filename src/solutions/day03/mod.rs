mod input;
use crate::solver::Solver;
pub struct Problem;

use regex::Regex;

impl Solver for Problem {
    type Ans1 = i64;
    type Ans2 = i64;

    fn solution1(&self) -> Self::Ans1 {
        let v = find_mul_pairs(input::INPUT);
        sum_pairs_mult(&v)
    }

    fn solution2(&self) -> Self::Ans2 {
        0
    }
}

fn find_mul_pairs(memory: &str) -> Vec<(i64,i64)> {
    let re = Regex::new(r#"mul\((\d+,\d+)\)"#).unwrap();
    re.captures_iter(memory)
        .map(|c| {
            let (_, capture): (&str, [&str; 1]) = c.extract();
            let mut digits = capture[0].split(",");
            let first = digits.next().unwrap().parse::<i64>().unwrap();
            let second = digits.next().unwrap().parse::<i64>().unwrap();
            (first, second)
        })
        .collect()
}

fn sum_pairs_mult(pairs: &Vec<(i64, i64)>) -> i64 {
    pairs.iter()
        .map(|(a,b)| a * b)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::solutions::day03::*;

    #[test]
    fn test_day_03_mul() {
        let v = find_mul_pairs(TEST_INPUT_1);
        assert_eq!(161, sum_pairs_mult(&v));
    }

const TEST_INPUT_1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
}

