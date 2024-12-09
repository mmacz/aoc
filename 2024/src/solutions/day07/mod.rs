mod input;
use crate::solver::Solver;
pub struct Problem;

impl Solver for Problem {
    type Ans1 = u64;
    type Ans2 = u64;

    fn solution1(&self) -> Self::Ans1 {
        let with_concat = false;
        solve(input::INPUT, with_concat)
    }

    fn solution2(&self) -> Self::Ans2 {
        let with_concat = true;
        solve(input::INPUT, with_concat)
    }
}

enum Operation {
    ADD,
    MUL,
    CONCAT,
}

fn generate_operations(num_operations: usize) -> Vec<Vec<Operation>> {
    let mut operations: Vec<Vec<Operation>> = Vec::new();
    for i in 0..1 << num_operations {
        let mut ops: Vec<Operation> = Vec::new();
        for b in 0..num_operations {
            match (i >> b) & 1 {
                0 => ops.push(Operation::ADD),
                _ => ops.push(Operation::MUL),
            }
        }
        operations.push(ops);
    }
    operations
}

fn generate_operations_with_concat(num_operations: usize) -> Vec<Vec<Operation>> {
    let mut operations: Vec<Vec<Operation>> = Vec::new();
    let num_combinations = 3_usize.pow(num_operations as u32); // ADD, MUL, CONCAT

    for i in 0..num_combinations {
        let mut ops: Vec<Operation> = Vec::new();
        let mut current = i;

        for _ in 0..num_operations {
            match current % 3 {
                0 => ops.push(Operation::ADD),
                1 => ops.push(Operation::MUL),
                2 => ops.push(Operation::CONCAT),
                _ => unreachable!(),
            }
            current /= 3;
        }
        operations.push(ops);
    }
    operations
}

fn is_correct(target: &u64, operands: &Vec<u64>, with_concat: bool) -> bool {
    let operations = match with_concat {
        false => generate_operations(operands.len() - 1),
        true => generate_operations_with_concat(operands.len() - 1),
    };

    for ops in operations {
        if let Some(result) = apply_operations(&operands, &ops) {
            if result == *target {
                return true;
            }
        }
    }

    false
}

fn apply_operations(operands: &Vec<u64>, operations: &Vec<Operation>) -> Option<u64> {
    let mut result = operands[0];

    for (i, op) in operations.iter().enumerate() {
        match op {
            Operation::ADD => result += operands[i + 1],
            Operation::MUL => result *= operands[i + 1],
            Operation::CONCAT => {
                let concatenated = format!("{}{}", result, operands[i + 1])
                    .parse::<u64>()
                    .unwrap_or(0);
                result = concatenated;
            }
        }
    }

    Some(result)
}

fn solve(input: &str, with_concat: bool) -> u64 {
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
            match is_correct(&result, &operands, with_concat) {
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
        let with_concat = false;
        assert_eq!(3749, solve(TEST_INPUT_1, with_concat));
    }

    #[test]
    fn test_day_07_sum_of_correct_with_concat() {
        let with_concat = true;
        assert_eq!(11387, solve(TEST_INPUT_1, with_concat));
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
