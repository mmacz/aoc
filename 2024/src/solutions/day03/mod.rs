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
        let v = find_enabled_mul_pairs(input::INPUT);
        sum_pairs_mult(&v)
    }
}

fn find_mul_pairs(memory: &str) -> Vec<(i64, i64)> {
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

fn find_enabled_mul_pairs(memory: &str) -> Vec<(i64, i64)> {
    let re = Regex::new(r#"do\(\)|mul\((\d+,\d+)\)|don't\(\)"#).unwrap();
    let mut is_enabled = true;
    let mut pairs: Vec<(i64, i64)> = Vec::new();

    for capture in re.captures_iter(memory) {
        match capture.get(0) {
            Some(matched) => match matched.as_str() {
                "don't()" => {
                    is_enabled = false;
                }
                "do()" => {
                    is_enabled = true;
                }
                _ if matched.as_str().starts_with("mul(") && is_enabled => {
                    if let Some(digits_match) = capture.get(1) {
                        let digits: Vec<&str> = digits_match.as_str().split(",").collect();
                        let a = digits[0].parse::<i64>().unwrap();
                        let b = digits[1].parse::<i64>().unwrap();
                        pairs.push((a, b));
                    }
                }
                _ => {}
            },
            None => {}
        }
    }

    pairs
}

fn sum_pairs_mult(pairs: &Vec<(i64, i64)>) -> i64 {
    pairs.iter().map(|(a, b)| a * b).sum()
}

#[cfg(test)]
mod tests {
    use crate::solutions::day03::*;

    #[test]
    fn test_day_03_mul() {
        let v = find_enabled_mul_pairs(TEST_INPUT_1);
        assert_eq!(161, sum_pairs_mult(&v));
    }

    #[test]
    fn test_day_03_mul_enabled() {
        let v = find_enabled_mul_pairs(TEST_INPUT_2);
        assert_eq!(48, sum_pairs_mult(&v));
    }

    const TEST_INPUT_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_INPUT_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
}
