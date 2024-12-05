mod input;
use crate::solver::Solver;
pub struct Problem;

use std::collections::HashMap;

enum UpdatesType { VALID, FIXED }

impl Solver for Problem {
    type Ans1 = usize;
    type Ans2 = usize;

    fn solution1(&self) -> Self::Ans1 {
        let v = split_page_ordering_and_updates(input::INPUT);
        let valid = get_updates_and_rules(&v, UpdatesType::VALID);
        sum_middle_elements(&valid)
    }

    fn solution2(&self) -> Self::Ans2 {
        let v = split_page_ordering_and_updates(input::INPUT);
        let fixed = get_updates_and_rules(&v, UpdatesType::FIXED);
        sum_middle_elements(&fixed)
    }
}

fn get_updates_and_rules(input: &Vec<&str>, update_type: UpdatesType) -> Vec<Vec<usize>> {
    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
    input[0].lines()
        .for_each(|line| {
            let parts: Vec<&str> = line.split("|").collect();
            let k: usize = parts[0].trim().parse().unwrap();
            let v: usize = parts[1].trim().parse().unwrap();
            rules.entry(k).or_insert_with(Vec::new).push(v);
        });

    let updates = input[1].lines()
        .filter_map(|line| {
            line.split(',')
                .map(|num| num.trim().parse::<usize>())
                .collect::<Result<Vec<_>, _>>()
                .ok()
        });
    let (valid, mut invalid): (Vec<Vec<usize>>, Vec<Vec<usize>>) = updates.into_iter()
        .partition(|v| {
            v.windows(2).all(|window| {
                let first = window[0];
                let second = window[1];
                rules.get(&first)
                    .map_or(false, |values| values.contains(&second))
            })
        });

    match update_type {
        UpdatesType::VALID => valid,
        UpdatesType::FIXED => {
            invalid.iter()
                .map(|update| {
                    let mut ordered = false;
                    let mut u = update.clone();
                    while !ordered {
                        ordered = true;
                        for i in 0..u.len() -1 {
                            let first = u[i];
                            let second = u[i + 1];
                            if !rules.get(&first).map_or(false, |values| values.contains(&second)) {
                                u.swap(i, i + 1);
                                ordered = false;
                                break;
                            }
                        }
                    }
                    u
                })
                .collect()
        }
    }
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
    fn test_day_05_sum_middles_of_valid_updates() {
        let v = split_page_ordering_and_updates(TEST_INPUT_1);
        let valid = get_updates_and_rules(&v, UpdatesType::VALID);
        assert_eq!(143, sum_middle_elements(&valid));
    }

    #[test]
    fn test_day_05_sum_middles_of_invalid_updates() {
        let v = split_page_ordering_and_updates(TEST_INPUT_1);
        let invalid = get_updates_and_rules(&v, UpdatesType::FIXED);
        assert_eq!(123, sum_middle_elements(&invalid));
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

