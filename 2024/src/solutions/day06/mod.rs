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
        route_to_unique(&guard.route).len()
    }

    fn solution2(&self) -> Self::Ans2 {
        let mut guard = get_guard(input::INPUT);
        let blocking = guard.check_for_obstructions_blocks_routes();
        blocking.len()
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    UP,
    RIGHT,
    LEFT,
    DOWN,
}
#[derive(Clone, PartialEq, Eq)]
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
type Data = (usize, usize, Direction);
type Map = Vec<Vec<Cell>>;
type PatrolRoute = Vec<Data>;
#[derive(Clone)]
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

    fn patrol(&mut self) -> bool {
        let mut is_loop: bool = false;
        // initialize position
        self.map[self.pos_y as usize][self.pos_x as usize] = Cell::EMPTY;
        self.route
            .push((self.pos_x, self.pos_y, self.dir[self.dir_idx]));

        let height = self.map.len() as i64;
        let width = self.map[0].len() as i64;

        loop {
            let (dx, dy) = self.dir[self.dir_idx].into();
            let (next_x, next_y) = (self.pos_x as i64 + dx, self.pos_y as i64 + dy);
            if next_x < 0 || next_x >= width || next_y < 0 || next_y >= height {
                break;
            }
            match self.map[next_y as usize][next_x as usize] {
                Cell::EMPTY => {
                    (self.pos_x, self.pos_y) = (next_x as usize, next_y as usize);
                    let data: Data = (next_x as usize, next_y as usize, self.dir[self.dir_idx]);
                    if self.route.contains(&data) {
                        is_loop = true;
                        break;
                    }
                    self.route.push(data);
                }
                _ => {
                    self.dir_idx += 1;
                    self.dir_idx &= self.dir.len() - 1;
                }
            }
        }
        is_loop
    }

    fn check_for_obstructions_blocks_routes(&mut self) -> HashSet<(usize, usize)> {
        let mut blocking_positions: HashSet<(usize, usize)> = HashSet::new();
        let (start_x, start_y) = (self.pos_x, self.pos_y);

        let mut clean_map = self.map.clone();
        self.patrol();
        let visited = route_to_unique(&self.route);

        for (x, y) in visited.iter() {
            if clean_map[*y][*x] != Cell::EMPTY {
                continue;
            }
            clean_map[*y][*x] = Cell::OBSTACLE;
            let mut test_guard = Guard::new(start_x, start_y, &clean_map);
            if test_guard.patrol() {
                blocking_positions.insert((*x, *y));
            }
            clean_map[*y][*x] = Cell::EMPTY;
        }

        blocking_positions
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

fn route_to_unique(route: &PatrolRoute) -> HashSet<(usize, usize)> {
    route.iter().map(|(x, y, _)| (*x, *y)).collect()
}

#[cfg(test)]
mod test {
    use crate::solutions::day06::*;

    #[test]
    fn test_day_06_unique_positions() {
        let mut guard = get_guard(TEST_INPUT_1);
        guard.patrol();
        assert_eq!(41, route_to_unique(&guard.route).len());
    }

    #[test]
    fn test_day_06_blocking_positions_count() {
        let mut guard = get_guard(TEST_INPUT_1);
        let blocking = guard.check_for_obstructions_blocks_routes();
        assert_eq!(6, blocking.len());
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
