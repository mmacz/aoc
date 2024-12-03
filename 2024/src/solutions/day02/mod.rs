mod input;
use crate::solver::Solver;
pub struct Problem;

impl Solver for Problem {
    type Ans1 = usize;
    type Ans2 = usize;

    fn solution1(&self) -> Self::Ans1 {
        let vec = read_into_vec(input::INPUT);
        count_safe_reports(&vec)
    }

    fn solution2(&self) -> Self::Ans2 {
        let vec = read_into_vec(input::INPUT);
        count_dampened_reports(&vec)
    }
}

fn read_into_vec(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|l| l.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

fn is_report_safe(report: &Vec<i64>) -> bool {
    let inc = report.windows(2).all(|w| w[0] < w[1]);
    let dec = report.windows(2).all(|w| w[0] > w[1]);
    let level_diff = report.windows(2).all(|w| {
        let diff = (w[0] - w[1]).abs();
        diff >= 1 && diff <= 3
    });
    (inc || dec) && level_diff
}

fn count_safe_reports(reports: &Vec<Vec<i64>>) -> usize {
    reports.iter().filter(|r| is_report_safe(&r)).count()
}

fn count_dampened_reports(reports: &Vec<Vec<i64>>) -> usize {
    reports
        .iter()
        .filter(|r| {
            if is_report_safe(&r) {
                return true;
            }
            is_dampened_safe(&r)
        })
        .count()
}

fn is_dampened_safe(report: &Vec<i64>) -> bool {
    for index in 0..report.len() {
        let mut cloned = report.clone();
        cloned.remove(index);
        if is_report_safe(&cloned) {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use crate::solutions::day02::*;

    #[test]
    fn test_day_02_safe_reports_count() {
        let input = read_into_vec(TEST_INPUT_1);
        assert_eq!(2, count_safe_reports(&input));
    }

    #[test]
    fn test_day_02_problem_dampener() {
        let input = read_into_vec(TEST_INPUT_1);
        assert_eq!(4, count_dampened_reports(&input));
    }

    const TEST_INPUT_1: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
}
