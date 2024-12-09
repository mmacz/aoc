mod input;
use crate::solver::Solver;
pub struct Problem;

impl Solver for Problem {
    type Ans1 = u64;
    type Ans2 = u64;

    fn solution1(&self) -> Self::Ans1 {
        solve_part_one(input::INPUT)
    }

    fn solution2(&self) -> Self::Ans2 {
        0
    }
}

enum Operation {
    ADD,
    MUL,
}

fn generate_operations(num_operations: usize) -> Vec<Vec<Operation>> {
    let mut operations: Vec<Vec<Operation>> = Vec::new();

    for i in 0..1 << num_operations {
        let mut ops: Vec<Operation> = Vec::new();
        for b in 0..num_operations {
            match (i >> b) & 1 {
                1 => ops.push(Operation::ADD),
                _ => ops.push(Operation::MUL),
            }
        }
        operations.push(ops);
    }
    operations
}

fn is_correct(target: &u64, operands: &Vec<u64>) -> bool {
    let operations = generate_operations(operands.len() - 1);

    for ops in operations {
        let mut result: u64 = operands[0];
        for (i, op) in ops.iter().enumerate() {
            match op {
                Operation::ADD => result += operands[i + 1],
                Operation::MUL => result *= operands[i + 1],
            }
        }
        if result == *target {
            return true;
        }
    }

    false
}

fn solve_part_one(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|line| {
            let split: Vec<_> = line.split(':').collect();
            let result = split[0].parse::<u64>().unwrap_or(0);
            let operands: Vec<_> = split[1]
                .trim()
                .split(' ')
                .map(|op| op.parse::<u64>().unwrap_or(0))
                .collect();
            match is_correct(&result, &operands) {
                true => Some(result),
                false => None,
            }
        })
        .collect::<Vec<_>>()
        .iter()
        .sum()
}

#[cfg(test)]
mod test {
    use crate::solutions::day07::*;

    #[test]
    fn test_day_07_sum_of_correct() {
        assert_eq!(3749, solve_part_one(TEST_INPUT_1));
    }

    const TEST_INPUT_1: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
}
