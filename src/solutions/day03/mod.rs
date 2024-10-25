use std::collections::HashMap;
use std::str::FromStr;

mod input;
use crate::solver::Solver;

struct Coords {
    dir: i64,
    steps: i64,
    is_vert: bool,
}

struct Line {
    coord: i64,
    tuple: (i64, i64)
}

#[derive(Clone)]
struct Point {
    x: i64,
    y: i64
}

impl Coords {
    pub fn new(entry: &str) -> Self {
        let d: char = entry.chars().nth(0).unwrap();
        let direction: i64 = if d == 'R' || d == 'U' { 1 } else { -1 };
        let steps: i64 = i64::from_str(&entry[1..]).unwrap();
        let is_vert: bool = d == 'U' || d == 'D';
        Coords {
            dir: direction,
            steps: steps,
            is_vert: is_vert,
        }
    }
}

impl Line {
    pub fn new(coord: i64, tuple: (i64, i64)) -> Self {
        let mut t1: i64 = tuple.0;
        let mut t2: i64 = tuple.1;
        if t1 > t2 {
            let tmp: i64 = t1;
            t1 = t2;
            t2 = tmp;
        }
        Line {
            coord: coord,
            tuple: (t1, t2),
        }
    }
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Point { x: x, y: y }
    }
}

fn get_wire_lines(wire: &str) -> Vec<Coords> {
    wire.split(",").map(|s: &str| Coords::new(s)).collect()
}

fn collect_lines(wire: &str) -> (Vec<Line>, Vec<Line>) {
    let mut horizontal_lines: Vec<Line> = Vec::new();
    let mut vertical_lines: Vec<Line> = Vec::new();
    let mut x: i64 = 0;
    let mut y: i64 = 0;

    let wires = get_wire_lines(wire);

    for w in wires {
        let shift = w.steps * w.dir;
        if w.is_vert {
            vertical_lines.push(Line::new(x, (y, y + shift)));
            y += shift;
        } else {
            horizontal_lines.push(Line::new(y, (x, x + shift)));
            x += shift;
        }
    }

    (horizontal_lines, vertical_lines)
}

fn is_in_range(lower: i64, upper: i64, value: i64) -> bool {
    return value > lower && value < upper;
}

fn get_intersections(horizontal: &Vec<Line>, vertical: &Vec<Line>) -> Vec<Point> {
    let mut intersections: Vec<Point> = Vec::new();
    for h in horizontal {
        for v in vertical {
            if is_in_range(h.tuple.0, h.tuple.1, v.coord)
                && is_in_range(v.tuple.0, v.tuple.1, h.coord)
            {
                intersections.push(Point::new(h.coord, v.coord));
            }
        }
    }
    intersections
}

fn manhattan_distance(pt: &Point) -> i64 {
    pt.x.abs() + pt.y.abs()
}

fn get_min_distance(intersections: &Vec<Point>) -> i64 {
    let mut min_distance = i64::MAX;
    for ip in intersections.into_iter() {
        let distance: i64 = manhattan_distance(&ip);
        if min_distance > distance {
            min_distance = distance;
        }
    }
    min_distance
}

fn is_intersection(x: i64, y: i64, intersections: &Vec<Point>) -> bool {
    for ip in intersections {
        /* Inverted entries in wrapper */
        if y == ip.x && x == ip.y {
            return true;
        }
    }
    false
}

fn build_key(x: i64, y: i64) -> u64 {
    (x as u64) << 32 | y as u64
}

fn get_min_steps(wires: &Vec<Vec<Coords>>, intersections: &Vec<Point>) -> i64 {
    let mut steps: HashMap<u64, u32> = HashMap::new();
    let mut key;
    for wire in wires {
        let mut x: i64 = 0;
        let mut y: i64 = 0;
        let mut step: u32 = 0;
        for c in wire {
            for _s in 0..c.steps {
                if c.is_vert {
                    y += c.dir as i64;
                } else {
                    x += c.dir as i64;
                }
                step += 1;
                if is_intersection(x, y, &intersections) {
                    key = build_key(x, y);
                    if steps.contains_key(&key) {
                        let new_step = steps.get(&key).unwrap() + step;
                        steps.insert(key, new_step);
                    } else {
                        steps.insert(key, step);
                    }
                }
            }
        }
    }

    let mut min_steps: u64 = u64::MAX;
    for s in steps {
        if min_steps > s.1 as u64 {
            min_steps = s.1 as u64;
        }
    }
    min_steps as i64
}

pub struct Problem;
impl Solver for Problem {
    type Ans1 = i64;
    type Ans2 = i64;

    fn solution1(&self) -> i64 {
        let lines1 = collect_lines(input::WIRE1);
        let lines2 = collect_lines(input::WIRE2);

        let mut intersections = get_intersections(&lines1.0, &lines2.1);
        intersections.extend(get_intersections(&lines2.0, &lines1.1));
        get_min_distance(&intersections)
    }

    fn solution2(&self) -> i64 {
        let lines1 = collect_lines(input::WIRE1);
        let lines2 = collect_lines(input::WIRE2);

        let mut intersections = get_intersections(&lines1.0, &lines2.1);
        intersections.extend(get_intersections(&lines2.0, &lines1.1));

        let wires: Vec<Vec<Coords>> = vec![
            get_wire_lines(input::WIRE1), get_wire_lines(input::WIRE2)
        ];
        get_min_steps(&wires, &intersections)
    }
}

