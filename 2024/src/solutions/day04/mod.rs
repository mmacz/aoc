mod input;
use crate::solver::Solver;

pub struct Problem;

impl Solver for Problem {
    type Ans1 = usize;
    type Ans2 = usize;

    fn solution1(&self) -> Self::Ans1 {
        let v: Vec<Vec<char>> = parse_input(input::INPUT);
        count_xmas_in_input(&v)
    }

    fn solution2(&self) -> Self::Ans2 {
        let v: Vec<Vec<char>> = parse_input(input::INPUT);
        count_cross_in_input(&v)
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn count_xmas_in_input(input: &Vec<Vec<char>>) -> usize {
    (0..input[0].len() as isize)
        .flat_map(|x| (0..input.len() as isize).map(move |y| (x, y)))
        .flat_map(|(x, y)| {
            [
                [(x, y), (x + 1, y - 1), (x + 2, y - 2), (x + 3, y - 3)],
                [(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
                [(x, y), (x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)],
                [(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
            ]
        })
        .filter(|coords| {
            let mut i = coords.iter().map(|(x, y)| {
                input
                    .get(*y as usize)
                    .and_then(|r| r.get(*x as usize).copied())
                    .unwrap_or_default()
            });
            let mut letters: [char; 4] = [' '; 4];
            letters.fill_with(|| i.next().unwrap_or_default());
            let text = String::from_iter(letters);
            match text.as_str() {
                "XMAS" | "SAMX" => true,
                _ => false,
            }
        })
        .count()
}

fn count_cross_in_input(input: &Vec<Vec<char>>) -> usize {
    (0..input[0].len() as isize)
        .flat_map(|x| (0..input.len() as isize).map(move |y| (x, y)))
        .map(|(x, y)| {
            [
                (x + 1, y + 1), // central letter
                (x, y),
                (x, y + 2),
                (x + 2, y),
                (x + 2, y + 2),
            ]
        })
        .filter(|coords| {
            let mut i = coords.iter().map(|(x, y)| {
                input
                    .get(*y as usize)
                    .and_then(|row| row.get(*x as usize).copied())
                    .unwrap_or_default()
            });

            if i.next().is_none_or(|n| n != 'A') {
                return false;
            }

            let mut letters: [char; 4] = [' '; 4];
            letters[0] = i.next().unwrap_or_default();
            letters[1] = i.next().unwrap_or_default();
            letters[2] = i.next().unwrap_or_default();
            letters[3] = i.next().unwrap_or_default();

            let text = String::from_iter(letters);
            match text.as_str() {
                "MMSS" | "MSMS" | "SSMM" | "SMSM" => true,
                _ => false,
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use crate::solutions::day04::*;

    #[test]
    fn test_day_04_count_xmas_in_input() {
        let v: Vec<Vec<char>> = parse_input(&TEST_INPUT_1);
        assert_eq!(18, count_xmas_in_input(&v));
    }

    #[test]
    fn test_day_04_count_cross_mas_in_input() {
        let v: Vec<Vec<char>> = parse_input(&TEST_INPUT_1);
        assert_eq!(9, count_cross_in_input(&v));
    }

    const TEST_INPUT_1: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
}
