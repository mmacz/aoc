mod input;
use crate::solver::Solver;
use std::iter::successors;

pub struct Problem;

fn fuel_counter(mass: u64) -> u64 {
    (mass / 3).saturating_sub(2)
}

fn total_fuel(mass: u64) -> u64 {
    successors(Some(fuel_counter(mass)), |&m| Some(fuel_counter(m)))
        .take_while(|&m| m != 0)
        .sum()
}

impl Solver for Problem {
    type Ans1 = u64;
    type Ans2 = u64;

    fn solution1(&self) -> u64 {
        input::INPUT.iter().map(|mass| fuel_counter(*mass)).sum()
    }

    fn solution2(&self) -> u64 {
        input::INPUT.iter().map(|mass| total_fuel(*mass)).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::day01::*;

    #[test]
    fn test_fuel_counter() {
        assert_eq!(fuel_counter(12), 2);
        assert_eq!(fuel_counter(14), 2);
        assert_eq!(fuel_counter(1969), 654);
        assert_eq!(fuel_counter(100_756), 33583);
        assert_eq!(fuel_counter(1), 0);
    }

    #[test]
    fn test_total_fuel() {
        assert_eq!(total_fuel(14), 2);
        assert_eq!(total_fuel(1969), 966);
        assert_eq!(total_fuel(100_756), 50346);
    }
}
