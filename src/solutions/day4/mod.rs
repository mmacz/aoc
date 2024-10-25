use std::collections::HashMap;
mod input;
use crate::solver::Solver;

fn validate_password(passwd: i32, can_group: bool) -> bool {
    let mut prev: i32 = -1;
    let mut pass: i32 = passwd;
    let mut dupplicates: HashMap<i32, u32> = HashMap::new();
    while pass > 0 {
        let digit: i32 = pass % 10;
        pass = pass / 10;
        if prev == -1 {
            prev = digit;
            continue;
        }
        if prev < digit {
            return false;
        }
        if prev == digit {
            *dupplicates.entry(digit).or_insert(0) += 1;
        }
        prev = digit;
    }
    let groups: usize = dupplicates.keys().count();
    if groups == 0 {
        return false;
    }
    if !can_group {
        for k in dupplicates.keys() {
            if *dupplicates.get(k).unwrap() == 1 as u32 {
                return true;
            }
        }
        return false;
    }
    true
}

fn check_passwords(can_group: bool) -> i32 {
    let mut valid_count: i32 = 0;
    for passwd in 172930..=683082 {
        match validate_password(passwd, can_group) {
            true => valid_count += 1,
            false => (),
        }
    }
    valid_count
}

pub struct Problem;
impl Solver for Problem {
    type Ans1 = i32;
    type Ans2 = i32;

    fn solution1(&self) -> i32 {
        check_passwords(true)
    }

    fn solution2(&self) -> i32 {
        check_passwords(false)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::day4::*;

    #[test]
    fn test_password_grouping_enabled() {
        let _ =input::SANITY_INPUTS_1
            .iter()
            .map(|(pass, res)| {
                assert_eq!(validate_password(*pass, true), *res)
            })
            .collect::<Vec<_>>();
    }

    #[test]
    fn test_password_grouping_disabled() {
        let _ = input::SANITY_INPUTS_2
            .iter()
            .map(|(pass, res)| {
                assert_eq!(validate_password(*pass, false), *res)
            })
            .collect::<Vec<_>>();
    }
}

