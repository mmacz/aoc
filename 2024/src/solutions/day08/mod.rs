use std::collections::HashMap;
use std::collections::HashSet;

use crate::solver::Solver;
pub struct Problem;

type Grid = Vec<Vec<char>>;
type Coords = (i32, i32);
type Positions = Vec<Coords>;
type Antennas = HashMap<char, Positions>;
type Antinodes = HashSet<Coords>;

impl Solver for Problem {
    type Ans1 = usize;
    type Ans2 = usize;

    fn solution1(&self, input: &str) -> Self::Ans1 {
        let grid : Grid = read_into_grid(input);
        let antennas = get_antennas(&grid);
        let antinodes = get_antinodes(&antennas, &grid);
        antinodes.len()
    }

    fn solution2(&self, input: &str) -> Self::Ans1 {
        0
    }
}

fn read_into_grid(input: &str) -> Grid {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn get_antennas(grid: &Grid) -> Antennas {
    let mut antennas: Antennas = HashMap::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell.is_ascii_alphanumeric() {
                antennas.entry(cell).or_insert_with(Vec::new).push((x as i32, y as i32));
            }
        }
    }

    antennas
}


fn get_antinodes(antennas: &Antennas, grid: &Grid) -> Antinodes {
    let mut antinodes: Antinodes = HashSet::new();

    for positions in antennas.values() {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let (x1, y1) = positions[i];
                let (x2, y2) = positions[j];

                let dx = x2 - x1;
                let dy = y2 - y1;

                let antinode1 = (x1 - dx, y1 - dy);
                let antinode2 = (x2 + dx, y2 + dy);

                if antinode1.0 >= 0 && antinode1.0 < grid[0].len() as i32 &&
                   antinode1.1 >= 0 && antinode1.1 < grid.len() as i32 {
                    antinodes.insert(antinode1);
                }
                if antinode2.0 >= 0 && antinode2.0 < grid[0].len() as i32 &&
                   antinode2.1 >= 0 && antinode2.1 < grid.len() as i32 {
                    antinodes.insert(antinode2);
                }
            }
        }
    }

    antinodes
}

#[cfg(test)]
mod test {
    use crate::solutions::day08::*;

    #[test]
    fn test_day_08_antinode_locations_count() {
        let grid : Grid = read_into_grid(TEST_INPUT_1);
        let antennas = get_antennas(&grid);
        let antinodes = get_antinodes(&antennas, &grid);
        assert_eq!(antinodes.len(), 14);
    }

    const TEST_INPUT_1: &str =
"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
}

