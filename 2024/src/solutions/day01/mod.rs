use crate::solver::Solver;
pub struct Problem;

impl Solver for Problem {
    type Ans1 = i64;
    type Ans2 = i64;

    fn solution1(&self, input: &str) -> Self::Ans1 {
        let vec = read_to_vec(input);
        total_distance(&vec)
    }

    fn solution2(&self, input: &str) -> Self::Ans2 {
        let vec = read_to_vec(input);
        total_similarity_score(&vec)
    }
}

fn read_to_vec(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let l: i64 = iter.next().unwrap().parse::<i64>().unwrap();
            let r: i64 = iter.next().unwrap().parse::<i64>().unwrap();
            (l, r)
        })
        .collect()
}

fn split_into_vectors(input: &Vec<(i64, i64)>) -> (Vec<i64>, Vec<i64>) {
    input.into_iter().map(|(l, r)| (l, r)).collect()
}

fn total_distance(input: &Vec<(i64, i64)>) -> i64 {
    let (mut left, mut right): (Vec<i64>, Vec<i64>) = split_into_vectors(input);
    left.sort();
    right.sort();

    left.iter().zip(right).map(|(l, r)| (l - r).abs()).sum()
}

fn total_similarity_score(input: &Vec<(i64, i64)>) -> i64 {
    let (left, right): (Vec<i64>, Vec<i64>) = split_into_vectors(input);

    let occurences: Vec<i64> = left
        .iter()
        .map(|l| right.iter().filter(|r| *r == l).count() as i64)
        .collect();

    left.iter().zip(occurences).map(|(l, o)| l * o).sum()
}

#[cfg(test)]
mod tests {
    use crate::solutions::day01::*;

    #[test]
    fn test_day_01_distance_in_list() {
        let locations = read_to_vec(TEST_INPUT_1);
        assert_eq!(11, total_distance(&locations));
    }

    #[test]
    fn test_day_01_similarity_score() {
        let locations = read_to_vec(TEST_INPUT_1);
        assert_eq!(31, total_similarity_score(&locations));
    }

    const TEST_INPUT_1: &str = "3   4
4   3
2   5
1   3
3   9
3   3";
}
