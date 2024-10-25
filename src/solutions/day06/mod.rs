use std::collections::HashMap;
use crate::solver::Solver;
mod input;

fn count(map: &HashMap<&str, &str>, item: &str) -> usize {
    map.get(item).map(|item| count(map, item) + 1).unwrap_or(0)
}

fn get_orbit_map(content: &str) -> HashMap<&str, &str> {
    let map: HashMap<&str, &str> = content
        .lines()
        .map(|line| line.split(")").collect::<Vec<&str>>())
        .fold(HashMap::new(), |mut map, po| {
            map.insert(po[1], po[0]);
            map
        });
    map
}

fn orbit_count_checksum(content: &str) -> usize {
    let map: HashMap<&str, &str> = get_orbit_map(content);
    map.keys().map(|planet| count(&map, planet)).sum::<usize>()
}

fn distance(content: &str, from: &str, to: &str) -> usize {
    let map: HashMap<&str, &str> = get_orbit_map(content);
    let mut route = vec![from];
    while let Some(item) = map.get(route.last().unwrap()) {
        route.push(item);
    }

    let mut count = 0;
    let mut last = to;
    while let Some(item) = map.get(last) {
        if let Some(i) = route.iter().position(|i| item == i) {
            return count + i - 1;
        }
        last = item;
        count += 1;
    }
    0
}

pub struct Problem;

impl Solver for Problem {
    type Ans1 = usize;
    type Ans2 = usize;

    fn solution1(&self) -> usize {
        orbit_count_checksum(input::INPUT)
    }

    fn solution2(&self) -> usize {
        distance(input::INPUT, "YOU", "SAN")
    }
}


