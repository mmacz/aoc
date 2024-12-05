mod input;
use crate::solver::Solver;
pub struct Problem;

use std::collections::HashMap;

impl Solver for Problem {
    type Ans1 = usize;
    type Ans2 = usize;

    fn solution1(&self) -> Self::Ans1 {
        let v = split_page_ordering_and_updates(input::INPUT);
        let valid = get_valid_updates(&v);
        sum_middle_elements(&valid)
    }

    fn solution2(&self) -> Self::Ans2 {
        0
    }
}

fn get_valid_updates(input: &Vec<&str>) -> Vec<Vec<usize>> {
    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
    input[0].lines()
        .for_each(|line| {
            let parts: Vec<&str> = line.split("|").collect();
            let k: usize = parts[0].trim().parse().unwrap();
            let v: usize = parts[1].trim().parse().unwrap();
            rules.entry(k).or_insert_with(Vec::new).push(v);
        });

    input[1].lines()
        .filter_map(|line| {
            line.split(',')
                .map(|num| num.trim().parse::<usize>())
                .collect::<Result<Vec<_>, _>>()
                .ok()
        })
        .filter(|v| {
            v.windows(2).all(|window| {
                let first = window[0];
                let second = window[1];
                rules.get(&first)
                    .map_or(false, |values| values.contains(&second))
            })
        })
        .collect()
}

fn sum_middle_elements(updates: &Vec<Vec<usize>>) -> usize {
    updates.iter()
        .map(|u| {
            u[u.len() / 2]
        })
        .sum()
}

fn split_page_ordering_and_updates(input: &str) -> Vec<&str> {
    input.split("\n\n").collect()
}

#[cfg(test)]
mod test {
    use crate::solutions::day05::*;

    #[test]
    fn test_day_05_sum_ordering_rules() {
        let v = split_page_ordering_and_updates(TEST_INPUT_1);
        let valid = get_valid_updates(&v);
        assert_eq!(143, sum_middle_elements(&valid));
    }

const TEST_INPUT_1: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
}

