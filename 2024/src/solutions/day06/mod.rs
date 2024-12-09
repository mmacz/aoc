mod input;
use crate::solver::Solver;
pub struct Problem;

use std::collections::HashSet;

impl Solver for Problem {
    type Ans1 = usize;
    type Ans2 = usize;

    fn solution1(&self) -> Self::Ans1 {
        let mut guard = get_guard(input::INPUT);
        guard.patrol();
        guard.patrol_route_len()
    }

    fn solution2(&self) -> Self::Ans2 {
        0
    }
}

#[derive(Clone, Copy)]
enum Direction {
    UP,
    RIGHT,
    LEFT,
    DOWN,
}
#[derive(Clone)]
enum Cell {
    EMPTY,
    OBSTACLE,
    GUARD,
}
impl From<char> for Cell {
    fn from(cell: char) -> Cell {
        match cell {
            '^' => Cell::GUARD,
            '.' => Cell::EMPTY,
            '#' => Cell::OBSTACLE,
            _ => panic!("Unknown cell type: {}", cell),
        }
    }
}
impl From<Direction> for (i64, i64) {
    fn from(dir: Direction) -> (i64, i64) {
        match dir {
            Direction::UP => (0, -1),
            Direction::RIGHT => (1, 0),
            Direction::DOWN => (0, 1),
            Direction::LEFT => (-1, 0),
        }
    }
}
type Map = Vec<Vec<Cell>>;
type PatrolRoute = HashSet<(usize, usize)>;

struct Guard {
    pos_x: usize,
    pos_y: usize,
    map: Map,
    route: PatrolRoute,
    dir: [Direction; 4],
    dir_idx: usize,
}

impl Guard {
    fn new(x: usize, y: usize, map: &Map) -> Guard {
        Guard {
            pos_x: x,
            pos_y: y,
            map: map.clone(),
            route: PatrolRoute::new(),
            dir: [
                Direction::UP,
                Direction::RIGHT,
                Direction::DOWN,
                Direction::LEFT,
            ],
            dir_idx: 0,
        }
    }

    fn patrol(&mut self) -> () {
        // initialize position
        self.map[self.pos_y as usize][self.pos_x as usize] = Cell::EMPTY;
        self.route.insert((self.pos_x, self.pos_y));

        let height = self.map.len() as i64;
        let width = self.map[0].len() as i64;

        loop {
            let (dx, dy) = self.dir[self.dir_idx].into();
            let (next_x, next_y) = (self.pos_x as i64 + dx, self.pos_y as i64 + dy);
            if next_x < 0 || next_x >= width || next_y < 0 || next_y >= height {
                return;
            }
            match self.map[next_y as usize][next_x as usize] {
                Cell::EMPTY => {
                    (self.pos_x, self.pos_y) = (next_x as usize, next_y as usize);
                    self.route.insert((next_x as usize, next_y as usize));
                }
                _ => {
                    self.dir_idx += 1;
                    self.dir_idx &= self.dir.len() - 1;
                }
            }
        }
    }

    fn patrol_route_len(&self) -> usize {
        self.route.len()
    }
}

fn get_guard(input: &str) -> Guard {
    let (mut x, mut y): (usize, usize) = (0, 0);
    let map: Vec<Vec<Cell>> = input
        .lines()
        .map(|line| line.chars().map(|cell| cell.into()).collect())
        .collect();
    for (yy, cells) in map.iter().enumerate() {
        for (xx, cell) in cells.iter().enumerate() {
            match cell {
                Cell::GUARD => {
                    x = xx;
                    y = yy;
                    break;
                }
                _ => {}
            }
        }
    }
    Guard::new(x, y, &map)
}

#[cfg(test)]
mod test {
    use crate::solutions::day06::*;

    #[test]
    fn test_day_06_unique_positions() {
        let mut guard = get_guard(TEST_INPUT_1);
        guard.patrol();
        assert_eq!(41, guard.patrol_route_len());
    }

    const TEST_INPUT_1: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
}
